use super::line::{add_br_if_needed, to_raw};
use crate::ast::line::Line;
use crate::inline::InlineNode;
use crate::container::table::Table;
use crate::container::codefence::FencedCode;
use crate::container::blockquote::Blockquote;
use crate::container::list::List;

pub enum Node {
    Paragraph {
        content: InlineNode
    },
    Header {
        level: usize,
        content: InlineNode
    },
    FencedCode(FencedCode),
    Table(Table),
    List(List),
    Blockquote(Blockquote),
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

    pub fn new_code_fence(lines: &Vec<Line>, language: &[u16], line_num: &Option<usize>, highlights: &Vec<usize>) -> Node {

        Node::FencedCode(FencedCode::new(
            lines.iter().map(to_raw).collect::<Vec<Vec<u16>>>().join(&['\n' as u16][..]),
            language.to_vec(),
            line_num.clone(),
            highlights.clone()
        ))
    }

    pub fn new_table(headers: &Vec<Line>, lines: &Vec<Line>, alignments: &Line) -> Node {
        Node::Table(Table::from_lines(headers, lines, alignments))
    }

    pub fn new_list(lines: &Vec<Line>) -> Node {
        Node::List(List::from_lines(lines))
    }

    pub fn new_blockquote(lines: &Vec<Line>) -> Node {
        Node::Blockquote(Blockquote::from_lines(lines))
    }

}