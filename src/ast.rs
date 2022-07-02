pub mod line;
mod predicate;
mod node;

use crate::inline::InlineNode;
use crate::render::render_option::RenderOption;
use crate::link::{predicate::read_link_reference, normalize_link};
use crate::footnote::predicate::is_valid_footnote_label;
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

pub struct AST {
    head_anchor: bool,
    headers: Vec<(usize, Vec<u16>)>,  // (level, content)
    link_references: HashMap<Vec<u16>, Vec<u16>>,  // (label, destination)
    footnote_references: HashMap<Vec<u16>, (usize, InlineNode)>,  // (label, (index, content))
    nodes: Vec<Node>
}

impl AST {

    pub fn from_lines(lines: Vec<Line>, options: &RenderOption) -> AST {
        let mut curr_ast = Vec::with_capacity(lines.len());
        let mut curr_nodes = vec![];
        let mut curr_node_type = NodeType::None;
        let mut link_references = HashMap::new();
        let mut footnote_references = HashMap::new();
        let mut headers = vec![];

        for line in lines.iter() {

            if curr_node_type.is_code_fence() {

                if line.is_code_fence() {
                    add_curr_node_to_ast(&mut curr_ast, &mut curr_nodes, &mut curr_node_type);
                }

                else {
                    curr_nodes.push(line.clone());
                }

                continue;
            }

            if line.is_header() {
                add_curr_node_to_ast(&mut curr_ast, &mut curr_nodes, &mut curr_node_type);

                let (level, content) = parse_header(line);
                headers.push((level, content.clone()));
                curr_ast.push(Node::new_header(level, content));
            }

            else if line.is_empty() {
                add_curr_node_to_ast(&mut curr_ast, &mut curr_nodes, &mut curr_node_type);
            }

            else if line.is_code_fence() {
                add_curr_node_to_ast(&mut curr_ast, &mut curr_nodes, &mut curr_node_type);

                let (language, line_num) = read_code_fence_info(line);
                curr_node_type = NodeType::CodeFence { language, line_num };
            }

            else if line.is_link_or_footnote_reference_definition() {
                let (link_label, link_destination) = read_link_reference(&line.content);

                if is_valid_footnote_label(&link_label) {
                    footnote_references.insert(normalize_link(&link_label), (footnote_references.len(), InlineNode::Raw(link_destination)));
                }

                else {
                    link_references.insert(normalize_link(&link_label), (options.link_handler)(&link_destination));
                }

            }

            else {
                curr_nodes.push(line.clone());

                if curr_node_type == NodeType::None {
                    curr_node_type = NodeType::Paragraph;
                }

            }

        }

        todo!()
    }

    pub fn parse_inlines(&mut self, render_option: &RenderOption) {
        self.nodes.iter_mut().for_each(
            |node| match node {
                Node::Paragraph { content } => {content.parse_raw(&self.link_references, &self.footnote_references, render_option);},
                Node::Header { content, .. } => {content.parse_raw(&self.link_references, &self.footnote_references, render_option);},
                Node::Empty | Node::FencedCode {..} => {}
            }
        )
    }

}

fn add_curr_node_to_ast(curr_ast: &mut Vec<Node>, curr_nodes: &mut Vec<Line>, curr_node_type: &mut NodeType) {

    if curr_nodes.len() == 0 {
        return;
    }

    match curr_node_type {
        NodeType::Paragraph => {
            curr_ast.push(Node::new_paragraph(curr_nodes));
            *curr_nodes = vec![];
            *curr_node_type = NodeType::None;
        }
        NodeType::CodeFence { language, line_num } => {
            curr_ast.push(Node::new_code_fence(curr_nodes, language.clone(), *line_num));
            *curr_nodes = vec![];
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
