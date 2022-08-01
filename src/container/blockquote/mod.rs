#[cfg(test)]
mod testbench;

use crate::render::render_option::RenderOption;
use crate::ast::doc_data::DocData;
use crate::ast::line::Line;
use crate::inline::InlineNode;
use crate::utils::into_v16;

#[derive(Clone)]
pub struct Blockquote {
    elements: Vec<ElementOrIndent>
}

impl Blockquote {

    pub fn to_html(&self, toc_rendered: &[u16], class_prefix: &str) -> Vec<u16> {
        let mut level = 0;
        let mut result = Vec::with_capacity(self.elements.len() * 2);

        for element in self.elements.iter() {

            match element {
                ElementOrIndent::Indent(n) => {
                    result.push(vec![into_v16("<blockquote>"); *n].concat());
                    level += *n;
                },
                ElementOrIndent::Element(element) => {
                    result.push(element.to_html(toc_rendered, class_prefix));
                    result.push(into_v16(" "));  // `\n` is converted to ` `
                }
            }

        }

        result.push(vec![into_v16("</blockquote>"); level].concat());
        result.concat()
    }

    pub fn parse_inlines(&mut self, doc_data: &mut DocData, options: &RenderOption) {

        for element in self.elements.iter_mut() {

            match element {
                ElementOrIndent::Element(content) => {content.parse_raw(doc_data, options);}
                _ => {}
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

                elements.push(ElementOrIndent::Element(InlineNode::Raw(line.content[end_index..].to_vec())));
            }

            else {
                elements.push(ElementOrIndent::Element(InlineNode::Raw(line.content.clone())));
            }

        }

        Blockquote { elements }
    }

}

fn count_level_and_end_index(content: &[u16]) -> (usize, usize) {  // (level, end_index)

    let mut level = 0;
    let mut conseq_space = 0;

    for (ind, c) in content.iter().enumerate() {

        if *c == '>' as u16 {
            level += 1;
            conseq_space = 0;
        }

        else if *c == ' ' as u16 {
            conseq_space += 1;

            if conseq_space == 4 {
                return (level, ind);
            }

        }

        else {
            return (level, ind);
        }

    }

    (level, content.len())
}

#[derive(Clone)]
enum ElementOrIndent {
    Element(InlineNode),
    Indent(usize)
}