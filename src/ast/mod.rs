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
        let mut buffer = Vec::with_capacity(self.nodes.len());
        let class_prefix = &self.render_option.class_prefix;

        // 1. It renderes the toc_data into a raw html string

        let mut toc_buffer = vec![];

        if self.doc_data.has_toc {

            for node in self.toc.iter() {
                node.to_html(&vec![], &self.render_option, &mut self.doc_data, &mut toc_buffer);
            }

        }

        let toc_rendered = toc_buffer.concat();

        // 2. It renders all the inner nodes to raw html strings

        for node in self.nodes.iter() {
            node.to_html(&toc_rendered, &self.render_option, &mut self.doc_data, &mut buffer);
        }

        // 3. It renders footnote_references

        if self.doc_data.footnote_references.len() > 0 {
            buffer.push(footnotes_to_html(&mut self.doc_data.footnote_references, &toc_rendered, class_prefix));
        }

        // 4. It appends scripts if needed

        let enable_js_for_tables = self.doc_data.has_collapsible_table && self.render_option.javascript_collapsible_tables;
        let enable_js_for_copy_buttons = self.doc_data.fenced_code_contents.len() > 0 && self.render_option.javascript_copy_buttons;
        let enable_js_for_tooltips = self.doc_data.tooltip_count > 0 && self.render_option.javascript_collapsible_tables;

        if enable_js_for_copy_buttons || enable_js_for_tables || enable_js_for_tooltips {
            buffer.push(into_v32("<script>"));

            if self.render_option.xml {
                buffer.push(into_v32("/*<![CDATA[*/"));
            }

            if enable_js_for_tables {
                buffer.push(into_v32(&collapsible_table_javascript()));
            }

            if enable_js_for_copy_buttons {
                buffer.push(into_v32(&copy_button_javascript(&self.doc_data.fenced_code_contents)));
            }

            if enable_js_for_tooltips {
                buffer.push(into_v32(&tooltip_javascript()));
            }

            // TODO: if self.doc_data.fenced_code_contents has `']]>'` inside, it wouldn't work
            if self.render_option.xml {
                buffer.push(into_v32("/*]]>*/"));
            }

            buffer.push(into_v32("</script>"));
        }

        buffer.concat()
    }

}
