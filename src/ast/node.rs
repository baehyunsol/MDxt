use super::line::add_br_if_needed;
use crate::RenderOption;
use crate::ast::{doc_data::DocData, line::Line};
use crate::inline::InlineNode;
use crate::inline::macros::multiline::{MultiLineMacro, MultiLineMacroType};
use crate::container::{
    blockquote::Blockquote,
    codefence::FencedCode,
    header::normalize_header,
    list::List,
    table::Table,
};
use crate::utils::into_v32;

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

    pub fn set_inner_nodes(&mut self, nodes: Vec<Node>) {

        match self {
            Node::MultiLineMacro(mm) => {
                mm.macro_type.set_inner_nodes(nodes);
            },
            _ => unreachable!()
        }

    }

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

    pub fn new_macro(line: &Line, doc_data: &mut DocData) -> Node {
        Node::MultiLineMacro(MultiLineMacro::from_line(line, doc_data))
    }

    pub fn new_math_ml(lines: &Vec<Line>) -> Node {
        Node::MultiLineMacro(
            MultiLineMacro {
                macro_type: MultiLineMacroType::Math(
                    lines.iter().map(
                        |line| line.to_raw()
                    ).collect::<Vec<Vec<u32>>>().join(&['\n' as u32][..])
                ),
                is_closing: false,
            },
        )
    }

    pub fn parse_inlines(&mut self, render_option: &RenderOption, doc_data: &mut DocData) {

        match self {
            Node::Paragraph { content } | Node::Header { content, .. } => { content.parse_raw(doc_data, render_option); },
            Node::Table(table) => { table.parse_inlines(doc_data, render_option); },
            Node::List(list) => { list.parse_inlines(doc_data, render_option); },
            Node::Blockquote(blockquote) => { blockquote.parse_inlines(doc_data, render_option); },
            Node::Empty | Node::ThematicBreak | Node::MultiLineMacro(_) => {},

            // TODO
            // this branch is ugly...
            // it doesn't `parse_inline` inside the `parse_inlines` function
            // but this is the only point where the `FencedCode` instances and `doc_data` meet
            // I should call this function when the fenced_codes are initialized, but `doc_data` doesn't exist at that timing
            Node::FencedCode(fenced_code) => { doc_data.add_fenced_code_content(fenced_code); },
        }

    }

    pub fn to_html(&self, toc_rendered: &Vec<u32>, render_option: &RenderOption, doc_data: &mut DocData, buffer: &mut Vec<Vec<u32>>) {
        let class_prefix = &render_option.class_prefix;

        match self {
            Node::Paragraph { content } => {
                buffer.push(
                    vec![
                        vec![60, 112, 62],  // into_v32("<p>")
                        content.to_html(toc_rendered, class_prefix),
                        vec![60, 47, 112, 62],  // into_v32("</p>")
                    ].concat()
                );
            },
            Node::ThematicBreak => {
                buffer.push(
                    vec![60, 104, 114, 47, 62]  // into_v32("<hr/>")
                );
            },
            Node::Table(table) => {
                buffer.push(table.to_html(toc_rendered, class_prefix));
            }
            Node::List(list) => {
                buffer.push(list.to_html(toc_rendered, class_prefix));
            }
            Node::Blockquote(blockquote) => {
                buffer.push(blockquote.to_html(toc_rendered, class_prefix));
            }
            Node::MultiLineMacro(multiline_macro) => {
                buffer.push(multiline_macro.to_html(toc_rendered, render_option, doc_data));
            }
            Node::Header { level, content, anchor } => {

                let anchor = if render_option.header_anchor && anchor.len() > 0 {
                    vec![
                        vec![32, 105, 100, 61, 34],  // into_v32(&format!(" id=\"")),
                        anchor.to_vec(),
                        vec![34],  // into_v32("\"")
                    ].concat()
                } else {
                    vec![]
                };

                buffer.push(
                    vec![
                        into_v32(&format!("<h{level}")),
                        anchor,
                        vec![62],  // into_v32(">"),
                        content.to_html(toc_rendered, class_prefix),
                        into_v32(&format!("</h{level}>")),
                    ].concat()
                );
            },
            Node::FencedCode(fenced_code) => {
                buffer.push(fenced_code.to_html(class_prefix));
            }
            Node::Empty => {}
        }

    }

}