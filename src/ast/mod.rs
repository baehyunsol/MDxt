pub mod line;
pub mod parse;
mod predicate;
mod node;

#[cfg(test)]
mod testbench;

use crate::inline::InlineNode;
use crate::inline::footnote::{footnotes_to_html, Footnote};
use crate::render::render_option::RenderOption;
use crate::utils::into_v16;
use node::Node;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MdData {
    headers: Vec<(usize, Vec<u16>)>,  // (level, content)
    pub has_math: bool,
    pub link_references: HashMap<Vec<u16>, Vec<u16>>,  // (label, destination)
    pub footnote_references: HashMap<Vec<u16>, Footnote>,  // (label, footnote)
    footnote_reference_count: usize
}

impl Default for MdData {

    fn default() -> Self {
        MdData {
            headers: vec![],
            has_math: false,
            link_references: HashMap::new(),
            footnote_references: HashMap::new(),
            footnote_reference_count: 0
        }
    }

}

impl MdData {

    pub fn new(headers: Vec<(usize, Vec<u16>)>, link_references: HashMap<Vec<u16>, Vec<u16>>, footnote_references: HashMap<Vec<u16>, Footnote>) -> Self {
        MdData { headers, link_references, footnote_references, footnote_reference_count: 0, has_math: false }
    }

    pub fn add_footnote_inverse_index(&mut self, label: &Vec<u16>) -> usize {
        let footnote = self.footnote_references.get_mut(label).unwrap();
        footnote.inverse_index.push(self.footnote_reference_count);
        self.footnote_reference_count += 1;

        self.footnote_reference_count - 1
    }

}

pub struct AST {
    render_option: RenderOption,
    pub md_data: MdData,
    nodes: Vec<Node>,
    is_inline_parsed: bool
}

impl AST {

    pub fn parse_inlines(&mut self) {

        if self.is_inline_parsed {
            return;
        }

        self.nodes.iter_mut().for_each(
            |node| match node {
                Node::Paragraph { content } => {content.parse_raw(&mut self.md_data, &self.render_option);},
                Node::Header { content, .. } => {
                    let tmp = self.render_option.is_macro_enabled;
                    self.render_option.is_macro_enabled = false;

                    // macros in headers are not rendered
                    content.parse_raw(&mut self.md_data, &self.render_option);

                    self.render_option.is_macro_enabled = tmp;
                },
                Node::Table(table) => {table.parse_inlines(&mut self.md_data, &self.render_option);},
                Node::Empty | Node::FencedCode {..} | Node::ThematicBreak => {}
            }
        );

        // I couldn't find any better way to avoid the borrow checker
        if self.md_data.footnote_references.len() > 0 {
            let mut md_data_cloned = self.md_data.clone();
            let render_option_cloned = self.render_option.clone();

            let footnote_parsed = self.md_data.footnote_references.iter().map(
                |(label, Footnote { content, .. })| {
                    let mut footnote_content = content.clone();
                    footnote_content.parse_raw(&mut md_data_cloned, &render_option_cloned);
                    (label.clone(), footnote_content)
                }
            ).collect::<Vec<(Vec<u16>, InlineNode)>>();

            for (label, content) in footnote_parsed.into_iter() {
                let mut footnote_reference = self.md_data.footnote_references.get_mut(&label).unwrap();
                footnote_reference.content = content;
            }

        }

        self.is_inline_parsed = true;
    }

    pub fn to_html(&mut self) -> Vec<u16> {
        self.parse_inlines();
        let mut result = Vec::with_capacity(self.nodes.len());

        for node in self.nodes.iter() {

            match node {
                Node::Paragraph { content } => {
                    result.push(
                        vec![
                            into_v16("<p>"),
                            content.to_html(),
                            into_v16("</p>")
                        ].concat()
                    );
                },
                Node::ThematicBreak => {
                    result.push(
                        into_v16("<hr/>")
                    );
                },
                Node::Table(table) => {
                    result.push(table.to_html());
                }
                Node::Header { level, content } => {
                    result.push(
                        vec![
                            into_v16(&format!("<h{}>", level)),
                            content.to_html(),
                            into_v16(&format!("</h{}>", level)),
                        ].concat()
                    );
                },
                Node::FencedCode { language, line_num, content } => todo!(),
                Node::Empty => {}
            }

        }

        if self.md_data.footnote_references.len() > 0 {
            result.push(footnotes_to_html(&mut self.md_data.footnote_references));
        }

        result.concat()
    }

}
