use super::{MdData, AST, line::Line, node::Node};
use crate::inline::{
    InlineNode,
    link::{predicate::read_link_reference, normalize_link},
    footnote::{predicate::is_valid_footnote_label, Footnote}
};
use crate::table::{count_cells, count_delimiter_cells};
use crate::utils::{drop_while, take_and_drop_while};
use crate::render::render_option::RenderOption;
use crate::codefence::read_code_fence_info;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum ParseState {  // this enum is only used internally by `AST::from_lines`
    Paragraph,
    CodeFence {
        language: Vec<u16>,
        line_num: Option<usize>,
        highlights: Vec<usize>,
        code_fence_size: usize,
        is_tilde_fence: bool
    },
    Table {
        header_lines: Vec<Line>,
        alignments: Line
    },
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

        let mut index = 0;

        while index < lines.len() {

            match &curr_parse_state {
                ParseState::CodeFence { code_fence_size, is_tilde_fence, .. } => {

                    if lines[index].is_code_fence_end() {
                        let (end_code_fence_size, is_tilde_end_fence) = match read_code_fence_info(&lines[index]) {
                            ParseState::CodeFence { code_fence_size, is_tilde_fence, .. } => (code_fence_size, is_tilde_fence),
                            _ => panic!("unreachable code")
                        };

                        if end_code_fence_size >= *code_fence_size && is_tilde_end_fence == *is_tilde_fence {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                            curr_parse_state = ParseState::None;
                            index += 1;
                            continue;
                        }

                    }

                    curr_lines.push(lines[index].clone());
                },
                ParseState::Paragraph => {

                    if lines[index].is_code_fence_begin() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_parse_state = read_code_fence_info(&lines[index]);
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

                        while header_end_index < lines.len() && lines[header_end_index].is_table_row() && !lines[header_end_index].is_table_delimiter() {
                            header_lines.push(&lines[header_end_index]);
                            header_end_index += 1;
                        }

                        if header_end_index < lines.len() && lines[header_end_index].is_table_delimiter() &&
                        count_cells(&lines[index].content, false) == count_delimiter_cells(&lines[header_end_index].content) {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

                            let header_lines = header_lines.into_iter().map(|line| (*line).clone()).collect::<Vec<Line>>();
                            let alignments = lines[header_end_index].clone();

                            curr_parse_state = ParseState::Table { header_lines, alignments };
                            index = header_end_index;
                        }

                        // paragraph
                        else {
                            curr_lines.push(lines[index].clone());
                        }

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
                        continue;
                    }

                },
                ParseState::List => {

                    if lines[index].is_empty() || lines[index].is_code_fence_begin() ||
                    lines[index].is_header() || lines[index].is_thematic_break() ||
                    lines[index].is_table_row() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        continue;
                    }

                    else {
                        curr_lines.push(lines[index].clone());
                    }

                },
                ParseState::None => {

                    if lines[index].is_code_fence_begin() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_parse_state = read_code_fence_info(&lines[index]);
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
                            let footnote_label = normalize_link(&link_label);
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
                            link_references.insert(normalize_link(&link_label), (options.link_handler)(&link_destination));
                        }

                    }

                    else if lines[index].is_thematic_break() {
                        curr_nodes.push(Node::ThematicBreak);
                    }

                    else if lines[index].is_table_row() {
                        let mut header_end_index = index + 1;
                        let mut header_lines = vec![&lines[index]];

                        while header_end_index < lines.len() && lines[header_end_index].is_table_row() && !lines[header_end_index].is_table_delimiter() {
                            header_lines.push(&lines[header_end_index]);
                            header_end_index += 1;
                        }

                        if header_end_index < lines.len() && lines[header_end_index].is_table_delimiter() &&
                        count_cells(&lines[index].content, false) == count_delimiter_cells(&lines[header_end_index].content) {

                            let header_lines = header_lines.into_iter().map(|line| (*line).clone()).collect::<Vec<Line>>();
                            let alignments = lines[header_end_index].clone();

                            curr_parse_state = ParseState::Table { header_lines, alignments };
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
                    else if lines[index].is_unordered_list() ||
                    lines[index].is_ordered_list() && index + 1 < lines.len() && lines[index + 1].is_ordered_list() {
                        curr_parse_state = ParseState::List;
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
            md_data: MdData::new(headers, link_references, footnote_references),
            render_option: options.clone(),
            is_inline_parsed: false
        }
    }

}

pub fn parse_header(line: &Line) -> (usize, Vec<u16>) {  // (level, content)
    let (sharps, sharps_removed) = take_and_drop_while(&line.content, '#' as u16);
    let indents_removed = drop_while(&sharps_removed, ' ' as u16);

    (sharps.len(), indents_removed)
}

fn add_curr_node_to_ast(curr_nodes: &mut Vec<Node>, curr_lines: &mut Vec<Line>, curr_parse_state: &mut ParseState) {

    match curr_parse_state {
        ParseState::Paragraph => {
            curr_nodes.push(Node::new_paragraph(curr_lines));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::Table { header_lines, alignments } => {
            curr_nodes.push(Node::new_table(header_lines, curr_lines, alignments));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::List { .. } => todo!(),
        ParseState::CodeFence { language, line_num, highlights, .. } => {
            curr_nodes.push(Node::new_code_fence(curr_lines, &language, &line_num, &highlights));
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
