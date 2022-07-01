use crate::utils::*;

use super::line::{add_br_if_needed, to_raw};
use crate::ast::line::Line;
use crate::inline::InlineNode;

pub enum Node {
    Paragraph {
        content: InlineNode
    },
    Header {
        level: usize,
        content: InlineNode
    },
    FencedCode {
        language: String,
        line_num: bool,
        content: Vec<u16>
    },
    Empty
}

impl Node {

    pub fn new_header(level: usize, content: Vec<u16>) -> Node {
        Node::Header { level, content: InlineNode::Raw(content) }
    }

    pub fn new_paragraph(lines: &Vec<Line>) -> Node {
        Node::Paragraph {
            content: InlineNode::Raw(lines.iter().map(add_br_if_needed).collect::<Vec<Vec<u16>>>().join(&[' ' as u16][..]))
        }
    }

    pub fn new_code_fence(lines: &Vec<Line>, language: String, line_num: bool) -> Node {
        Node::FencedCode {
            content: lines.iter().map(to_raw).collect::<Vec<Vec<u16>>>().join(&['\n' as u16][..]),
            language, line_num
        }
    }

}

pub fn parse_header(line: &Line) -> (usize, Vec<u16>) {  // (level, content)
    let (sharps, sharps_removed) = take_and_drop_while(&line.content, '#' as u16);
    let indents_removed = drop_while(&sharps_removed, ' ' as u16);

    (sharps.len(), indents_removed)
}