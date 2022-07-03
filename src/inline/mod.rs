mod predicate;
pub mod parse;

#[cfg(test)]
mod testbench;

use crate::utils::into_v16;

pub enum InlineNode {
    Raw(Vec<u16>),
    Complex(Vec<Box<InlineNode>>),
    CodeSpan(Vec<u16>),
    Footnote((usize, Vec<u16>)),  // index, label
    Link {
        text: Vec<Box<InlineNode>>,
        destination: Vec<u16>
    },
    Image {
        description: Vec<u16>,
        address: Vec<u16>
    },
    Decoration {
        deco_type: DecorationType,
        content: Vec<Box<InlineNode>>
    }
}

pub enum DecorationType {
    Bold, Italic, Underline, Deletion, Subscript, Superscript,
    Macro(InlineMacro)
}

pub enum InlineMacro {
    Alignment(Vec<u16>),
    Color(Vec<u16>),
    Size(Vec<u16>),
    Char(u16),
    Math(Vec<u16>),
    Box,
    Toc,
    Blank,
    Br,
    Icon {
        name: Vec<u16>,
        size: u16
    }
}

impl InlineNode {

    pub fn to_html(&self) -> Vec<u16> {
        match self {
            InlineNode::Raw(content) => content.clone(),

            InlineNode::CodeSpan(content) => vec![
                into_v16("<code class=\"short\">"),
                content.clone(),
                into_v16("</code>")
            ].concat(),

            InlineNode::Footnote((index, _)) => into_v16(&format!(
                "<sup><a href=\"#footnote_cite{}\">{}</a></sup>",
                index,
                index
            )),

            InlineNode::Complex(content) => content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),

            InlineNode::Link {text, destination} => vec![
                into_v16("<a href=\""),
                destination.clone(),
                into_v16("\">"),
                text.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                into_v16("</a>")
            ].concat(),

            InlineNode::Image {description, address} => vec![
                into_v16("<img src=\""),
                address.clone(),
                into_v16("\" alt=\""),
                description.clone(),
                into_v16("\"/>")
            ].concat(),

            InlineNode::Decoration {deco_type, content} => match deco_type {
                DecorationType::Italic => vec![
                    into_v16("<em>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</em>")
                ].concat(),
                DecorationType::Bold => vec![
                    into_v16("<strong>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</strong>")
                ].concat(),
                DecorationType::Underline => vec![
                    into_v16("<u>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</u>")
                ].concat(),
                DecorationType::Deletion => vec![
                    into_v16("<del>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</del>")
                ].concat(),
                DecorationType::Subscript => vec![
                    into_v16("<sub>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</sub>")
                ].concat(),
                DecorationType::Superscript => vec![
                    into_v16("<sup>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</sup>")
                ].concat(),
                DecorationType::Macro(macro_type) => match macro_type {
                    InlineMacro::Color(color) => vec![
                        into_v16("<div class=\"color_"),
                        color.clone(),
                        into_v16("\">"),
                        content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</div>")
                    ].concat(),
                    InlineMacro::Size(size) => vec![
                        into_v16("<div class=\"size_"),
                        size.clone(),
                        into_v16("\">"),
                        content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</div>")
                    ].concat(),
                    InlineMacro::Alignment(alignment) => vec![
                        into_v16("<div class=\"align_"),
                        alignment.clone(),
                        into_v16("\">"),
                        content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</div>")
                    ].concat(),
                    InlineMacro::Box => vec![
                        into_v16("<div class=\"box\">"),
                        content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</div>")
                    ].concat(),
                    InlineMacro::Char(num) => into_v16(&format!("&#{};", num)),
                    InlineMacro::Br => into_v16("<br/>"),
                    InlineMacro::Blank => into_v16("&nbsp;"),
                    InlineMacro::Toc | InlineMacro::Math(_) | InlineMacro::Icon { .. } => todo!()
                }
            }
        }
    }

    pub fn to_md(&self) -> Vec<u16> {
        match self {
            InlineNode::Raw(content) => content.clone(),

            InlineNode::CodeSpan(content) => vec![
                into_v16("`"),
                content.clone(),
                into_v16("`")
            ].concat(),

            InlineNode::Footnote((_, label)) => vec![
                into_v16("["),
                label.clone(),
                into_v16("]")
            ].concat(),

            InlineNode::Complex(content) => content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),

            InlineNode::Link {text, destination} => vec![
                into_v16("["),
                text.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                into_v16("]("),
                destination.clone(),
                into_v16(")")
            ].concat(),

            InlineNode::Image {description, address} => vec![
                into_v16("!["),
                description.clone(),
                into_v16("]("),
                address.clone(),
                into_v16(")")
            ].concat(),

            InlineNode::Decoration {deco_type, content} => match deco_type {
                DecorationType::Italic => vec![
                    into_v16("*"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("*")
                ].concat(),
                DecorationType::Bold => vec![
                    into_v16("**"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("**")
                ].concat(),
                DecorationType::Underline => vec![
                    into_v16("~_"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("_~")
                ].concat(),
                DecorationType::Deletion => vec![
                    into_v16("~~"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("~~")
                ].concat(),
                DecorationType::Subscript => vec![
                    into_v16("~"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("~")
                ].concat(),
                DecorationType::Superscript => vec![
                    into_v16("^"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("^")
                ].concat(),
                DecorationType::Macro(macro_type) => match macro_type {
                    InlineMacro::Color(name) | InlineMacro::Size(name) | InlineMacro::Alignment(name) => vec![
                        into_v16("[["),
                        name.clone(),
                        into_v16("]]"),
                        content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("[[/"),
                        name.clone(),
                        into_v16("]]")
                    ].concat(),
                    InlineMacro::Box => vec![
                        into_v16("[[box]]"),
                        content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("[[/box]]"),
                    ].concat(),
                    InlineMacro::Math(content) => vec![
                        into_v16("[[math]]"),
                        content.clone(),
                        into_v16("[[/math]]"),
                    ].concat(),
                    InlineMacro::Char(num) => into_v16(&format!("[[char={}]]", num)),
                    InlineMacro::Br => into_v16("[[br]]"),
                    InlineMacro::Blank => into_v16("[[blank]]"),
                    InlineMacro::Toc => into_v16("[[toc]]"),
                    InlineMacro::Icon { .. } => todo!()
                }
            }
        }
    }

    pub fn to_vec(self) -> Vec<Box<InlineNode>> {

        match self {
            InlineNode::Raw(_) => vec![Box::new(self)],
            InlineNode::Complex(vec) => vec,
            _ => panic!("oh no!")
        }

    }

}

const INLINE_CODESPAN_MARKER1: u16 = u16::MAX - 1999;
const INLINE_CODESPAN_MARKER2: u16 = u16::MAX - 1998;
const INLINE_CODESPAN_MARKER3: u16 = u16::MAX - 1997;
const INLINE_CODESPAN_MARKER4: u16 = u16::MAX - 1996;