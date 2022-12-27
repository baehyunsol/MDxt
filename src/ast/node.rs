use super::line::add_br_if_needed;
use crate::ast::line::Line;
use crate::inline::InlineNode;
use crate::inline::macros::multiline::MultiLineMacro;
use crate::container::{
    blockquote::Blockquote,
    codefence::FencedCode,
    header::normalize_header,
    list::List,
    table::Table,
};

#[cfg(test)]
use crate::testbench::debugger::*;
use crate::utils::from_v16;

#[derive(Clone)]
pub enum Node {
    Paragraph {
        content: InlineNode
    },
    Header {
        level: usize,
        content: InlineNode,
        anchor: Vec<u16>
    },
    FencedCode(FencedCode),
    Table(Table),
    List(List),
    Blockquote(Blockquote),
    ThematicBreak,
    MultiLineMacro(MultiLineMacro),
    Empty
}

impl Node {

    pub fn new_header(level: usize, content: Vec<u16>) -> Node {

        #[cfg(test)]
        push_call_stack("Node::new_header", &from_v16(&content));

        let result = Node::Header {
            level,
            anchor: normalize_header(&content),
            content: InlineNode::Raw(content)
        };

        #[cfg(test)]
        pop_call_stack();

        result
    }

    pub fn new_paragraph(lines: &Vec<Line>) -> Node {

        #[cfg(test)]
        push_call_stack("Node::new_paragraph", "");

        let result = Node::Paragraph {
            content: InlineNode::Raw(
                lines
                    .iter()
                    .map(|line| add_br_if_needed(&line.content))
                    .collect::<Vec<Vec<u16>>>()
                    .join(&[' ' as u16][..])
            )
        };

        #[cfg(test)]
        pop_call_stack();

        result
    }

    pub fn new_code_fence(
        lines: &Vec<Line>,
        language: &[u16],
        line_num: &Option<usize>,
        highlights: &Vec<usize>,
        copy_button: bool,
        index: usize
    ) -> Node {

        #[cfg(test)]
        push_call_stack("Node::new_paragraph", "");

        let result = Node::FencedCode(FencedCode::new(
            lines.iter().map(|line| line.to_raw()).collect::<Vec<Vec<u16>>>().join(&['\n' as u16][..]),
            language.to_vec(),
            line_num.clone(),
            highlights.clone(),
            copy_button,
            index
        ));

        #[cfg(test)]
        pop_call_stack();

        result
    }

    pub fn new_table(headers: &Vec<Line>, lines: &Vec<Line>, alignments: &Line, index: usize) -> Node {
        Node::Table(Table::from_lines(headers, lines, alignments, index))
    }

    pub fn new_list(lines: &Vec<Line>) -> Node {
        Node::List(List::from_lines(lines))
    }

    pub fn new_blockquote(lines: &Vec<Line>) -> Node {
        Node::Blockquote(Blockquote::from_lines(lines))
    }

    pub fn new_macro(line: &Line) -> Node {
        Node::MultiLineMacro(MultiLineMacro::from_line(line))
    }

}