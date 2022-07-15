pub mod line;
pub mod parse;
pub mod doc_data;
pub mod node;
mod predicate;

#[cfg(test)]
mod testbench;

use crate::inline::{
    InlineNode,
    footnote::{footnotes_to_html, Footnote}
};
use crate::render::render_option::RenderOption;
use crate::utils::into_v16;
use crate::{mathjax_javascript, collapsible_table_javascript};
use std::collections::HashMap;
use node::Node;
use doc_data::DocData;

#[derive(Clone)]
pub struct AST {
    pub render_option: RenderOption,
    pub doc_data: DocData,
    pub nodes: Vec<Node>,
    pub toc: Vec<Node>,
    is_inline_parsed: bool
}

impl AST {

    pub fn parse_inlines(&mut self) {

        if self.is_inline_parsed {
            return;
        }

        self.nodes.iter_mut().for_each(
            |node| match node {
                Node::Paragraph { content } | Node::Header { content, .. } => {content.parse_raw(&mut self.doc_data, &self.render_option);},
                Node::Table(table) => {table.parse_inlines(&mut self.doc_data, &self.render_option);},
                Node::List(list) => {list.parse_inlines(&mut self.doc_data, &self.render_option);},
                Node::Blockquote(blockquote) => {blockquote.parse_inlines(&mut self.doc_data, &self.render_option);},
                Node::Empty | Node::FencedCode {..} | Node::ThematicBreak => {}
            }
        );

        // I couldn't find any better way to avoid the borrow checker
        if self.doc_data.footnote_references.len() > 0 {
            let mut doc_data_cloned = self.doc_data.clone();
            let render_option_cloned = self.render_option.clone();

            let footnote_parsed = self.doc_data.footnote_references.iter().map(
                |(label, Footnote { content, .. })| {
                    let mut footnote_content = content.clone();
                    footnote_content.parse_raw(&mut doc_data_cloned, &render_option_cloned);
                    (label.clone(), footnote_content)
                }
            ).collect::<Vec<(Vec<u16>, InlineNode)>>();

            for (label, content) in footnote_parsed.into_iter() {
                let mut footnote_reference = self.doc_data.footnote_references.get_mut(&label).unwrap();
                footnote_reference.content = content;
            }

        }

        self.is_inline_parsed = true;

        if self.doc_data.has_toc {
            self.render_toc();
        }

    }

    pub fn to_html(&mut self) -> Vec<u16> {
        self.parse_inlines();
        let mut result = Vec::with_capacity(self.nodes.len());

        // TODO: this block is to ugly
        let toc_rendered = if self.doc_data.has_toc {
            let mut tmp_ast_for_toc = self.clone();
            tmp_ast_for_toc.nodes = tmp_ast_for_toc.toc.clone();
            tmp_ast_for_toc.doc_data.has_toc = false;  // to prevent infinite recursion
            tmp_ast_for_toc.doc_data.footnote_references = HashMap::new();  // not to render it multiple times
            tmp_ast_for_toc.render_option.javascript = false;  // not to render it multiple times
            tmp_ast_for_toc.to_html()
        } else {
            vec![]
        };

        for node in self.nodes.iter() {

            match node {
                Node::Paragraph { content } => {
                    result.push(
                        vec![
                            into_v16("<p>"),
                            content.to_html(&toc_rendered),
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
                    result.push(table.to_html(&toc_rendered));
                }
                Node::List(list) => {
                    result.push(list.to_html(&toc_rendered));
                }
                Node::Blockquote(blockquote) => {
                    result.push(blockquote.to_html(&toc_rendered));
                }
                Node::Header { level, content, anchor } => {

                    let anchor = if self.render_option.header_anchor && anchor.len() > 0 {
                        vec![
                            into_v16(&format!(" id=\"")),
                            anchor.to_vec(),
                            into_v16("\"")
                        ].concat()
                    } else {
                        into_v16("")
                    };

                    result.push(
                        vec![
                            into_v16(&format!("<h{}", level)),
                            anchor,
                            into_v16(">"),
                            content.to_html(&toc_rendered),
                            into_v16(&format!("</h{}>", level)),
                        ].concat()
                    );
                },
                Node::FencedCode(fenced_code) => {
                    result.push(fenced_code.to_html());
                }
                Node::Empty => {}
            }

        }

        if self.doc_data.footnote_references.len() > 0 {
            result.push(footnotes_to_html(&mut self.doc_data.footnote_references, &toc_rendered));
        }

        if self.doc_data.has_collapsible_table && self.render_option.javascript {
            result.push(into_v16("<script>"));
            result.push(into_v16(&collapsible_table_javascript()));
            result.push(into_v16("</script>"));
        }

        if self.doc_data.has_math && self.render_option.javascript {
            result.push(into_v16(&mathjax_javascript()));
        }

        result.concat()
    }

}
