use crate::utils::*;

use super::line::{add_br_if_needed, to_raw};
use crate::ast::line::Line;

pub enum Node {
    Paragraph {
        content: Vec<u16>
    },
    Header {
        level: usize,
        content: Vec<u16>
    },
    FencedCode {
        language: String,
        line_num: bool,
        content: Vec<u16>
    },
    Empty
}

impl Node {

    pub fn new_header(line: &Line) -> Node {

        let (sharps, sharps_removed) = take_and_drop_while(&line.content, '#' as u16);
        let indents_removed = drop_while(&sharps_removed, ' ' as u16);

        Node::Header {
            level: sharps.len(),
            content: indents_removed
        }
    }

    pub fn new_paragraph(lines: &Vec<Line>) -> Node {
        Node::Paragraph {
            content: lines.iter().map(add_br_if_needed).collect::<Vec<Vec<u16>>>().join(&[' ' as u16][..])
        }
    }

    pub fn new_code_fence(lines: &Vec<Line>, language: String, line_num: bool) -> Node {
        Node::FencedCode {
            content: lines.iter().map(to_raw).collect::<Vec<Vec<u16>>>().join(&['\n' as u16][..]),
            language, line_num
        }
    }

}