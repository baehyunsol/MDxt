pub mod doc_data;
pub mod line;
pub mod node;
pub mod parse;
mod predicate;

#[cfg(test)]
mod testbench;

use crate::inline::{
    InlineNode,
    footnote::{footnotes_to_html, Footnote}
};
use crate::{collapsible_table_javascript, tooltip_javascript};
use crate::container::codefence::html::copy_button_javascript;
use crate::render::render_option::RenderOption;
use crate::utils::into_v32;
use doc_data::DocData;
use node::Node;
use std::collections::HashMap;

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
                Node::Paragraph { content } | Node::Header { content, .. } => { content.parse_raw(&mut self.doc_data, &self.render_option); },
                Node::Table(table) => { table.parse_inlines(&mut self.doc_data, &self.render_option); },
                Node::List(list) => { list.parse_inlines(&mut self.doc_data, &self.render_option); },
                Node::Blockquote(blockquote) => { blockquote.parse_inlines(&mut self.doc_data, &self.render_option); },
                Node::Empty | Node::ThematicBreak | Node::MultiLineMacro(_) => {},

                // TODO
                // this branch is ugly...
                // it doesn't `parse_inline` inside the `parse_inlines` function
                // but this is the only point where the `FencedCode` instances and `doc_data` meet
                // I should call this function when the fenced_codes are initialized, but `doc_data` doesn't exist at that timing
                Node::FencedCode(fenced_code) => { self.doc_data.add_fenced_code_content(fenced_code); },
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
            ).collect::<Vec<(Vec<u32>, InlineNode)>>();

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

    pub fn to_html(&mut self) -> Vec<u32> {
        self.parse_inlines();
        let mut result = Vec::with_capacity(self.nodes.len());
        let class_prefix = &self.render_option.class_prefix;

        // TODO: this block is to ugly
        let toc_rendered = if self.doc_data.has_toc {
            let mut tmp_ast_for_toc = self.clone();
            tmp_ast_for_toc.nodes = tmp_ast_for_toc.toc.clone();
            tmp_ast_for_toc.doc_data.has_toc = false;  // to prevent infinite recursion
            tmp_ast_for_toc.doc_data.footnote_references = HashMap::new();  // not to render it multiple times
            tmp_ast_for_toc.render_option.javascript_collapsible_tables = false;  // not to render it multiple times
            tmp_ast_for_toc.render_option.javascript_copy_buttons = false;  // not to render it multiple times
            tmp_ast_for_toc.to_html()
        } else {
            vec![]
        };

        for node in self.nodes.iter() {

            match node {
                Node::Paragraph { content } => {
                    result.push(
                        vec![
                            into_v32("<p>"),
                            content.to_html(&toc_rendered, class_prefix),
                            into_v32("</p>")
                        ].concat()
                    );
                },
                Node::ThematicBreak => {
                    result.push(
                        into_v32("<hr/>")
                    );
                },
                Node::Table(table) => {
                    result.push(table.to_html(&toc_rendered, class_prefix));
                }
                Node::List(list) => {
                    result.push(list.to_html(&toc_rendered, class_prefix));
                }
                Node::Blockquote(blockquote) => {
                    result.push(blockquote.to_html(&toc_rendered, class_prefix));
                }
                Node::MultiLineMacro(multiline_macro) => {
                    result.push(multiline_macro.to_html(class_prefix));
                }
                Node::Header { level, content, anchor } => {

                    let anchor = if self.render_option.header_anchor && anchor.len() > 0 {
                        vec![
                            into_v32(&format!(" id=\"")),
                            anchor.to_vec(),
                            into_v32("\"")
                        ].concat()
                    } else {
                        into_v32("")
                    };

                    result.push(
                        vec![
                            into_v32(&format!("<h{}", level)),
                            anchor,
                            into_v32(">"),
                            content.to_html(&toc_rendered, class_prefix),
                            into_v32(&format!("</h{}>", level)),
                        ].concat()
                    );
                },
                Node::FencedCode(fenced_code) => {
                    result.push(fenced_code.to_html(class_prefix));
                }
                Node::Empty => {}
            }

        }

        if self.doc_data.footnote_references.len() > 0 {
            result.push(footnotes_to_html(&mut self.doc_data.footnote_references, &toc_rendered, class_prefix));
        }

        let enable_js_for_tables = self.doc_data.has_collapsible_table && self.render_option.javascript_collapsible_tables;
        let enable_js_for_copy_buttons = self.doc_data.fenced_code_contents.len() > 0 && self.render_option.javascript_copy_buttons;
        let enable_js_for_tooltips = self.doc_data.tooltip_count > 0 && self.render_option.javascript_collapsible_tables;

        if enable_js_for_copy_buttons || enable_js_for_tables || enable_js_for_tooltips {
            result.push(into_v32("<script>"));

            if self.render_option.xml {
                result.push(into_v32("/*<![CDATA[*/"));
            }

            if enable_js_for_tables {
                result.push(into_v32(&collapsible_table_javascript()));
            }

            if enable_js_for_copy_buttons {
                result.push(into_v32(&copy_button_javascript(&self.doc_data.fenced_code_contents)));
            }

            if enable_js_for_tooltips {
                result.push(into_v32(&tooltip_javascript()));
            }

            // TODO: if self.doc_data.fenced_code_contents has `']]>'` inside, it wouldn't work
            if self.render_option.xml {
                result.push(into_v32("/*]]>*/"));
            }

            result.push(into_v32("</script>"));
        }

        result.concat()
    }

}
