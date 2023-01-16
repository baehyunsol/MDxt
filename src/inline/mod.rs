pub mod footnote;
pub mod link;
pub mod macros;
pub mod math;
pub mod parse;
mod predicate;

#[cfg(test)]
mod testbench;

use crate::container::icon::get_icon;
use crate::utils::{from_v16, into_v16};
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

    // `[[char = 32]]` -> `32` -> `&#32;`
    // `[[char = therefore]]` -> `there4` -> `&there4;`
    Char(Vec<u16>),

    Math(Vec<u16>),
    Box {
        border: bool,
        inline: bool,
        width: Vec<u16>,
        height: Vec<u16>,
    },
    Toc,
    Tooltip {
        message: Vec<Box<InlineNode>>,
        index: usize,
        label: Vec<u16>
    },
    Blank { repeat: usize },
    Br { repeat: usize },
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

    pub fn to_html(&self, toc_rendered: &[u16], class_prefix: &str) -> Vec<u16> {
        match self {
            InlineNode::Raw(content) => content.clone(),

            InlineNode::CodeSpan(content) => vec![
                into_v16(&format!("<code class=\"{}inline-code-span\">", class_prefix)),
                content.clone(),
                into_v16("</code>")
            ].concat(),

            InlineNode::Footnote((index, inverse_index, _)) => into_v16(&format!(
                "<span class=\"{}footnote-ref\" id=\"footnote-ref-{}\"><a href=\"#footnote-cite-{}\">[{}]</a></span>",
                class_prefix,
                inverse_index,
                index,
                inverse_index
            )),

            InlineNode::Complex(content) => content.iter().map(
                |node| node.to_html(toc_rendered, class_prefix)
            ).collect::<Vec<Vec<u16>>>().concat(),

            InlineNode::Link {text, destination} => vec![
                into_v16("<a href=\""),
                destination.clone(),
                into_v16("\">"),
                text.iter().map(
                    |node| node.to_html(toc_rendered, class_prefix)
                ).collect::<Vec<Vec<u16>>>().concat(),
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
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</em>")
                ].concat(),
                DecorationType::Bold => vec![
                    into_v16("<strong>"),
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</strong>")
                ].concat(),
                DecorationType::Underline => vec![
                    into_v16("<u>"),
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</u>")
                ].concat(),
                DecorationType::Deletion => vec![
                    into_v16("<del>"),
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</del>")
                ].concat(),
                DecorationType::Subscript => vec![
                    into_v16("<sub>"),
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</sub>")
                ].concat(),
                DecorationType::Superscript => vec![
                    into_v16("<sup>"),
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</sup>")
                ].concat(),
                DecorationType::Macro(macro_type) => match macro_type {
                    InlineMacro::Color(color) => vec![
                        into_v16(&format!("<span class=\"{}color-", class_prefix)),
                        color.clone(),
                        into_v16("\">"),
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span>")
                    ].concat(),
                    InlineMacro::Size(size) => vec![
                        into_v16(&format!("<span class=\"{}size-", class_prefix)),
                        size.clone(),
                        into_v16("\">"),
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span>")
                    ].concat(),
                    InlineMacro::Highlight(color) => vec![
                        into_v16(&format!("<span class=\"{}highlight-", class_prefix)),
                        color.clone(),
                        into_v16("\">"),
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span>")
                    ].concat(),
                    InlineMacro::Alignment(alignment) => vec![
                        into_v16(&format!("<span class=\"{}align-", class_prefix)),
                        alignment.clone(),
                        into_v16("\">"),
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span>")
                    ].concat(),
                    InlineMacro::Box { border, inline, width, height } => vec![
                        into_v16(&format!(
                            "<span class=\"{}box{}{}{}{}\">",
                            class_prefix,
                            if !border {
                                format!(" {}no-border", class_prefix)
                            } else {
                                String::new()
                            },
                            if *inline {
                                format!(" {}inline", class_prefix)
                            } else {
                                String::new()
                            },
                            if width.len() > 0 {
                                format!(" {}width-{}", class_prefix, from_v16(&width))
                            } else {
                                String::new()
                            },
                            if height.len() > 0 {
                                format!(" {}height-{}", class_prefix, from_v16(&height))
                            } else {
                                String::new()
                            }
                        )),
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span>")
                    ].concat(),
                    InlineMacro::HTML { tag, class, id } => {
                        let mut result = vec![];

                        result.push(into_v16("<"));

                        if tag == &into_v16("anchor") {
                            result.push(into_v16("a"));
                        }

                        else {
                            result.push(tag.clone());
                        }

                        if class.len() > 0 {
                            result.push(into_v16(&format!(" class=\"{}", class_prefix)));
                            result.push(class.clone());
                            result.push(into_v16("\""));
                        }

                        if id.len() > 0 {
                            result.push(into_v16(" id=\""));
                            result.push(id.clone());
                            result.push(into_v16("\""));
                        }

                        result.push(into_v16(">"));
                        result.push(content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u16>>>().concat());
                        result.push(into_v16("</"));

                        if tag == &into_v16("anchor") {
                            result.push(into_v16("a"));
                        }

                        else {
                            result.push(tag.clone());
                        }

                        result.push(into_v16(">"));

                        result.concat()
                    }
                    InlineMacro::Tooltip { message, index, .. } => vec![
                        into_v16(&format!(
                            "<span class=\"{}tooltip-container\" id=\"tooltip-container-{}\">",
                            class_prefix,
                            index
                        )),
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16(&format!(
                            "<span class=\"{}tooltip-message\" id=\"tooltip-message-{}\">",
                            class_prefix,
                            index
                        )),
                        message.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("</span></span>")
                    ].concat(),
                    InlineMacro::Char(character) => if character[0] < 'A' as u16 {
                        vec![
                            into_v16("&#"),
                            character.clone(),
                            into_v16(";")
                        ].concat()
                    } else {
                        vec![
                            into_v16("&"),
                            character.clone(),
                            into_v16(";")
                        ].concat()
                    },
                    InlineMacro::Br { repeat } => vec![into_v16("<br/>"); *repeat].concat(),
                    InlineMacro::Blank { repeat } => vec![into_v16("&nbsp;"); *repeat].concat(),
                    InlineMacro::Math (content) => render_math(content),
                    InlineMacro::Toc => toc_rendered.to_vec(),
                    InlineMacro::Icon { name, size } => get_icon(name, *size as usize, None, false).unwrap()
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

            InlineNode::Complex(content) => content.iter().map(
                |node| node.to_mdxt()
            ).collect::<Vec<Vec<u16>>>().concat(),

            InlineNode::Link {text, destination} => vec![
                into_v16("["),
                text.iter().map(
                    |node| node.to_mdxt()
                ).collect::<Vec<Vec<u16>>>().concat(),
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
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("*")
                ].concat(),
                DecorationType::Bold => vec![
                    into_v16("**"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("**")
                ].concat(),
                DecorationType::Underline => vec![
                    into_v16("~_"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("_~")
                ].concat(),
                DecorationType::Deletion => vec![
                    into_v16("~~"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("~~")
                ].concat(),
                DecorationType::Subscript => vec![
                    into_v16("~"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("~")
                ].concat(),
                DecorationType::Superscript => vec![
                    into_v16("^"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("^")
                ].concat(),
                DecorationType::Macro(macro_type) => match macro_type {
                    InlineMacro::Color(name) | InlineMacro::Size(name) | InlineMacro::Alignment(name) => vec![
                        into_v16("[["),
                        name.clone(),
                        into_v16("]]"),
                        content.iter().map(
                            |node| node.to_mdxt()
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("[[/"),
                        name.clone(),
                        into_v16("]]")
                    ].concat(),
                    InlineMacro::Highlight(color) => vec![
                        into_v16("[[highlight="),
                        color.clone(),
                        into_v16("]]"),
                        content.iter().map(
                            |node| node.to_mdxt()
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("[[/highlight]]")
                    ].concat(),
                    InlineMacro::Tooltip { label, .. } => vec![
                        into_v16("[[tooltip="),
                        label.clone(),
                        into_v16("]]"),
                        content.iter().map(
                            |node| node.to_mdxt()
                        ).collect::<Vec<Vec<u16>>>().concat(),
                        into_v16("[[/tooltip]]")
                    ].concat(),
                    InlineMacro::Box { border, inline, width, height } => vec![
                        into_v16(&format!(
                            "[[box{}{}{}{}]]",
                            if !border {
                                ", no border"
                            } else {
                                ""
                            },
                            if *inline {
                                ", inline"
                            } else {
                                ""
                            },
                            if width.len() > 0 {
                                from_v16(&width)
                            } else {
                                String::new()
                            },
                            if height.len() > 0 {
                                from_v16(&height)
                            } else {
                                String::new()
                            },
                        )),
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
                        result.push(content.iter().map(
                            |node| node.to_mdxt()
                        ).collect::<Vec<Vec<u16>>>().concat());
                        result.push(into_v16("[[/"));
                        result.push(tag.clone());
                        result.push(into_v16("]]"));

                        result.concat()
                    }
                    InlineMacro::Char(character) => vec![
                        into_v16("[[char="),
                        character.clone(),
                        into_v16("]]")
                    ].concat(),
                    InlineMacro::Br { repeat } => if *repeat == 1 {
                        into_v16("[[br]]")
                    } else {
                        into_v16(&format!("[[br={}]]", repeat))
                    },
                    InlineMacro::Blank { repeat } => if *repeat == 1 {
                        into_v16("[[blank]]")
                    } else {
                        into_v16(&format!("[[blank={}]]", repeat))
                    },
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

// those are illegal unicodes, which are appropriate to be used as internal meta characters
const INLINE_CODE_SPAN_MARKER1: u16 = 0xd800;
const INLINE_CODE_SPAN_MARKER2: u16 = 0xd801;
const INLINE_CODE_SPAN_MARKER3: u16 = 0xd802;
const INLINE_CODE_SPAN_MARKER4: u16 = 0xd803;