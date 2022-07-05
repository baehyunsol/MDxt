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

#[derive(PartialEq)]
pub enum ParseState {  // this enum is only used internally by `AST::from_lines`
    Paragraph,
    CodeFence {
        language: String,
        line_num: bool,
        code_fence_size: usize,
        is_tilde_fence: bool
    },
    Table,
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
                ParseState::CodeFence { language, line_num, code_fence_size, is_tilde_fence } => {

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

                        if index + 1 < lines.len() && lines[index + 1].is_table_delimiter() &&
                        count_cells(&lines[index].content, false) == count_delimiter_cells(&lines[index + 1].content) {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                            curr_lines.push(lines[index].clone());
                            curr_parse_state = ParseState::Table;
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
                ParseState::Table => {

                    if lines[index].is_table_row() {
                        curr_lines.push(lines[index].clone());
                    }

                    else {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        continue;
                    }

                },
                ParseState::None => {

                    if lines[index].is_code_fence_begin() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_parse_state = read_code_fence_info(&lines[index]);
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

                        if index + 1 < lines.len() && lines[index + 1].is_table_delimiter() &&
                        count_cells(&lines[index].content, false) == count_delimiter_cells(&lines[index + 1].content) {
                            add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                            curr_lines.push(lines[index].clone());
                            curr_parse_state = ParseState::Table;
                        }

                        // paragraph
                        else {
                            curr_lines.push(lines[index].clone());
                            curr_parse_state = ParseState::Paragraph;
                        }

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

    if curr_lines.len() == 0 {
        return;
    }

    match curr_parse_state {
        ParseState::Paragraph => {
            curr_nodes.push(Node::new_paragraph(curr_lines));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::Table => {
            curr_nodes.push(Node::new_table(curr_lines));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::CodeFence { language, line_num, .. } => {
            curr_nodes.push(Node::new_code_fence(curr_lines, language.clone(), *line_num));
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
