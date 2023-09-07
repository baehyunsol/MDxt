mod auto_url;
pub mod footnote;
pub mod link;
pub mod macros;
pub mod math;
pub mod parse;
mod predicate;

#[cfg(test)]
mod testbench;

pub use auto_url::render_auto_urls;
use crate::container::icon::get_icon;
use crate::utils::{from_v32, into_v32};
use crate::file_ext::{FileExt, read_file_extension};
use link::is_youtube;
use math::render_math;

#[derive(Clone)]
pub enum MediaType {
    Image,
    Video(FileExt),
    Audio(FileExt),
    Youtube
}

impl MediaType {

    pub fn from_url(v: &[u32], enable_youtube: bool) -> Self {

        if let Some(ext) = read_file_extension(v) {

            match ext {
                FileExt::Mp4 | FileExt::Webm => MediaType::Video(ext),
                FileExt::Mp3 | FileExt::Wav | FileExt::Ogg | FileExt::M4a => MediaType::Audio(ext),
                FileExt::Jpg | FileExt::Png | FileExt::Svg | FileExt::Gif => MediaType::Image
            }

        }

        else if enable_youtube && is_youtube(v) {
            MediaType::Youtube
        }

        else {
            MediaType::Image
        }

    }

}

#[derive(Clone)]
pub enum InlineNode {
    Raw(Vec<u32>),
    Complex(Vec<InlineNode>),
    CodeSpan(Vec<u32>),
    Footnote((usize, usize, Vec<u32>)),  // index, inverse_index, label
    Link {
        text: Vec<InlineNode>,
        destination: Vec<u32>
    },
    Image {
        media_type: MediaType,
        description: Vec<u32>,
        address: Vec<u32>
    },
    Decoration {
        deco_type: DecorationType,
        content: Vec<InlineNode>
    }
}

#[derive(Clone)]
pub enum DecorationType {
    Bold, Italic, Underline, Deletion, Subscript, Superscript,
    Macro(InlineMacro), None
}

#[derive(Clone)]
pub enum InlineMacro {
    Alignment(Vec<u32>),
    Color(Vec<u32>),
    Size(Vec<u32>),
    LineHeight(Vec<u32>),
    Highlight(Vec<u32>),

    // `[[char = 32]]` -> `32` -> `&#32;`
    // `[[char = therefore]]` -> `there4` -> `&there4;`
    Char(Vec<u32>),

    Math(Vec<u32>),
    Box {
        border: bool,
        inline: bool,
        width: Vec<u32>,
        height: Vec<u32>,
    },
    Toc,
    Tooltip {
        message: Vec<InlineNode>,
        index: usize,
        label: Vec<u32>
    },
    Blank { repeat: usize },
    Br { repeat: usize },
    HTML {
        tag: Vec<u32>,
        class: Vec<u32>,
        id: Vec<u32>
    },
    Icon {
        name: Vec<u32>,
        size: u32
    }
}

impl InlineNode {

    pub fn to_html(&self, toc_rendered: &[u32], class_prefix: &str) -> Vec<u32> {
        match self {
            InlineNode::Raw(content) => content.clone(),

            InlineNode::CodeSpan(content) => vec![
                into_v32(&format!("<code class=\"{class_prefix}inline-code-span\">")),
                content.clone(),
                vec![60, 47, 99, 111, 100, 101, 62]  // into_v32("</code>")
            ].concat(),

            InlineNode::Footnote((index, inverse_index, _)) => into_v32(&format!(
                "<span class=\"{class_prefix}footnote-ref\" id=\"footnote-ref-{inverse_index}\"><a href=\"#footnote-cite-{index}\">[{inverse_index}]</a></span>",
            )),

            InlineNode::Complex(content) => content.iter().map(
                |node| node.to_html(toc_rendered, class_prefix)
            ).collect::<Vec<Vec<u32>>>().concat(),

            InlineNode::Link {text, destination} => vec![
                vec![60, 97, 32, 104, 114, 101, 102, 61, 34],  // into_v32("<a href=\""),
                destination.clone(),
                vec![34, 62],  // into_v32("\">")
                text.iter().map(
                    |node| node.to_html(toc_rendered, class_prefix)
                ).collect::<Vec<Vec<u32>>>().concat(),
                vec![60, 47, 97, 62],  // into_v32("</a>")
            ].concat(),

            InlineNode::Image {description, address, media_type} => match media_type {
                MediaType::Image => vec![
                    vec![60, 105, 109, 103, 32, 115, 114, 99, 61, 34],  // into_v32("<img src=\""),
                    address.clone(),
                    vec![34, 32, 97, 108, 116, 61, 34],  // into_v32("\" alt=\""),
                    description.clone(),
                    vec![34, 47, 62],  // into_v32("\"/>")
                ].concat(),
                MediaType::Video(ext) => vec![
                    into_v32("<video controls=\"controls\">"),  // `<video controls>` is not compatible with my HXML lib
                    into_v32("<source src=\""),
                    address.clone(),
                    into_v32("\" type=\"video/"),
                    ext.mime_type(),
                    vec![34, 47, 62],  // into_v32("\"/>")
                    description.clone(),
                    into_v32("</video>"),
                ].concat(),
                MediaType::Audio(ext) => vec![
                    into_v32("<audio controls=\"controls\">"),  // `<audio controls>` is not compatible with my HXML lib
                    into_v32("<source src=\""),
                    address.clone(),
                    into_v32("\" type=\"audio/"),
                    ext.mime_type(),
                    vec![34, 47, 62],  // into_v32("\"/>")
                    description.clone(),
                    into_v32("</audio>"),
                ].concat(),
                MediaType::Youtube => vec![
                    into_v32("<iframe src=\"https://www.youtube.com/embed/"),
                    address.clone(),
                    into_v32("\"></iframe>")
                ].concat(),
            },

            InlineNode::Decoration { deco_type, content } => match deco_type {
                DecorationType::Italic => vec![
                    vec![60, 101, 109, 62],  // into_v32("<em>")
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    vec![60, 47, 101, 109, 62],  // into_v32("</em>")
                ].concat(),
                DecorationType::Bold => vec![
                    vec![60, 115, 116, 114, 111, 110, 103, 62],  // into_v32("<strong>")
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    vec![60, 47, 115, 116, 114, 111, 110, 103, 62],  // into_v32("</strong>")
                ].concat(),
                DecorationType::Underline => vec![
                    vec![60, 117, 62],  // into_v32("<u>")
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    vec![60, 47, 117, 62],  // into_v32("</u>")
                ].concat(),
                DecorationType::Deletion => vec![
                    vec![60, 100, 101, 108, 62],  // into_v32("<del>")
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    vec![60, 47, 100, 101, 108, 62],  // into_v32("</del>")
                ].concat(),
                DecorationType::Subscript => vec![
                    vec![60, 115, 117, 98, 62],  // into_v32("<sub>")
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    vec![60, 47, 115, 117, 98, 62],  // into_v32("</sub>")
                ].concat(),
                DecorationType::Superscript => vec![
                    vec![60, 115, 117, 112, 62],  // into_v32("<sup>")
                    content.iter().map(
                        |node| node.to_html(toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    vec![60, 47, 115, 117, 112, 62],  // into_v32("</sup>")
                ].concat(),
                DecorationType::None => {
                    #[cfg(test)] assert!(content.is_empty());

                    vec![]
                }
                DecorationType::Macro(macro_type) => match macro_type {
                    InlineMacro::Color(color) => vec![
                        into_v32(&format!("<span class=\"{class_prefix}color-")),
                        color.clone(),
                        vec![34, 62],  // into_v32("\">")
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        vec![60, 47, 115, 112, 97, 110, 62],  // into_v32("</span>")
                    ].concat(),
                    InlineMacro::LineHeight(height) => vec![
                        into_v32(&format!("<span class=\"{class_prefix}line-height-")),
                        height.clone(),
                        vec![34, 62],  // into_v32("\">")
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        vec![60, 47, 115, 112, 97, 110, 62],  // into_v32("</span>")
                    ].concat(),
                    InlineMacro::Size(size) => vec![
                        into_v32(&format!("<span class=\"{class_prefix}size-")),
                        size.clone(),
                        vec![34, 62],  // into_v32("\">")
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        vec![60, 47, 115, 112, 97, 110, 62],  // into_v32("</span>")
                    ].concat(),
                    InlineMacro::Highlight(color) => vec![
                        into_v32(&format!("<span class=\"{}highlight-", class_prefix)),
                        color.clone(),
                        vec![34, 62],  // into_v32("\">")
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        vec![60, 47, 115, 112, 97, 110, 62],  // into_v32("</span>")
                    ].concat(),
                    InlineMacro::Alignment(alignment) => vec![
                        into_v32(&format!("<span class=\"{class_prefix}align-")),
                        alignment.clone(),
                        vec![34, 62],  // into_v32("\">")
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        vec![60, 47, 115, 112, 97, 110, 62],  // into_v32("</span>")
                    ].concat(),
                    InlineMacro::Box { border, inline, width, height } => vec![
                        into_v32(&format!(
                            "<span class=\"{class_prefix}box{}{}{}{}\">",
                            if !border {
                                format!(" {class_prefix}no-border")
                            } else {
                                String::new()
                            },
                            if *inline {
                                format!(" {class_prefix}inline")
                            } else {
                                String::new()
                            },
                            if !width.is_empty() {
                                format!(" {class_prefix}width-{}", from_v32(&width))
                            } else {
                                String::new()
                            },
                            if !height.is_empty() {
                                format!(" {class_prefix}height-{}", from_v32(&height))
                            } else {
                                String::new()
                            }
                        )),
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        vec![60, 47, 115, 112, 97, 110, 62],  // into_v32("</span>")
                    ].concat(),
                    InlineMacro::HTML { tag, class, id } => {
                        let mut result = vec![];

                        result.push(vec![60]);  // into_v32("<")

                        // into_v32("anchor") -> [97, 110, 99, 104, 111, 114]
                        if tag == &[97, 110, 99, 104, 111, 114] {
                            result.push(vec![97]);  // into_v32("a")
                        }

                        else {
                            result.push(tag.clone());
                        }

                        if !class.is_empty() {
                            result.push(into_v32(&format!(" class=\"{class_prefix}")));
                            result.push(class.clone());
                            result.push(vec![34]);  // into_v32("\"")
                        }

                        if !id.is_empty() {
                            result.push(vec![32, 105, 100, 61, 34]);  // into_v32(" id=\"")
                            result.push(id.clone());
                            result.push(vec![34]);  // into_v32("\"")
                        }

                        result.push(vec![62]);  // into_v32(">")
                        result.push(content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat());
                        result.push(vec![60, 47]);  // into_v32("</")

                        // into_v32("anchor") -> [97, 110, 99, 104, 111, 114]
                        if tag == &[97, 110, 99, 104, 111, 114] {
                            result.push(vec![97]);  // into_v32("a")
                        }

                        else {
                            result.push(tag.clone());
                        }

                        result.push(vec![62]);  // into_v32(">")

                        result.concat()
                    }
                    InlineMacro::Tooltip { message, index, .. } => vec![
                        into_v32(&format!(
                            "<span class=\"{class_prefix}tooltip-container\" id=\"tooltip-container-{index}\">",
                        )),
                        content.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        into_v32(&format!(
                            "<span class=\"{class_prefix}tooltip-message\" id=\"tooltip-message-{index}\">",
                        )),
                        message.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat(),

                        // into_v32("</span></span>\n") -> [60, 47, 115, 112, 97, 110, 62, 60, 47, 115, 112, 97, 110, 62, 10]
                        vec![60, 47, 115, 112, 97, 110, 62, 60, 47, 115, 112, 97, 110, 62, 10]
                    ].concat(),
                    InlineMacro::Char(character) => if character[0] < 'A' as u32 {
                        vec![
                            vec![38, 35],  // into_v32("&#")
                            character.clone(),
                            vec![59],  // into_v32(";")
                        ].concat()
                    } else {
                        vec![
                            vec![38],  // into_v32("&")
                            character.clone(),
                            vec![59],  // into_v32(";")
                        ].concat()
                    },

                    // into_v32("<br/>") -> [60, 98, 114, 47, 62]
                    InlineMacro::Br { repeat } => vec![vec![60, 98, 114, 47, 62]; *repeat].concat(),

                    // into_v32("&nbsp;") -> [38, 110, 98, 115, 112, 59]
                    InlineMacro::Blank { repeat } => vec![vec![38, 110, 98, 115, 112, 59]; *repeat].concat(),
                    InlineMacro::Math (content) => render_math(content),
                    InlineMacro::Toc => toc_rendered.to_vec(),
                    InlineMacro::Icon { name, size } => get_icon(name, *size as usize, None, false).unwrap()
                }
            }
        }
    }

    #[cfg(test)]  // it's used for roundtrip tests
    pub fn to_mdxt(&self) -> Vec<u32> {
        match self {
            InlineNode::Raw(content) => content.clone(),

            InlineNode::CodeSpan(content) => {
                let backtick_count = content.iter().filter(
                    |c| **c == '`' as u32
                ).collect::<Vec<&u32>>().len();
                let backtick_string = vec!['`' as u32; backtick_count + 1];

                vec![
                    backtick_string.clone(),
                    into_v32(" "),
                    content.clone(),
                    into_v32(" "),
                    backtick_string
                ].concat()
            },

            InlineNode::Footnote((_, _, label)) => vec![
                into_v32("["),
                label.clone(),
                into_v32("]")
            ].concat(),

            InlineNode::Complex(content) => content.iter().map(
                |node| node.to_mdxt()
            ).collect::<Vec<Vec<u32>>>().concat(),

            InlineNode::Link {text, destination} => vec![
                into_v32("["),
                text.iter().map(
                    |node| node.to_mdxt()
                ).collect::<Vec<Vec<u32>>>().concat(),
                into_v32("]("),
                destination.clone(),
                into_v32(")")
            ].concat(),

            InlineNode::Image {description, address, ..} => vec![
                into_v32("!["),
                description.clone(),
                into_v32("]("),
                address.clone(),
                into_v32(")")
            ].concat(),

            InlineNode::Decoration {deco_type, content} => match deco_type {
                DecorationType::Italic => vec![
                    into_v32("*"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    into_v32("*")
                ].concat(),
                DecorationType::Bold => vec![
                    into_v32("**"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    into_v32("**")
                ].concat(),
                DecorationType::Underline => vec![
                    into_v32("~_"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    into_v32("_~")
                ].concat(),
                DecorationType::Deletion => vec![
                    into_v32("~~"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    into_v32("~~")
                ].concat(),
                DecorationType::Subscript => vec![
                    into_v32("~"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    into_v32("~")
                ].concat(),
                DecorationType::Superscript => vec![
                    into_v32("^"),
                    content.iter().map(
                        |node| node.to_mdxt()
                    ).collect::<Vec<Vec<u32>>>().concat(),
                    into_v32("^")
                ].concat(),
                DecorationType::None => {
                    #[cfg(test)] assert!(content.is_empty());

                    vec![]
                }
                DecorationType::Macro(macro_type) => match macro_type {
                    InlineMacro::Color(name) | InlineMacro::Size(name) | InlineMacro::Alignment(name) => vec![
                        into_v32("[["),
                        name.clone(),
                        into_v32("]]"),
                        content.iter().map(
                            |node| node.to_mdxt()
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        into_v32("[[/"),
                        name.clone(),
                        into_v32("]]")
                    ].concat(),
                    InlineMacro::LineHeight(height) => vec![
                        into_v32("[[lineheight="),
                        height.clone(),
                        into_v32("]]"),
                        content.iter().map(
                            |node| node.to_mdxt()
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        into_v32("[[/lineheight]]")
                    ].concat(),
                    InlineMacro::Highlight(color) => vec![
                        into_v32("[[highlight="),
                        color.clone(),
                        into_v32("]]"),
                        content.iter().map(
                            |node| node.to_mdxt()
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        into_v32("[[/highlight]]")
                    ].concat(),
                    InlineMacro::Tooltip { label, .. } => vec![
                        into_v32("[[tooltip="),
                        label.clone(),
                        into_v32("]]"),
                        content.iter().map(
                            |node| node.to_mdxt()
                        ).collect::<Vec<Vec<u32>>>().concat(),
                        into_v32("[[/tooltip]]")
                    ].concat(),
                    InlineMacro::Box { border, inline, width, height } => vec![
                        into_v32(&format!(
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
                            if !width.is_empty() {
                                from_v32(&width)
                            } else {
                                String::new()
                            },
                            if !height.is_empty() {
                                from_v32(&height)
                            } else {
                                String::new()
                            },
                        )),
                        content.iter().map(|node| node.to_mdxt()).collect::<Vec<Vec<u32>>>().concat(),
                        into_v32("[[/box]]"),
                    ].concat(),
                    InlineMacro::Math(content) => vec![
                        into_v32("[[math]]"),
                        content.clone(),
                        into_v32("[[/math]]"),
                    ].concat(),
                    InlineMacro::HTML { tag, class, id } => {
                        let mut result = vec![];
                        result.push(into_v32("[["));
                        result.push(tag.clone());

                        let classes = class.split(
                            |c| *c == ' ' as u32
                        ).map(
                            |class| vec![
                                into_v32(",class="),
                                class.to_vec()
                            ].concat()
                        ).collect::<Vec<Vec<u32>>>().concat();

                        let ids = id.split(
                            |c| *c == ' ' as u32
                        ).map(
                            |id| vec![
                                into_v32(",id="),
                                id.to_vec()
                            ].concat()
                        ).collect::<Vec<Vec<u32>>>().concat();

                        result.push(classes);
                        result.push(ids);
                        result.push(into_v32("]]"));
                        result.push(content.iter().map(
                            |node| node.to_mdxt()
                        ).collect::<Vec<Vec<u32>>>().concat());
                        result.push(into_v32("[[/"));
                        result.push(tag.clone());
                        result.push(into_v32("]]"));

                        result.concat()
                    }
                    InlineMacro::Char(character) => vec![
                        into_v32("[[char="),
                        character.clone(),
                        into_v32("]]")
                    ].concat(),
                    InlineMacro::Br { repeat } => if *repeat == 1 {
                        into_v32("[[br]]")
                    } else {
                        into_v32(&format!("[[br={repeat}]]"))
                    },
                    InlineMacro::Blank { repeat } => if *repeat == 1 {
                        into_v32("[[blank]]")
                    } else {
                        into_v32(&format!("[[blank={repeat}]]"))
                    },
                    InlineMacro::Toc => into_v32("[[toc]]"),
                    InlineMacro::Icon { name, size } => vec![
                        into_v32("[[icon="),
                        name.clone(),
                        into_v32(&format!(",size={size}]]"))
                    ].concat(),
                }
            }
        }
    }

    pub fn to_vec(self) -> Vec<InlineNode> {

        match self {
            InlineNode::Raw(_) => vec![self],
            InlineNode::Complex(vec) => vec,
            _ => unreachable!()
        }

    }

}

// those are illegal unicodes, which are appropriate to be used as internal meta characters
const INLINE_CODE_SPAN_MARKER1: u32 = 0x602_005;
const INLINE_CODE_SPAN_MARKER2: u32 = 0x602_006;
const INLINE_CODE_SPAN_MARKER3: u32 = 0x602_007;
const INLINE_CODE_SPAN_MARKER4: u32 = 0x602_008;
