#[cfg(test)]
mod testbench;

use crate::ast::doc_data::DocData;
use crate::ast::line::{add_br_if_needed, Line};
use crate::escape::HTML_ESCAPE_OFFSET;
use crate::inline::InlineNode;
use crate::render::render_option::RenderOption;
use crate::utils::into_v32;

#[derive(Clone)]
pub struct Blockquote {
    elements: Vec<ElementOrIndent>
}

impl Blockquote {

    pub fn to_html(&self, toc_rendered: &[u32], class_prefix: &str) -> Vec<u32> {
        let mut level = 0;
        let mut result = Vec::with_capacity(self.elements.len() * 2);

        for element in self.elements.iter() {

            match element {
                ElementOrIndent::Indent(n) => {
                    result.push(vec![into_v32("<blockquote>"); *n].concat());
                    level += *n;
                },
                ElementOrIndent::Element(element) => {
                    result.push(element.to_html(toc_rendered, class_prefix));
                    result.push(into_v32(" "));  // `\n` is converted to ` `
                }
            }

        }

        result.push(vec![into_v32("</blockquote>"); level].concat());
        result.concat()
    }

    pub fn parse_inlines(&mut self, doc_data: &mut DocData, options: &RenderOption) {

        for element in self.elements.iter_mut() {

            if let ElementOrIndent::Element(content) = element {
                content.parse_raw(doc_data, options);
            }
        }

    }

    pub fn from_lines(lines: &Vec<Line>) -> Self {
        let mut curr_level = 0;
        let mut elements = Vec::with_capacity(lines.len());

        for line in lines.iter() {

            if line.is_blockquote() {
                let (level, end_index) = count_level_and_end_index(&line.content);

                if level > curr_level {
                    elements.push(ElementOrIndent::Indent(level - curr_level));
                    curr_level = level;
                }

                elements.push(ElementOrIndent::Element(InlineNode::Raw(add_br_if_needed(&line.content[end_index..]))));
            }

            else {
                elements.push(ElementOrIndent::Element(InlineNode::Raw(add_br_if_needed(&line.content))));
            }

        }

        Blockquote { elements }
    }

}

fn count_level_and_end_index(content: &[u32]) -> (usize, usize) {  // (level, end_index)
    let mut level = 0;
    let mut conseq_space = 0;

    for (index, ch) in content.iter().enumerate() {

        if *ch == HTML_ESCAPE_OFFSET + '>' as u32 {
            level += 1;
            conseq_space = 0;
        }

        else if *ch == ' ' as u32 {
            conseq_space += 1;

            if conseq_space == 4 {
                return (level, index);
            }

        }

        else {
            return (level, index);
        }

    }

    (level, content.len())
}

#[derive(Clone)]
enum ElementOrIndent {
    Element(InlineNode),
    Indent(usize)
}