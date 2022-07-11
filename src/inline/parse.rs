use super::predicate::*;
use super::{
    InlineNode, DecorationType,
    INLINE_CODE_SPAN_MARKER1, INLINE_CODE_SPAN_MARKER2, INLINE_CODE_SPAN_MARKER3, INLINE_CODE_SPAN_MARKER4
};
use super::link::predicate::{
    read_direct_link, read_reference_link, read_shortcut_reference_link
};
use super::math::escape_inside_math_blocks;
use super::footnote::predicate::read_footnote;
use super::link::normalize_link;
use super::macros::predicate::check_and_parse_macro_inline;
use crate::ast::doc_data::DocData;
use crate::render::render_option::RenderOption;
use crate::utils::get_bracket_end_index;
use crate::escape::{render_backslash_escapes, undo_backslash_escapes};

impl InlineNode {

    pub fn parse_raw(
        &mut self,
        doc_data: &mut DocData,
        render_option: &RenderOption
    ) {

        match self {
            InlineNode::Raw(content) => {
                *self = Self::from_mdxt(&content, doc_data, render_option);
            }
            _ => {}  // it's already parsed
        }

    }

    pub fn from_mdxt(content: &[u16], doc_data: &mut DocData, render_option: &RenderOption) -> Self {

        // it prevents inline elements inside code spans from being rendered
        // code spans are rendered later
        let mut content = escape_code_spans(content);

        // inline elements inside math blocks are not rendered
        if render_option.is_macro_enabled {
            content = escape_inside_math_blocks(content);
        }

        let mut index = 0;

        while index < content.len() {

            if is_code_span_marker_begin(&content, index) {
                index = get_code_span_marker_end_index(&content, index);
                continue;
            }

            // it continues if the current character is not a special character
            if content[index] < '*' as u16 ||
            ('*' as u16) < content[index] && content[index] < '[' as u16 ||
            ('`' as u16) < content[index] && content[index] < '~' as u16 ||
            content[index] > '~' as u16 {
                index += 1;
                continue;
            }

            match is_bold_italic(&content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Italic,

                        content: vec![Box::new(InlineNode::Decoration{
                            deco_type: DecorationType::Bold,

                            // `Self::from_mdxt` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                            content: Self::from_mdxt(&content[index + 3..end - 2], doc_data, render_option).to_vec()
                        })]
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[end + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_deletion_subscript(&content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Deletion,

                        content: vec![Box::new(InlineNode::Decoration{
                            deco_type: DecorationType::Subscript,

                            // `Self::from_mdxt` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                            content: Self::from_mdxt(&content[index + 3..end - 2], doc_data, render_option).to_vec()
                        })]
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[end + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_italic(&content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Italic,

                        // `Self::from_mdxt` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_mdxt(&content[index + 1..end], doc_data, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[end + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_bold(&content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Bold,

                        // `Self::from_mdxt` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_mdxt(&content[index + 2..end - 1], doc_data, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[end + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_deletion(&content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Deletion,

                        // `Self::from_mdxt` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_mdxt(&content[index + 2..end - 1], doc_data, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[end + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_underline(&content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Underline,

                        // `Self::from_mdxt` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_mdxt(&content[index + 2..end - 1], doc_data, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[end + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_superscript(&content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Superscript,

                        // `Self::from_mdxt` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_mdxt(&content[index + 1..end], doc_data, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[end + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match is_subscript(&content, index) {
                Bool::True(end) => {
                    let mut result = vec![];

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Subscript,

                        // `Self::from_mdxt` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                        content: Self::from_mdxt(&content[index + 1..end], doc_data, render_option).to_vec()
                    }));

                    if end + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[end + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match read_direct_link(&content, index, &doc_data.link_references) {
                Some((link_text, link_destination, last_index)) => {
                    let mut result = vec![];
                    let mut is_image = false;

                    if index > 0 && content[index - 1] == '!' as u16 {
                        is_image = true;
                        index -= 1;
                    }

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    if is_image {
                        result.push(Box::new(InlineNode::Image {
                            description: undo_code_span_escapes(&link_text),
                            address: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    else {
                        result.push(Box::new(InlineNode::Link {
                            text: Self::from_mdxt(&link_text, doc_data, render_option).to_vec(),
                            destination: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    if last_index + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[last_index + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match read_reference_link(&content, index, &doc_data.link_references) {
                Some((link_text, link_label, last_index)) => {
                    let mut result = vec![];
                    let mut is_image = false;

                    // the existence of the link reference was tested by the `read_reference_link` function
                    // it clones to result in order to avoid the borrow checker
                    let link_destination = doc_data.link_references.get(&normalize_link(&link_label)).unwrap().clone();

                    if index > 0 && content[index - 1] == '!' as u16 {
                        is_image = true;
                        index -= 1;
                    }

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    if is_image {
                        result.push(Box::new(InlineNode::Image {
                            description: undo_code_span_escapes(&link_text),
                            address: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    else {
                        result.push(Box::new(InlineNode::Link {
                            text: Self::from_mdxt(&link_text, doc_data, render_option).to_vec(),
                            destination: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    if last_index + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[last_index + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match read_footnote(&content, index, &doc_data.footnote_references) {
                Some(footnote_index) => {
                    let bracket_end_index = get_bracket_end_index(&content, index).unwrap();
                    let footnote_label = normalize_link(&content[index + 1..bracket_end_index]);
                    let mut result = vec![];

                    let inverse_index = doc_data.add_footnote_inverse_index(&footnote_label);

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    result.push(Box::new(InlineNode::Footnote((footnote_index, inverse_index, footnote_label))));

                    if bracket_end_index + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[bracket_end_index + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            match read_shortcut_reference_link(&content, index, &doc_data.link_references) {
                Some((link_text, last_index)) => {
                    let mut result = vec![];
                    let mut is_image = false;

                    // the existence of the link reference was tested by the `read_reference_link` function
                    // it clones to result in order to avoid the borrow checker
                    let link_destination = doc_data.link_references.get(&normalize_link(&link_text)).unwrap().clone();

                    if index > 0 && content[index - 1] == '!' as u16 {
                        is_image = true;
                        index -= 1;
                    }

                    if index > 0 {
                        result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                    }

                    if is_image {
                        result.push(Box::new(InlineNode::Image {
                            description: undo_code_span_escapes(&link_text),
                            address: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    else {
                        result.push(Box::new(InlineNode::Link {
                            text: Self::from_mdxt(&link_text, doc_data, render_option).to_vec(),
                            destination: (render_option.link_handler)(&link_destination)
                        }));
                    }

                    if last_index + 1 < content.len() {
                        result.push(Box::new(Self::from_mdxt(&content[last_index + 1..content.len()], doc_data, render_option)));
                    }

                    return InlineNode::Complex(result).render_code_spans();
                },
                _ => {}
            }

            if render_option.is_macro_enabled {

                match check_and_parse_macro_inline(&content, index, doc_data, render_option) {
                    Some((parsed, last_index)) => {
                        let mut result = vec![];

                        if index > 0 {
                            result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                        }

                        result.push(Box::new(parsed));

                        if last_index + 1 < content.len() {
                            result.push(Box::new(Self::from_mdxt(&content[last_index + 1..content.len()], doc_data, render_option)));
                        }

                        return InlineNode::Complex(result).render_code_spans();
                    }
                    _ => {}
                }

            }

            index += 1;
        }

        // there're no inline element in the content
        InlineNode::Raw(render_backslash_escapes(&content)).render_code_spans()
    }

    pub fn render_code_spans(self) -> Self {
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
                            let code_span_code = if code_span_end_index - index > 4 &&
                            content[index + 2] == ' ' as u16 && content[code_span_end_index - 1] == ' ' as u16 {
                                content[index + 3..code_span_end_index - 1].to_vec()
                            } else {
                                content[index + 2..code_span_end_index].to_vec()
                            };

                            complex_contents.push(Box::new(InlineNode::CodeSpan(code_span_code)));
                        }

                        last_index = code_span_end_index + 2;
                        index = code_span_end_index + 2;
                        continue;
                    }

                    // when `[[math]]` macros and code spans messed up really badly,
                    // a code_span_marker_begin dies and its corresponding code_span_marker_end survives
                    else if is_code_span_marker_end(&content, index) {
                        complex_contents.push(Box::new(InlineNode::Raw(undo_code_span_escapes(&content[last_index..index + 2]))));
                        last_index = index + 2;
                        index += 2;
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
            InlineNode::CodeSpan(_) => self,
            InlineNode::Footnote(_) => self
        }
    }

}

pub fn escape_code_spans(content: &[u16]) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len() * 5 / 4);
    let mut index = 0;

    while index < content.len() {

        match is_code_span(content, index) {
            Bool::True(end) => {
                result.push(INLINE_CODE_SPAN_MARKER1);
                result.push(INLINE_CODE_SPAN_MARKER2);
                let code_span_code = undo_backslash_escapes(&content[index..end + 1]);
                let backtick_string_size = count_code_span_start(content, index);

                for c in code_span_code[backtick_string_size..code_span_code.len() - backtick_string_size].iter() {
                    result.push(*c);
                }

                result.push(INLINE_CODE_SPAN_MARKER3);
                result.push(INLINE_CODE_SPAN_MARKER4);
                index = end + 1;
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
    content[index] == INLINE_CODE_SPAN_MARKER1 && index + 1 < content.len() && content[index + 1] == INLINE_CODE_SPAN_MARKER2
}

pub fn is_code_span_marker_end(content: &[u16], index: usize) -> bool {
    content[index] == INLINE_CODE_SPAN_MARKER3 && index + 1 < content.len() && content[index + 1] == INLINE_CODE_SPAN_MARKER4
}

pub fn get_code_span_marker_end_index(content: &[u16], mut index: usize) -> usize {

    while content[index] != INLINE_CODE_SPAN_MARKER3 || content[index + 1] != INLINE_CODE_SPAN_MARKER4 {
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
            "``", "`a`", "`code span`",
            "```", "`a`a", "code span",
            "*`*", "`*`*", "`*`*`", "",
            "*`*`*`", "*`*`*`*", "`*`*`*`*", "`*`*`*`*`"
        ];

        let cases = cases.iter().map(|s| into_v16(s)).collect::<Vec<Vec<u16>>>();

        for case in cases.iter() {
            assert_eq!(case, &undo_code_span_escapes(&escape_code_spans(case)));
        }

    }

}