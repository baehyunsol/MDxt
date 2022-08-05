use super::{AST, doc_data::DocData, line::Line, node::Node};
use crate::inline::{
    footnote::{Footnote, predicate::is_valid_footnote_label},
    InlineNode,
    link::{normalize_link_label, predicate::read_link_reference},
    macros::{get_macro_name, parse_arguments, predicate::read_macro, MACROS}
};
use crate::container::{
    codefence::read_code_fence_info,
    header::parse_header,
    table::{count_cells, count_delimiter_cells},
};
use crate::render::render_option::RenderOption;
use crate::utils::{from_v16, into_v16};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum ParseState {  // this enum is only used internally by `AST::from_lines`
    Paragraph,
    CodeFence {
        language: Vec<u16>,
        line_num: Option<usize>,
        highlights: Vec<usize>,
        code_fence_size: usize,
        copy_button: bool,
        is_tilde_fence: bool,
        index: usize  // index is used when making `copy to clipboard` buttons
    },
    Table {
        header_lines: Vec<Line>,
        alignments: Line,
        index: usize  // index is used when making collapsible tables
    },
    Blockquote,
    List,
    None
}

impl AST {

    pub fn from_lines(lines: Vec<Line>, options: &RenderOption) -> AST {
        let mut curr_nodes = Vec::with_capacity(lines.len());
        let mut curr_lines = vec![];
        let mut curr_parse_state = ParseState::None;
        let mut link_references = HashMap::new();
        let mut footnote_references = HashMap::new();
        let mut headers = vec![];
        let mut table_count = 0;
        let mut fenced_code_count = 0;

        let mut macro_closing_indexes = vec![];

        let mut index = 0;

        'outer_loop: while index < lines.len() {

            match &curr_parse_state {
                ParseState::CodeFence { code_fence_size, is_tilde_fence, .. } => {

                    if lines[index].is_code_fence_end() {
                        let (end_code_fence_size, is_tilde_end_fence) = match read_code_fence_info(&lines[index], fenced_code_count) {
                            ParseState::CodeFence { code_fence_size, is_tilde_fence, .. } => (code_fence_size, is_tilde_fence),
                            _ => panic!("unreachable code")
                        };

                        if end_code_fence_size >= *code_fence_size && is_tilde_end_fence == *is_tilde_fence {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                            curr_parse_state = ParseState::None;
                            fenced_code_count += 1;
                            index += 1;
                            continue;
                        }

                    }

                    curr_lines.push(lines[index].clone());
                },
                ParseState::Paragraph => {

                    if macro_closing_indexes.contains(&index) {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_nodes.push(Node::new_macro(&lines[index]));
                        macro_closing_indexes = macro_closing_indexes.into_iter().filter(|i| *i != index).collect();
                        index += 1;
                        continue;
                    }

                    else if lines[index].is_code_fence_begin() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_parse_state = read_code_fence_info(&lines[index], fenced_code_count);
                    }

                    else if lines[index].is_header() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

                        let (level, content) = parse_header(&lines[index]);
                        headers.push((level, content.clone()));
                        curr_nodes.push(Node::new_header(level, content));
                        curr_parse_state = ParseState::None;
                    }

                    else if lines[index].is_empty() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
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
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

                            let header_lines = header_lines.into_iter().map(|line| (*line).clone()).collect::<Vec<Line>>();
                            let alignments = lines[header_end_index].clone();

                            curr_parse_state = ParseState::Table { header_lines, alignments, index: table_count };
                            index = header_end_index;
                        }

                        // paragraph
                        else {
                            curr_lines.push(lines[index].clone());
                        }

                    }

                    else if lines[index].is_blockquote() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_parse_state = ParseState::Blockquote;
                        curr_lines.push(lines[index].clone());
                    }

                    // a single line of an ordered list is not rendered to `<ol>`
                    // a single line of an unordered list is fine
                    else if lines[index].is_unordered_list()
                        || lines[index].is_ordered_list()
                        && index + 1 < lines.len()
                        && lines[index + 1].is_ordered_list()
                    {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_parse_state = ParseState::List;
                        curr_lines.push(lines[index].clone());
                    }

                    // paragraph
                    else {
                        curr_lines.push(lines[index].clone());
                    }

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

                },
                ParseState::None => {

                    if macro_closing_indexes.contains(&index) {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_nodes.push(Node::new_macro(&lines[index]));
                        macro_closing_indexes = macro_closing_indexes.into_iter().filter(|i| *i != index).collect();
                        index += 1;
                        continue;
                    }

                    else if lines[index].is_code_fence_begin() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_parse_state = read_code_fence_info(&lines[index], fenced_code_count);
                    }

                    else if lines[index].is_header() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

                        let (level, content) = parse_header(&lines[index]);
                        headers.push((level, content.clone()));
                        curr_nodes.push(Node::new_header(level, content));
                        curr_parse_state = ParseState::None;
                    }

                    else if lines[index].is_link_or_footnote_reference_definition() {
                        let (link_label, link_destination) = read_link_reference(&lines[index].content);

                        if is_valid_footnote_label(&link_label) {
                            let footnote_label = normalize_link_label(&link_label);
                            let footnote_index = if footnote_references.contains_key(&footnote_label) {
                                footnote_references.len() - 1
                            }

                            else {
                                footnote_references.len()
                            };

                            footnote_references.insert(
                                footnote_label,
                                Footnote {
                                    index: footnote_index,
                                    inverse_index: vec![],
                                    content: InlineNode::Raw(link_destination)
                                }
                            );
                        }

                        else {
                            link_references.insert(normalize_link_label(&link_label), into_v16(&options.handle_link(&from_v16(&link_destination))));
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

                                            if inner_macro_stack.len() == 0 {
                                                curr_nodes.push(Node::new_macro(&lines[index]));
                                                macro_closing_indexes.push(macro_closing_index);
                                                index += 1;
                                                continue 'outer_loop;
                                            }

                                            else {
                                                curr_closing_macro = inner_macro_stack[inner_macro_stack.len() - 1].get_closing_macro();
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
                                                if curr_macro[0] == '/' as u16 {
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

                    else if lines[index].is_thematic_break() {
                        curr_nodes.push(Node::ThematicBreak);
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

                    // a single line of an ordered list is not rendered to `<ol>`
                    // a single line of an unordered list is fine
                    else if lines[index].is_unordered_list()
                        || lines[index].is_ordered_list()
                        && index + 1 < lines.len()
                        && lines[index + 1].is_ordered_list()
                    {
                        curr_parse_state = ParseState::List;
                        curr_lines.push(lines[index].clone());
                    }

                    else if lines[index].is_blockquote() {
                        curr_parse_state = ParseState::Blockquote;
                        curr_lines.push(lines[index].clone());
                    }

                    else if lines[index].is_empty() {
                        curr_nodes.push(Node::Empty);
                    }

                    // paragraph
                    else {
                        curr_lines.push(lines[index].clone());
                        curr_parse_state = ParseState::Paragraph;
                    }

                }
            }

            index += 1;
        }

        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

        AST {
            nodes: curr_nodes,
            doc_data: DocData::new(headers, link_references, footnote_references),
            toc: vec![],  // if needed, will be rendered later
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
        ParseState::CodeFence { language, line_num, highlights, copy_button, index, .. } => {
            curr_nodes.push(Node::new_code_fence(curr_lines, &language, &line_num, &highlights, *copy_button, *index));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::None => {

            if curr_lines.len() != 0 {
                panic!("What should I do?");
            }

        }
    }

}
