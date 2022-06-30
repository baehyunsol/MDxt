pub mod line;
mod predicate;
mod node;

use crate::render::render_option::RenderOption;
use crate::link::{predicate::read_link_reference, normalize_link};
use line::Line;
use node::{Node, parse_header};
use std::collections::HashMap;

#[derive(PartialEq)]
enum NodeType {  // this enum is only used internally by `AST::from_lines`
    Paragraph,
    CodeFence {
        language: String,
        line_num: bool
    },
    None
}

impl NodeType {
    pub fn is_code_fence(&self) -> bool {
        match self {
            NodeType::CodeFence{..} => true,
            _ => false
        }
    }
}

pub struct ASTConfig {
    head_anchor: bool
}

pub struct AST {
    config: ASTConfig,
    headers: Vec<(usize, Vec<u16>)>,  // (level, content)
    link_refs: HashMap<Vec<u16>, Vec<u16>>,  // (link_label, link_destination)
    nodes: Vec<Node>
}

impl AST {

    pub fn from_lines(lines: Vec<Line>, options: &mut RenderOption) -> AST {
        let mut curr_ast = Vec::with_capacity(lines.len());
        let mut curr_node = vec![];
        let mut curr_node_type = NodeType::None;
        let mut link_refs = HashMap::new();
        let mut headers = vec![];

        for line in lines.iter() {

            if curr_node_type.is_code_fence() {

                if line.is_code_fence() {
                    add_curr_node_to_ast(&mut curr_ast, &mut curr_node, &mut curr_node_type);
                }

                else {
                    curr_node.push(line.clone());
                }

                continue;
            }

            if line.is_header() {
                add_curr_node_to_ast(&mut curr_ast, &mut curr_node, &mut curr_node_type);
                let (level, content) = parse_header(line);
                headers.push((level, content.clone()));
                curr_ast.push(Node::new_header(level, content));
            }

            else if line.is_empty() {
                add_curr_node_to_ast(&mut curr_ast, &mut curr_node, &mut curr_node_type);
            }

            else if line.is_code_fence() {
                add_curr_node_to_ast(&mut curr_ast, &mut curr_node, &mut curr_node_type);
                let (language, line_num) = read_code_fence_info(line);
                curr_node_type = NodeType::CodeFence { language, line_num };
            }

            else if line.is_link_reference_definition() {
                let (link_label, link_destination) = read_link_reference(&line.content);
                link_refs.insert(normalize_link(&link_label), (options.link_handler)(&link_destination));
            }

            else {
                curr_node.push(line.clone());

                if curr_node_type == NodeType::None {
                    curr_node_type = NodeType::Paragraph;
                }

            }

        }

        todo!()
    }

}

fn add_curr_node_to_ast(curr_ast: &mut Vec<Node>, curr_node: &mut Vec<Line>, curr_node_type: &mut NodeType) {

    if curr_node.len() == 0 {
        return;
    }

    match curr_node_type {
        NodeType::Paragraph => {
            curr_ast.push(Node::new_paragraph(curr_node));
            *curr_node = vec![];
            *curr_node_type = NodeType::None;
        }
        NodeType::CodeFence { language, line_num } => {
            curr_ast.push(Node::new_code_fence(curr_node, language.clone(), *line_num));
            *curr_node = vec![];
            *curr_node_type = NodeType::None;
        },
        NodeType::None => {
            panic!("something went wrong!");
        }
    }

}

fn read_code_fence_info(line: &Line) -> (String, bool) {
    todo!()
}