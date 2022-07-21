mod predicate;
pub mod parse;
pub mod link;
pub mod footnote;
pub mod macros;
pub mod math;

#[cfg(test)]
mod testbench;

use crate::utils::into_v16;
use math::render_math;

#[derive(Clone)]
pub enum InlineNode {
    Raw(Vec<u16>),
    Complex(Vec<Box<InlineNode>>),
    CodeSpan(Vec<u16>),
    Footnote((usize, usize, Vec<u16>)),  // index, inverse_index, label
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

#[derive(Clone)]
pub enum DecorationType {
    Bold, Italic, Underline, Deletion, Subscript, Superscript,
    Macro(InlineMacro)
}

#[derive(Clone)]
pub enum InlineMacro {
    Alignment(Vec<u16>),
    Color(Vec<u16>),
    Size(Vec<u16>),
    Highlight(Vec<u16>),
    Char(u16),
    Math(Vec<u16>),
    Box { border: bool },
    Toc,
    Blank,
    Br,
    HTML {
        tag: Vec<u16>,
        class: Vec<u16>,
        id: Vec<u16>
    },
    Icon {
        name: Vec<u16>,
        size: u16
    }
}

impl InlineNode {

    pub fn to_html(&self, toc_rendered: &[u16]) -> Vec<u16> {
        match self {
            InlineNode::Raw(content) => content.clone(),

            InlineNode::CodeSpan(content) => vec![
                into_v16("<code class=\"short\">"),
                content.clone(),
                into_v16("</code>")
            ].concat(),

            InlineNode::Footnote((index, inverse_index, _)) => into_v16(&format!(
                "<sup id=\"footnote_ref{}\"><a href=\"#footnote_cite{}\">[{}]</a></sup>",
                inverse_index,
                index,
                inverse_index
            )),

            InlineNode::Complex(content) => content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),

            InlineNode::Link {text, destination} => vec![
                into_v16("<a href=\""),
                destination.clone(),
                into_v16("\">"),
                text.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
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
                    content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</em>")
                ].concat(),
                DecorationType::Bold => vec![
                    into_v16("<strong>"),
                    content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</strong>")
                ].concat(),
                DecorationType::Underline => vec![
                    into_v16("<u>"),
                    content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</u>")
                ].concat(),
                DecorationType::Deletion => vec![
                    into_v16("<del>"),
                    content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</del>")
                ].concat(),
                DecorationType::Subscript => vec![
                    into_v16("<sub>"),
                    content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</sub>")
                ].concat(),
                DecorationType::Superscript => vec![
                    into_v16("<sup>"),
                    content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</sup>")
                ].concat(),
                DecorationType::Macro(macro_type) => match macro_type {
                    InlineMacro::Color(color) => vec![
                        into_v16("<span class=\"color_"),
                        color.clone(),
                        into_v16("\">"),
                        content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span>")
                    ].concat(),
                    InlineMacro::Size(size) => vec![
                        into_v16("<span class=\"size_"),
                        size.clone(),
                        into_v16("\">"),
                        content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span>")
                    ].concat(),
                    InlineMacro::Highlight(color) => vec![
                        into_v16("<span class=\"highlight_"),
                        color.clone(),
                        into_v16("\">"),
                        content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span>")
                    ].concat(),
                    InlineMacro::Alignment(alignment) => vec![
                        into_v16("<span class=\"align_"),
                        alignment.clone(),
                        into_v16("\">"),
                        content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span>")
                    ].concat(),
                    InlineMacro::Box { border } => vec![
                        if *border {
                            into_v16("<div class=\"box\">")
                        } else {
                            into_v16("<div class=\"box no-border\">")
                        },
                        content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</div>")
                    ].concat(),
                    InlineMacro::HTML { tag, class, id } => {
                        let mut result = vec![];

                        result.push(into_v16("<"));
                        result.push(tag.clone());

                        if class.len() > 0 {
                            result.push(into_v16(" class=\""));
                            result.push(class.clone());
                            result.push(into_v16("\""));
                        }

                        if id.len() > 0 {
                            result.push(into_v16(" id=\""));
                            result.push(id.clone());
                            result.push(into_v16("\""));
                        }

                        result.push(into_v16(">"));
                        result.push(content.iter().map(|node| node.to_html(toc_rendered)).collect::<Vec<Vec<u16>>>().concat());
                        result.push(into_v16("</"));
                        result.push(tag.clone());
                        result.push(into_v16(">"));

                        result.concat()
                    }
                    InlineMacro::Char(num) => into_v16(&format!("&#{};", num)),
                    InlineMacro::Br => into_v16("<br/>"),
                    InlineMacro::Blank => into_v16("&nbsp;"),
                    InlineMacro::Math (content) => render_math(content),
                    InlineMacro::Toc => toc_rendered.to_vec(),
                    InlineMacro::Icon { .. } => todo!()
                }
            }
        }
    }

    pub fn to_mdxt(&self) -> Vec<u16> {
        match self {
            InlineNode::Raw(content) => content.clone(),

            InlineNode::CodeSpan(content) => {
                let backtick_count = content.iter().filter(
                    |c| **c == '`' as u16
                ).collect::<Vec<&u16>>().len();
                let backtick_string = vec!['`' as u16; backtick_count + 1];

                vec![
                    backtick_string.clone(),
                    into_v16(" "),
                    content.clone(),
                    into_v16(" "),
                    backtick_string
                ].concat()
            },

            InlineNode::Footnote((_, _, label)) => vec![
                into_v16("["),
                label.clone(),
                into_v16("]")
            ].concat(),

            InlineNode::Complex(content) => content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),

            InlineNode::Link {text, destination} => vec![
                into_v16("["),
                text.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
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
                    content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("*")
                ].concat(),
                DecorationType::Bold => vec![
                    into_v16("**"),
                    content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("**")
                ].concat(),
                DecorationType::Underline => vec![
                    into_v16("~_"),
                    content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("_~")
                ].concat(),
                DecorationType::Deletion => vec![
                    into_v16("~~"),
                    content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("~~")
                ].concat(),
                DecorationType::Subscript => vec![
                    into_v16("~"),
                    content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("~")
                ].concat(),
                DecorationType::Superscript => vec![
                    into_v16("^"),
                    content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("^")
                ].concat(),
                DecorationType::Macro(macro_type) => match macro_type {
                    InlineMacro::Color(name) | InlineMacro::Size(name) | InlineMacro::Alignment(name) => vec![
                        into_v16("[["),
                        name.clone(),
                        into_v16("]]"),
                        content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("[[/"),
                        name.clone(),
                        into_v16("]]")
                    ].concat(),
                    InlineMacro::Highlight(color) => vec![
                        into_v16("[[highlight="),
                        color.clone(),
                        into_v16("]]"),
                        content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("[[/highlight]]")
                    ].concat(),
                    InlineMacro::Box { border } => vec![
                        if *border {
                            into_v16("[[box]]")
                        } else {
                            into_v16("[[box, no border]]")
                        },
                        content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("[[/box]]"),
                    ].concat(),
                    InlineMacro::Math(content) => vec![
                        into_v16("[[math]]"),
                        content.clone(),
                        into_v16("[[/math]]"),
                    ].concat(),
                    InlineMacro::HTML { tag, class, id } => {
                        let mut result = vec![];
                        result.push(into_v16("[["));
                        result.push(tag.clone());

                        let classes = class.split(
                            |c| *c == ' ' as u16
                        ).map(
                            |class| vec![
                                into_v16(",class="),
                                class.to_vec()
                            ].concat()
                        ).collect::<Vec<Vec<u16>>>().concat();

                        let ids = id.split(
                            |c| *c == ' ' as u16
                        ).map(
                            |id| vec![
                                into_v16(",id="),
                                id.to_vec()
                            ].concat()
                        ).collect::<Vec<Vec<u16>>>().concat();

                        result.push(classes);
                        result.push(ids);
                        result.push(into_v16("]]"));
                        result.push(content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u16>>>().concat());
                        result.push(into_v16("[[/"));
                        result.push(tag.clone());
                        result.push(into_v16("]]"));

                        result.concat()
                    }
                    InlineMacro::Char(num) => into_v16(&format!("[[char={}]]", num)),
                    InlineMacro::Br => into_v16("[[br]]"),
                    InlineMacro::Blank => into_v16("[[blank]]"),
                    InlineMacro::Toc => into_v16("[[toc]]"),
                    InlineMacro::Icon { name, size } => vec![
                        into_v16("[[icon="),
                        name.clone(),
                        into_v16(&format!(",size={}]]", size))
                    ].concat()
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

const INLINE_CODE_SPAN_MARKER1: u16 = u16::MAX - 1999;
const INLINE_CODE_SPAN_MARKER2: u16 = u16::MAX - 1998;
const INLINE_CODE_SPAN_MARKER3: u16 = u16::MAX - 1997;
const INLINE_CODE_SPAN_MARKER4: u16 = u16::MAX - 1996;