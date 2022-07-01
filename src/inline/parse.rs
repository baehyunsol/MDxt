use super::predicate::*;
use super::{
    InlineNode, DecorationType,
    INLINE_CODESPAN_MARKER1, INLINE_CODESPAN_MARKER2, INLINE_CODESPAN_MARKER3, INLINE_CODESPAN_MARKER4
};
use crate::link::predicate::{
    read_direct_link, read_reference_link, read_shortcut_reference_link
};
use crate::link::normalize_link;
use crate::render::render_option::RenderOption;
use crate::macros::predicate::check_and_parse_macro_inline;
use std::collections::HashMap;

impl InlineNode {

    pub fn parse_raw(&mut self, link_references: &HashMap<Vec<u16>, Vec<u16>>, render_option: &mut RenderOption) {

        match self {
            InlineNode::Raw(content) => {
                *self = Self::from_md(&content, link_references, render_option);
            }
            _ => panic!("the program's logic went wrong!")
        }

    }

    pub fn from_md(content: &[u16], link_references: &HashMap<Vec<u16>, Vec<u16>>, render_option: &mut RenderOption) -> Self {

        // it has to be rendered before other inline elements
        let content = &escape_code_spans(content);

        let mut index = 0;
        let mut result: Vec<Box<InlineNode>> = vec![];

        while index < content.len() {

            /*
            if not special_character: continue;
            */

            if is_code_span_marker_begin(content, index) {
                index = get_code_span_marker_end_index(content, index);
                continue;
            }

            match is_bold_italic(content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Italic,

                        content: vec![Box::new(InlineNode::Decoration{
                            deco_type: DecorationType::Bold,

                            // `Self::from_md` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                            content: Self::from_md(&content[index + 3..end - 2], link_references, render_option).to_vec()
                        })]
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[end + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_deletion_subscript(content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Deletion,

                        content: vec![Box::new(InlineNode::Decoration{
                            deco_type: DecorationType::Subscript,

                            // `Self::from_md` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                            content: Self::from_md(&content[index + 3..end - 2], link_references, render_option).to_vec()
                        })]
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[end + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_italic(content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Italic,

                        // `Self::from_md` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_md(&content[index + 1..end], link_references, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[end + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_bold(content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Bold,

                        // `Self::from_md` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_md(&content[index + 2..end - 1], link_references, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[end + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_deletion(content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Deletion,

                        // `Self::from_md` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_md(&content[index + 2..end - 1], link_references, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[end + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_underline(content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Underline,

                        // `Self::from_md` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_md(&content[index + 2..end - 1], link_references, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[end + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_superscript(content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Superscript,

                        // `Self::from_md` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_md(&content[index + 1..end], link_references, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[end + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_subscript(content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Subscript,

                        // `Self::from_md` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_md(&content[index + 1..end], link_references, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[end + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match read_direct_link(content, index, link_references) {
                Some((link_text, link_destination, last_index)) => {
                    let mut result = vec![];
                    let mut is_image = false;

                    if index > 0 && content[index - 1] == '!' as u16 {
                        is_image = true;
                        index -= 1;
                    }

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    if is_image {
                        result.push(Box::new(InlineNode::Image {
                            description: undo_code_span_escapes(&link_text),
                            address: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    else {
                        result.push(Box::new(InlineNode::Link {
                            text: Self::from_md(&link_text, link_references, render_option).to_vec(),
                            destination: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    if last_index + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[last_index + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match read_reference_link(content, index, link_references) {
                Some((link_text, link_label, last_index)) => {
                    let mut result = vec![];
                    let mut is_image = false;

                    // the existence of the link reference was tested by the `read_reference_link` function
                    let link_destination = link_references.get(&normalize_link(&link_label)).unwrap();

                    if index > 0 && content[index - 1] == '!' as u16 {
                        is_image = true;
                        index -= 1;
                    }

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    if is_image {
                        result.push(Box::new(InlineNode::Image {
                            description: undo_code_span_escapes(&link_text),
                            address: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    else {
                        result.push(Box::new(InlineNode::Link {
                            text: Self::from_md(&link_text, link_references, render_option).to_vec(),
                            destination: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    if last_index + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[last_index + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match read_shortcut_reference_link(content, index, link_references) {
                Some((link_text, last_index)) => {
                    let mut result = vec![];
                    let mut is_image = false;

                    // the existence of the link reference was tested by the `read_reference_link` function
                    let link_destination = link_references.get(&normalize_link(&link_text)).unwrap();

                    if index > 0 && content[index - 1] == '!' as u16 {
                        is_image = true;
                        index -= 1;
                    }

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    if is_image {
                        result.push(Box::new(InlineNode::Image {
                            description: undo_code_span_escapes(&link_text),
                            address: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    else {
                        result.push(Box::new(InlineNode::Link {
                            text: Self::from_md(&link_text, link_references, render_option).to_vec(),
                            destination: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    if last_index + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[last_index + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match check_and_parse_macro_inline(content, index, link_references, render_option) {
                Some((parsed, last_index)) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(content[0..index].to_vec())));
                    }

                    result.push(Box::new(parsed));

                    if last_index + 1 < content.len() {
                        result.push(Box::new(Self::from_md(&content[last_index + 1..content.len()], link_references, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                }
                _ => {}
            }

            index += 1;
        }

        // there're no inline element in the content
        InlineNode::Raw(content.to_vec()).render_code_spans()
    }

    pub fn render_code_spans(mut self) -> Self {
        match self {
            InlineNode::Raw(content) => {
                let mut complex_contents = vec![];
                let mut index = 0;
                let mut last_index = 0;

                while index < content.len() {

                    if is_code_span_marker_begin(&content, index) {
                        let code_span_end_index = get_code_span_marker_end_index(&content, index);

                        if index > last_index {
                            complex_contents.push(Box::new(InlineNode::Raw(content[last_index..index].to_vec())));
                        }

                        if code_span_end_index > index + 2 {
                            complex_contents.push(Box::new(InlineNode::CodeSpan(content[index + 2..code_span_end_index].to_vec())));
                        }

                        last_index = code_span_end_index + 2;
                        index = code_span_end_index + 2;
                        continue;
                    }

                    index += 1;
                }

                if complex_contents.len() == 0 {
                    InlineNode::Raw(content)
                }

                else {

                    if content.len() > last_index {
                        complex_contents.push(Box::new(InlineNode::Raw(content[last_index..content.len()].to_vec())));
                    }

                    InlineNode::Complex(complex_contents)
                }
            },
            InlineNode::Complex(contents) => InlineNode::Complex(
                contents.into_iter().map(|node| Box::new(node.render_code_spans())).collect()
            ),
            InlineNode::Decoration {deco_type, content} => InlineNode::Decoration {
                deco_type,
                content: content.into_iter().map(|node| Box::new(node.render_code_spans())).collect()
            },
            InlineNode::Link {text, destination} => InlineNode::Link {
                text: text.into_iter().map(|node| Box::new(node.render_code_spans())).collect(),
                destination
            },
            InlineNode::Image {description, address} => InlineNode::Image {description, address},
            InlineNode::CodeSpan(content) => InlineNode::CodeSpan(content),
        }
    }

}

pub fn escape_code_spans(content: &[u16]) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len() * 5 / 4);
    let mut index = 0;

    while index < content.len() {

        match is_code_span(content, index) {
            Bool::True(end) => {
                result.push(INLINE_CODESPAN_MARKER1);
                result.push(INLINE_CODESPAN_MARKER2);
                index += 1;

                while index < end {
                    result.push(content[index]);
                    index += 1;
                }

                result.push(INLINE_CODESPAN_MARKER3);
                result.push(INLINE_CODESPAN_MARKER4);
                index += 1;
            },
            _ => {
                result.push(content[index]);
                index += 1;
            }
        }

    }

    result
}

pub fn undo_code_span_escapes(content: &[u16]) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        if is_code_span_marker_begin(content, index) || is_code_span_marker_end(content, index) {
            result.push('`' as u16);
            index += 1;
        }

        else {
            result.push(content[index]);
        }

        index += 1;
    }

    result
}

pub fn is_code_span_marker_begin(content: &[u16], index: usize) -> bool {
    content[index] == INLINE_CODESPAN_MARKER1 && index + 1 < content.len() && content[index + 1] == INLINE_CODESPAN_MARKER2
}

pub fn is_code_span_marker_end(content: &[u16], index: usize) -> bool {
    content[index] == INLINE_CODESPAN_MARKER3 && index + 1 < content.len() && content[index + 1] == INLINE_CODESPAN_MARKER4
}

pub fn get_code_span_marker_end_index(content: &[u16], mut index: usize) -> usize {

    while content[index] != INLINE_CODESPAN_MARKER3 || content[index + 1] != INLINE_CODESPAN_MARKER4 {
        index += 1;
    }

    index
}

#[cfg(test)]
mod tests {

    #[test]
    fn code_span_escape_test() {
        use super::{escape_code_spans, undo_code_span_escapes};
        use crate::utils::into_v16;

        let cases = vec![
            "``", "`a`", "`codespan`",
            "```", "`a`a", "codespan",
            "*`*", "`*`*", "`*`*`", "",
            "*`*`*`", "*`*`*`*", "`*`*`*`*", "`*`*`*`*`"
        ];

        let cases = cases.iter().map(|s| into_v16(s)).collect::<Vec<Vec<u16>>>();

        for case in cases.iter() {
            assert_eq!(case, &undo_code_span_escapes(&escape_code_spans(case)));
        }

    }

}