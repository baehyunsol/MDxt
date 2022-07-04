use super::line::{add_br_if_needed, to_raw};
use crate::ast::line::Line;
use crate::inline::InlineNode;
use crate::table::Table;

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
    Table(Table),
    ThematicBreak,
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

    pub fn new_table(lines: &Vec<Line>) -> Node {
        Node::Table (Table::from_lines(lines))
    }

}