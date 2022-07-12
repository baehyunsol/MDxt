pub mod line;
pub mod parse;
pub mod doc_data;
mod predicate;
mod node;

#[cfg(test)]
mod testbench;

use crate::inline::InlineNode;
use crate::inline::footnote::{footnotes_to_html, Footnote};
use crate::render::render_option::RenderOption;
use crate::utils::into_v16;
use node::Node;
use doc_data::DocData;

pub struct AST {
    render_option: RenderOption,
    pub doc_data: DocData,
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
                Node::Paragraph { content } => {content.parse_raw(&mut self.doc_data, &self.render_option);},
                Node::Header { content, .. } => {
                    let tmp = self.render_option.render_macro;
                    self.render_option.render_macro = false;

                    // macros in headers are not rendered
                    content.parse_raw(&mut self.doc_data, &self.render_option);

                    self.render_option.render_macro = tmp;
                },
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
                Node::List(list) => {
                    result.push(list.to_html());
                }
                Node::Blockquote(blockquote) => {
                    result.push(blockquote.to_html());
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
                            content.to_html(),
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
            result.push(footnotes_to_html(&mut self.doc_data.footnote_references));
        }

        if self.doc_data.has_collapsible_table {

            // it doesn't add javascript for collapsed tables
            // instead, it sets render_result.has_collapsible_table to `true`, so that external engines can handle it

            /*result.push(into_v16("<script>

</script>
            "));*/
        }

        result.concat()
    }

}
