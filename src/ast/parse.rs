use super::{AST, doc_data::DocData, line::Line, node::Node};
use crate::inline::{
    footnote::{Footnote, predicate::is_valid_footnote_label},
    InlineNode,
    link::{normalize_link_label, predicate::read_link_reference},
    macros::{get_macro_name, parse_arguments, predicate::read_macro, MACROS, multiline::{MultiLineMacro, MultiLineMacroType}},
};
use crate::container::{
    codefence::read_code_fence_info,
    header::parse_header,
    table::{count_cells, count_delimiter_cells},
};
use crate::render::render_option::RenderOption;
use crate::utils::{from_v32, into_v32};
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub enum ParseState {  // this enum is only used internally by `AST::from_lines`
    Paragraph,
    CodeFence {
        language: Vec<u32>,
        line_num: Option<usize>,
        highlights: Vec<usize>,
        code_fence_size: usize,
        copy_button: bool,
        is_tilde_fence: bool,
        id: Option<Vec<u32>>,
        classes: Vec<Vec<u32>>,
        index: usize,  // index is used when making `copy to clipboard` buttons
    },
    IndentedCodeBlock,
    Table {
        header_lines: Vec<Line>,
        alignments: Line,
        index: usize,  // index is used when making collapsible tables
    },
    Math {  // multiline [[math]] macro
        end_index: usize,
    },
    Blockquote,
    List,
    None,
}

impl AST {

    pub fn from_lines(lines: Vec<Line>, options: &RenderOption) -> AST {
        let mut curr_nodes = Vec::with_capacity(lines.len());
        let mut curr_lines = vec![];
        let mut curr_parse_state = ParseState::None;
        let mut table_count = 0;
        let mut fenced_code_count = 0;
        let mut sidebar = vec![];
        let mut doc_data = DocData::default();

        let mut has_multiline_macro = false;

        let mut macro_closing_indexes = HashSet::new();

        let mut index = 0;

        'outer_loop: while index < lines.len() {
            match &curr_parse_state {
                ParseState::CodeFence { code_fence_size, is_tilde_fence, .. } => {
                    if lines[index].is_code_fence_end() {
                        let (end_code_fence_size, is_tilde_end_fence) = match read_code_fence_info(&lines[index], fenced_code_count) {
                            ParseState::CodeFence { code_fence_size, is_tilde_fence, .. } => (code_fence_size, is_tilde_fence),
                            _ => unreachable!()
                        };

                        if end_code_fence_size >= *code_fence_size && is_tilde_end_fence == *is_tilde_fence {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                            fenced_code_count += 1;
                            index += 1;
                            continue;
                        }
                    }

                    curr_lines.push(lines[index].clone());
                },
                ParseState::IndentedCodeBlock => {
                    if lines[index].is_empty() {
                        curr_lines.push(lines[index].try_sub_indent(4));
                    }

                    else if lines[index].indent < 4 {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        continue;
                    }

                    else {
                        curr_lines.push(lines[index].try_sub_indent(4));
                    }
                }
                ParseState::Paragraph | ParseState::None => {
                    if macro_closing_indexes.contains(&index) {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_nodes.push(Node::new_macro(&lines[index], &mut doc_data));
                        macro_closing_indexes.remove(&index);
                        index += 1;
                        continue;
                    }

                    // an indented code block cannot interrupt a paragraph
                    else if lines[index].indent >= 4 && curr_parse_state == ParseState::None {
                        if !curr_lines.is_empty() {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        }

                        curr_parse_state = ParseState::IndentedCodeBlock;
                        curr_lines.push(lines[index].try_sub_indent(4));
                    }

                    else if lines[index].is_code_fence_begin() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_parse_state = read_code_fence_info(&lines[index], fenced_code_count);
                    }

                    else if lines[index].is_header() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

                        let (level, content) = parse_header(&lines[index]);
                        doc_data.headers.push((level, content.clone()));
                        curr_nodes.push(Node::new_header(level, content));
                    }

                    else if curr_parse_state == ParseState::None && lines[index].is_thematic_break() {
                        curr_nodes.push(Node::ThematicBreak);
                    }

                    else if lines[index].is_empty() {
                        if !curr_lines.is_empty() {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        }

                        curr_nodes.push(Node::Empty);
                        curr_parse_state = ParseState::None;
                    }

                    else if lines[index].is_table_row() {
                        let mut header_end_index = index + 1;
                        let mut header_lines = vec![&lines[index]];

                        while header_end_index < lines.len()
                            && lines[header_end_index].is_table_row()
                            && !lines[header_end_index].is_table_delimiter()
                        {
                            header_lines.push(&lines[header_end_index]);
                            header_end_index += 1;
                        }

                        if header_end_index < lines.len()
                            && lines[header_end_index].is_table_delimiter()
                            && count_cells(&lines[index].content, false) == count_delimiter_cells(&lines[header_end_index].content)
                        {

                            if !curr_lines.is_empty() {
                                add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                            }

                            let header_lines = header_lines.into_iter().map(|line| (*line).clone()).collect::<Vec<Line>>();
                            let alignments = lines[header_end_index].clone();

                            curr_parse_state = ParseState::Table { header_lines, alignments, index: table_count };
                            index = header_end_index;
                        }

                        // paragraph
                        else {
                            curr_lines.push(lines[index].clone());
                            curr_parse_state = ParseState::Paragraph;
                        }
                    }

                    else if lines[index].is_blockquote() {
                        if !curr_lines.is_empty() {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        }

                        curr_parse_state = ParseState::Blockquote;
                        curr_lines.push(lines[index].clone());
                    }

                    // a single line of an ordered list is not rendered to `<ol>`
                    // a single line of an unordered list is fine
                    else if (lines[index].is_unordered_list()
                        || lines[index].is_ordered_list()
                        && index + 1 < lines.len()
                        && (
                            lines[index + 1].is_ordered_list()
                            || lines[index + 1].is_unordered_list()

                        // the indentation of the first element must be less than 4
                        // otherwise, it's an indented code block
                        )) && lines[index].indent < 4
                    {
                        if !curr_lines.is_empty() {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        }

                        curr_parse_state = ParseState::List;
                        curr_lines.push(lines[index].clone());
                    }

                    // it cannot interrupt a paragraph
                    else if curr_parse_state == ParseState::None && lines[index].is_link_or_footnote_reference_definition() {
                        let (link_label, link_destination) = read_link_reference(&lines[index].content);

                        if is_valid_footnote_label(&link_label) {
                            let footnote_label = normalize_link_label(&link_label);

                            let footnote_index = match doc_data.footnote_references.get(&footnote_label) {
                                Some(f) => f.index,
                                None => doc_data.footnote_references.len()
                            };

                            doc_data.footnote_references.insert(
                                footnote_label,
                                Footnote {
                                    index: footnote_index,
                                    inverse_index: vec![],
                                    content: InlineNode::Raw(link_destination)
                                }
                            );
                        }

                        else {
                            doc_data.link_references.insert(normalize_link_label(&link_label), into_v32(&options.handle_link(&from_v32(&link_destination))));
                        }
                    }

                    // All the closing macros are handled up there
                    else if lines[index].is_multiline_macro() {
                        let macro_content = read_macro(&lines[index].content, 0).unwrap();
                        let macro_arguments = parse_arguments(&macro_content);
                        let macro_name = get_macro_name(&macro_arguments);

                        match MACROS.get(&macro_name) {
                            // if it has a closing, find its partner
                            Some(macro_) if macro_.has_closing && macro_.is_valid(&macro_arguments) => {
                                let mut macro_closing_index = index + 1;

                                let mut inner_macro_stack = vec![macro_.clone()];
                                let mut curr_closing_macro = macro_.get_closing_macro();

                                while macro_closing_index < lines.len() {
                                    if lines[macro_closing_index].is_multiline_macro() {
                                        let curr_macro = read_macro(&lines[macro_closing_index].content, 0).unwrap();

                                        if curr_macro == curr_closing_macro {
                                            inner_macro_stack.pop().unwrap();

                                            if inner_macro_stack.is_empty() {
                                                add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

                                                // into_v32("math") -> [109, 97, 116, 104]
                                                if macro_name == &[109, 97, 116, 104] {
                                                    curr_parse_state = ParseState::Math { end_index: macro_closing_index };
                                                }

                                                else {
                                                    macro_closing_indexes.insert(macro_closing_index);
                                                    curr_nodes.push(Node::new_macro(&lines[index], &mut doc_data));
                                                    curr_parse_state = ParseState::Paragraph;
                                                }

                                                index += 1;
                                                has_multiline_macro = true;
                                                continue 'outer_loop;
                                            }

                                            else {
                                                curr_closing_macro = inner_macro_stack.last().unwrap().get_closing_macro();
                                                macro_closing_index += 1;
                                                continue;
                                            }
                                        }

                                        let curr_macro_arguments = parse_arguments(&curr_macro);
                                        let curr_macro_name = get_macro_name(&curr_macro_arguments);

                                        match MACROS.get(&curr_macro_name) {
                                            Some(inner_macro)
                                                if inner_macro.has_closing
                                                    && inner_macro.is_valid(&curr_macro_arguments) =>
                                            {
                                                inner_macro_stack.push(inner_macro.clone());
                                                curr_closing_macro = inner_macro.get_closing_macro();
                                            },
                                            _ => {
                                                if curr_macro[0] == '/' as u32 {
                                                    let possibly_another_macro = curr_macro[1..].to_vec();

                                                    // it assumes that all the closing macros have the same form: `'/' + macro_name`
                                                    match MACROS.get(&possibly_another_macro) {
                                                        // it's always valid because I checked `MACROS.get(macro)`, not `MACROS.get(macro_name)`
                                                        // if a valid macro is found here, that means the very first macro is not properly closed
                                                        Some(another_macro) if another_macro.has_closing => {
                                                            break;
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    macro_closing_index += 1;
                                }
                            },

                            // otherwise it's just a paragraph
                            _ => {}
                        }

                        curr_lines.push(lines[index].clone());
                        curr_parse_state = ParseState::Paragraph;
                    }

                    // paragraph
                    else {
                        curr_lines.push(lines[index].clone());
                        curr_parse_state = ParseState::Paragraph;
                    }
                },
                ParseState::Math { end_index } => if index == *end_index {
                    add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                }

                else {
                    curr_lines.push(lines[index].clone());
                },
                ParseState::Table { .. } => {
                    if lines[index].is_table_row() {
                        curr_lines.push(lines[index].clone());
                    }

                    else {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        table_count += 1;
                        continue;
                    }
                },
                ParseState::List => {
                    if lines[index].is_empty()
                        || lines[index].is_code_fence_begin()
                        || lines[index].is_header() || lines[index].is_thematic_break()
                        || lines[index].is_table_row() || lines[index].is_blockquote()
                        || lines[index].is_multiline_macro()
                    {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        continue;
                    }

                    else {
                        curr_lines.push(lines[index].clone());
                    }
                },
                ParseState::Blockquote => {
                    if lines[index].is_empty() || lines[index].is_code_fence_begin()
                        || lines[index].is_header() || lines[index].is_thematic_break()
                        || lines[index].is_table_row() || lines[index].is_ordered_list()
                        || lines[index].is_unordered_list()
                    {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        continue;
                    }

                    else {
                        curr_lines.push(lines[index].clone());
                    }
                }
            }

            index += 1;
        }

        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

        // some multiline macros (tooltip, sidebar, collapsible) have to know their inner content
        if has_multiline_macro {
            collect_nodes_for_multiline_macros(&mut curr_nodes, &mut sidebar, options, &mut doc_data);
        }

        AST {
            nodes: curr_nodes,
            doc_data,
            toc: vec![],  // if needed, will be rendered later
            sidebar,
            render_option: options.clone(),
            is_inline_parsed: false
        }
    }
}

fn add_curr_node_to_ast(curr_nodes: &mut Vec<Node>, curr_lines: &mut Vec<Line>, curr_parse_state: &mut ParseState) {
    match curr_parse_state {
        ParseState::Paragraph => {
            curr_nodes.push(Node::new_paragraph(curr_lines));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::Table { header_lines, alignments, index } => {
            curr_nodes.push(Node::new_table(header_lines, curr_lines, alignments, *index));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::List => {
            curr_nodes.push(Node::new_list(curr_lines));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::Blockquote => {
            curr_nodes.push(Node::new_blockquote(curr_lines));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::CodeFence { language, line_num, highlights, copy_button, id, classes, index, .. } => {
            curr_nodes.push(Node::new_code_fence(curr_lines, &language, &line_num, &highlights, *copy_button, &id, &classes, *index));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::IndentedCodeBlock => {

            // removes trailing empty lines
            while let Some(line) = curr_lines.last() {

                if line.is_empty() { curr_lines.pop().unwrap(); }

                else { break; }

            }

            let mut preceding_empty_lines = 0;

            while let Some(line) = curr_lines.get(preceding_empty_lines) {

                if line.is_empty() { preceding_empty_lines += 1; }

                else { break; }

            }

            *curr_lines = curr_lines[preceding_empty_lines..].to_vec();

            // empty code blocks are ignored
            if !curr_lines.is_empty() {
                // a code fence without any decoration
                curr_nodes.push(Node::new_code_fence(curr_lines, &vec![], &None, &vec![], false, &None, &vec![], usize::MAX));
                *curr_lines = vec![];
            }

            *curr_parse_state = ParseState::None;
        }
        ParseState::None => if !curr_lines.is_empty() {
            panic!("What should I do?");
        },
        ParseState::Math { .. } => {
            curr_nodes.push(Node::new_math_ml(curr_lines));

            // since the above line only generates an opening macro, it adds a closing one
            curr_nodes.push(Node::MultiLineMacro(
                MultiLineMacro {
                    macro_type: MultiLineMacroType::Math(vec![91, 91, 47, 109, 97, 116, 104, 93, 93]),  // into_v32("[[/math]]")
                    is_closing: true,
                }
            ));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        }
    }
}

/*
```
[[collapsible]]

foo

bar

[[/collapsible]]
```

in the above mdxt, `foo` and `bar` has to be children of `[[collapsible]]`, not the main AST
*/
fn collect_nodes_for_multiline_macros(nodes: &mut Vec<Node>, sidebar: &mut Vec<Node>, render_option: &RenderOption, doc_data: &mut DocData) {
    let mut index = 0;
    let mut stack_of_nodes = vec![];

    while index < nodes.len() {
        if let Node::MultiLineMacro(MultiLineMacro { macro_type, is_closing, .. }) = &nodes[index] {
            if macro_type.has_inner_nodes() {
                if *is_closing {

                    // contents of a sidebar is stored in AST, not in the MacroType
                    // `MultiLineMacroType::Sidebar` is ignored by Node::to_html
                    if macro_type.is_sidebar() {
                        *sidebar = stack_of_nodes.pop().unwrap();
                    }

                    else {
                        // `nodes[index - 1]` must be the opening multiline-macro 
                        nodes[index - 1].set_inner_nodes(stack_of_nodes.pop().unwrap());
                    }
                }

                else {
                    stack_of_nodes.push(vec![]);
                    index += 1;
                    continue;
                }
            }

        }

        if !stack_of_nodes.is_empty() {
            let last_index = stack_of_nodes.len() - 1;

            // TODO: this is O(n^2)
            // I can think of O(n) alternatives, but the code would get uglier...
            // the number of inner nodes must be very small, so why don't we just keep this solution?
            let mut node = nodes.remove(index);
            node.parse_inlines(render_option, doc_data);

            stack_of_nodes[last_index].push(node);
        }

        else {
            index += 1;
        }
    }
}
