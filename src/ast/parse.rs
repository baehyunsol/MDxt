use super::{MdData, AST, line::Line, node::Node};
use crate::inline::InlineNode;
use crate::link::{predicate::read_link_reference, normalize_link};
use crate::footnote::{predicate::is_valid_footnote_label, Footnote};
use crate::utils::{drop_while, take_and_drop_while};
use crate::render::render_option::RenderOption;
use std::collections::HashMap;

#[derive(PartialEq)]
enum ParseState {  // this enum is only used internally by `AST::from_lines`
    Paragraph,
    CodeFence {
        language: String,
        line_num: bool
    },
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

        for line in lines.iter() {

            match &curr_parse_state {
                ParseState::CodeFence { language, line_num } => {

                    if line.is_code_fence_end() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_parse_state = ParseState::None;
                    }
    
                    else {
                        curr_lines.push(line.clone());
                    }

                },
                ParseState::Paragraph => {

                    if line.is_code_fence_begin() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

                        let (language, line_num) = read_code_fence_info(line);
                        curr_parse_state = ParseState::CodeFence { language, line_num };
                    }

                    else if line.is_header() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

                        let (level, content) = parse_header(line);
                        headers.push((level, content.clone()));
                        curr_nodes.push(Node::new_header(level, content));
                        curr_parse_state = ParseState::None;
                    }

                    else if line.is_empty() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);
                        curr_nodes.push(Node::Empty);
                        curr_parse_state = ParseState::None;
                    }

                    // paragraph
                    else {
                        curr_lines.push(line.clone());
                    }

                },
                ParseState::None => {

                    if line.is_code_fence_begin() {
                        add_curr_node_to_ast(&mut curr_nodes, &mut curr_lines, &mut curr_parse_state);

                        let (language, line_num) = read_code_fence_info(line);
                        curr_parse_state = ParseState::CodeFence { language, line_num };
                    }

                    else if line.is_link_or_footnote_reference_definition() {
                        let (link_label, link_destination) = read_link_reference(&line.content);

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

                    else if line.is_thematic_break() {
                        curr_nodes.push(Node::ThematicBreak);
                    }

                    else if line.is_empty() {
                        curr_nodes.push(Node::Empty);
                    }

                    // paragraph
                    else {
                        curr_lines.push(line.clone());
                        curr_parse_state = ParseState::Paragraph;
                    }

                }
            }

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

fn read_code_fence_info(line: &Line) -> (String, bool) {
    todo!()
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
        }
        ParseState::CodeFence { language, line_num } => {
            curr_nodes.push(Node::new_code_fence(curr_lines, language.clone(), *line_num));
            *curr_lines = vec![];
            *curr_parse_state = ParseState::None;
        },
        ParseState::None => {
            panic!("something went wrong!");
        }
    }

}
