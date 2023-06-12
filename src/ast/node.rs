use super::line::add_br_if_needed;
use crate::ast::line::Line;
use crate::inline::InlineNode;
use crate::inline::macros::multiline::{MultiLineMacro, MultiLineMacroType};
use crate::container::{
    blockquote::Blockquote,
    codefence::FencedCode,
    header::normalize_header,
    list::List,
    table::Table,
};

#[derive(Clone)]
pub enum Node {
    Paragraph {
        content: InlineNode
    },
    Header {
        level: usize,
        content: InlineNode,
        anchor: Vec<u32>
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

    pub fn new_header(level: usize, content: Vec<u32>) -> Node {
        Node::Header {
            level,
            anchor: normalize_header(&content),
            content: InlineNode::Raw(content)
        }
    }

    pub fn new_paragraph(lines: &Vec<Line>) -> Node {
        Node::Paragraph {
            content: InlineNode::Raw(
                lines
                    .iter()
                    .map(|line| add_br_if_needed(&line.content))
                    .collect::<Vec<Vec<u32>>>()
                    .join(&[' ' as u32][..])
            )
        }
    }

    pub fn new_code_fence(
        lines: &Vec<Line>,
        language: &[u32],
        line_num: &Option<usize>,
        highlights: &Vec<usize>,
        copy_button: bool,
        index: usize
    ) -> Node {
        Node::FencedCode(FencedCode::new(
            lines.iter().map(|line| line.to_raw()).collect::<Vec<Vec<u32>>>().join(&['\n' as u32][..]),
            language.to_vec(),
            line_num.clone(),
            highlights.clone(),
            copy_button,
            index
        ))
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

    pub fn new_macro(line: &Line, macro_id: u64) -> Node {
        Node::MultiLineMacro(MultiLineMacro::from_line(line, macro_id))
    }

    pub fn new_math_ml(lines: &Vec<Line>, macro_id: u64) -> Node {
        Node::MultiLineMacro(
            MultiLineMacro {
                macro_type: MultiLineMacroType::Math(
                    lines.iter().map(
                        |line| line.to_raw()
                    ).collect::<Vec<Vec<u32>>>().join(&['\n' as u32][..])
                ),
                is_closing: false,
                id: macro_id
            },
        )
    }

}