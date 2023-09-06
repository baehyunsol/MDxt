use super::{
    InlineNode, DecorationType, InlineMacro, MediaType,
    INLINE_CODE_SPAN_MARKER1, INLINE_CODE_SPAN_MARKER2, INLINE_CODE_SPAN_MARKER3, INLINE_CODE_SPAN_MARKER4,
    render_auto_urls,
};
use super::auto_url::UrlOrNot;
use super::footnote::predicate::read_footnote;
use super::link::normalize_link_label;
use super::link::predicate::{
    read_direct_link, read_reference_link, read_shortcut_reference_link
};
use super::macros::{predicate::check_and_parse_macro_inline, tooltip::load_tooltip_message};
use super::math::escape_inside_math_blocks;
use super::predicate::*;
use crate::ast::doc_data::DocData;
use crate::escape::{render_backslash_escapes, undo_backslash_escapes};
use crate::render::render_option::RenderOption;
use crate::utils::{from_v32, get_bracket_end_index, into_v32};

impl InlineNode {

    pub fn parse_raw(
        &mut self,
        doc_data: &mut DocData,
        render_option: &RenderOption
    ) {

        // otherwise, it's already parsed
        if let InlineNode::Raw(content) = self {
            *self = Self::from_mdxt(&content, doc_data, render_option);
        }

    }

    pub fn from_mdxt(content: &[u32], doc_data: &mut DocData, render_option: &RenderOption) -> Self {
        // it prevents inline elements inside code spans from being rendered
        // code spans are rendered later
        let mut content = escape_code_spans(content);
        content = escape_inside_math_blocks(content);

        let mut index = 0;

        while index < content.len() {

            if is_code_span_marker_begin(&content, index) {
                index = get_code_span_marker_end_index(&content, index);
                continue;
            }

            // it continues if the current character is not a special character
            if content[index] < '*' as u32
                || ('*' as u32) < content[index] && content[index] < '[' as u32
                || ('`' as u32) < content[index] && content[index] < '~' as u32
                || content[index] > '~' as u32
            {
                index += 1;
                continue;
            }

            if let Bool::True(end) = is_bold_italic(&content, index) {
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
                    result.push(Box::new(Self::from_mdxt(&content[(end + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Bool::True(end) = is_deletion_subscript(&content, index) {
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
                    result.push(Box::new(Self::from_mdxt(&content[(end + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Bool::True(end) = is_italic(&content, index) {
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
                    result.push(Box::new(Self::from_mdxt(&content[(end + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Bool::True(end) = is_bold(&content, index) {
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
                    result.push(Box::new(Self::from_mdxt(&content[(end + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Bool::True(end) = is_deletion(&content, index) {
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
                    result.push(Box::new(Self::from_mdxt(&content[(end + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Bool::True(end) = is_underline(&content, index) {
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
                    result.push(Box::new(Self::from_mdxt(&content[(end + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Bool::True(end) = is_superscript(&content, index) {
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
                    result.push(Box::new(Self::from_mdxt(&content[(end + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Bool::True(end) = is_subscript(&content, index) {
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
                    result.push(Box::new(Self::from_mdxt(&content[(end + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Some((link_text, link_destination, last_index)) = read_direct_link(&content, index, &doc_data.link_references) {
                let mut result = vec![];
                let mut is_image = false;

                if index > 0 && content[index - 1] == '!' as u32 {
                    is_image = true;
                    index -= 1;
                }

                if index > 0 {
                    result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                }

                let handled_link = into_v32(&render_option.handle_link(&from_v32(&link_destination)));

                if is_image {
                    result.push(Box::new(InlineNode::Image {
                        media_type: MediaType::from_url(&handled_link, render_option.enable_youtube),
                        description: undo_code_span_escapes(&link_text),
                        address: handled_link
                    }));
                }

                else {
                    result.push(Box::new(InlineNode::Link {
                        text: Self::from_mdxt(&link_text, doc_data, render_option).to_vec(),
                        destination: handled_link
                    }));
                }

                if last_index + 1 < content.len() {
                    result.push(Box::new(Self::from_mdxt(&content[(last_index + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Some((link_text, link_label, last_index)) = read_reference_link(&content, index, &doc_data.link_references) {
                let mut result = vec![];
                let mut is_image = false;

                // the existence of the link reference was tested by the `read_reference_link` function
                // it clones to result in order to avoid the borrow checker
                let link_destination = doc_data.link_references.get(&normalize_link_label(&link_label)).unwrap().clone();

                if index > 0 && content[index - 1] == '!' as u32 {
                    is_image = true;
                    index -= 1;
                }

                if index > 0 {
                    result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                }

                let handled_link = into_v32(&render_option.handle_link(&from_v32(&link_destination)));

                if is_image {
                    result.push(Box::new(InlineNode::Image {
                        media_type: MediaType::from_url(&handled_link, render_option.enable_youtube),
                        description: undo_code_span_escapes(&link_text),
                        address: handled_link
                    }));
                }

                else {
                    result.push(Box::new(InlineNode::Link {
                        text: Self::from_mdxt(&link_text, doc_data, render_option).to_vec(),
                        destination: handled_link
                    }));
                }

                if last_index + 1 < content.len() {
                    result.push(Box::new(Self::from_mdxt(&content[(last_index + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Some(footnote_index) = read_footnote(&content, index, &doc_data.footnote_references) {
                let bracket_end_index = get_bracket_end_index(&content, index).unwrap();
                let footnote_label = normalize_link_label(&content[index + 1..bracket_end_index]);
                let mut result = vec![];

                let inverse_index = doc_data.add_footnote_inverse_index(&footnote_label);

                if index > 0 {
                    result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                }

                if render_option.footnote_tooltip {
                    result.push(Box::new(InlineNode::Decoration {
                        deco_type: DecorationType::Macro(InlineMacro::Tooltip {
                            label: footnote_label[1..].to_vec(),
                            message: load_tooltip_message(&footnote_label[1..], doc_data, render_option),
                            index: doc_data.add_tooltip()
                        }),
                        content: vec![Box::new(InlineNode::Footnote((footnote_index, inverse_index, footnote_label)))]
                    }));
                }

                else {
                    result.push(Box::new(InlineNode::Footnote((footnote_index, inverse_index, footnote_label))));
                }

                if bracket_end_index + 1 < content.len() {
                    result.push(Box::new(Self::from_mdxt(&content[(bracket_end_index + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Some((link_text, last_index)) = read_shortcut_reference_link(&content, index, &doc_data.link_references) {
                let mut result = vec![];
                let mut is_image = false;

                // the existence of the link reference was tested by the `read_reference_link` function
                // it clones to result in order to avoid the borrow checker
                let link_destination = doc_data.link_references.get(&normalize_link_label(&link_text)).unwrap().clone();

                if index > 0 && content[index - 1] == '!' as u32 {
                    is_image = true;
                    index -= 1;
                }

                if index > 0 {
                    result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                }

                let handled_link = into_v32(&render_option.handle_link(&from_v32(&link_destination)));

                if is_image {
                    result.push(Box::new(InlineNode::Image {
                        media_type: MediaType::from_url(&handled_link, render_option.enable_youtube),
                        description: undo_code_span_escapes(&link_text),
                        address: handled_link
                    }));
                }

                else {
                    result.push(Box::new(InlineNode::Link {
                        text: Self::from_mdxt(&link_text, doc_data, render_option).to_vec(),
                        destination: handled_link
                    }));
                }

                if last_index + 1 < content.len() {
                    result.push(Box::new(Self::from_mdxt(&content[(last_index + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            if let Some((parsed, last_index)) = check_and_parse_macro_inline(&content, index, doc_data, render_option) {
                let mut result = vec![];

                if index > 0 {
                    result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&content[0..index]))));
                }

                result.push(Box::new(parsed));

                if last_index + 1 < content.len() {
                    result.push(Box::new(Self::from_mdxt(&content[(last_index + 1)..], doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            }

            index += 1;
        }

        // TODO: How about toggling auto_urls?
        let auto_url_rendered = match render_auto_urls(&content) {
            UrlOrNot::NoUrl(result) => {
                #[cfg(test)] assert_eq!(content, result);

                result
            },
            UrlOrNot::HasUrl(prefix, url, suffix) => {
                #[cfg(test)] assert_eq!(
                    vec![prefix.clone(), url.clone(), suffix.clone()].concat(),
                    content.clone(),
                );

                let mut result = vec![];

                if !prefix.is_empty() {
                    result.push(Box::new(InlineNode::Raw(render_backslash_escapes(&prefix))));
                }

                result.push(Box::new(InlineNode::Link {
                    text: vec![Box::new(InlineNode::Raw(url.clone()))],
                    destination: url,
                }));

                if !suffix.is_empty() {
                    result.push(Box::new(Self::from_mdxt(&suffix, doc_data, render_option)));
                }

                return InlineNode::Complex(result).render_code_spans();
            },
        };

        // there're no inline element in the content
        InlineNode::Raw(render_backslash_escapes(&auto_url_rendered)).render_code_spans()
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
                            content[index + 2] == ' ' as u32 && content[code_span_end_index - 1] == ' ' as u32 {
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

                if complex_contents.is_empty() {
                    InlineNode::Raw(content)
                }

                else {

                    if content.len() > last_index {
                        complex_contents.push(Box::new(InlineNode::Raw(content[last_index..].to_vec())));
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
            InlineNode::Image { .. } => self,
            InlineNode::CodeSpan(_) => self,
            InlineNode::Footnote(_) => self
        }
    }

}

pub fn escape_code_spans(content: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(content.len() * 5 / 4);
    let mut index = 0;

    while index < content.len() {

        // "*a*b`a``a` `a``a`"
        if is_code_span_marker_begin(content, index) {
            let end_index = get_code_span_marker_end_index(content, index);

            while index <= end_index {
                result.push(content[index]);
                index += 1;
            }

            continue;
        }

        match is_code_span(content, index) {
            Bool::True(end) => {
                result.push(INLINE_CODE_SPAN_MARKER1);
                result.push(INLINE_CODE_SPAN_MARKER2);
                let code_span_code = undo_backslash_escapes(&content[index..end + 1]);
                let backtick_string_size = count_code_span_start(content, index);

                for c in code_span_code[backtick_string_size..(code_span_code.len() - backtick_string_size)].iter() {
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

pub fn undo_code_span_escapes(content: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        if is_code_span_marker_begin(content, index) || is_code_span_marker_end(content, index) {
            result.push('`' as u32);
            index += 1;
        }

        else {
            result.push(content[index]);
        }

        index += 1;
    }

    result
}

pub fn is_code_span_marker_begin(content: &[u32], index: usize) -> bool {
    content.get(index) == Some(&INLINE_CODE_SPAN_MARKER1)
    && content.get(index + 1) == Some(&INLINE_CODE_SPAN_MARKER2)
}

pub fn is_code_span_marker_end(content: &[u32], index: usize) -> bool {
    content.get(index) == Some(&INLINE_CODE_SPAN_MARKER3)
    && content.get(index + 1) == Some(&INLINE_CODE_SPAN_MARKER4)
}

// it doesn't check boundary because it assumes that `content` is always valid
// it assumes that it's called after `get_code_span_marker_begin_index` returned something
pub fn get_code_span_marker_end_index(content: &[u32], mut index: usize) -> usize {

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
        use crate::utils::into_v32;

        let cases = vec![
            "``", "`a`", "`code span`",
            "```", "`a`a", "code span",
            "*`*", "`*`*", "`*`*`", "",
            "*`*`*`", "*`*`*`*", "`*`*`*`*", "`*`*`*`*`"
        ];

        let cases = cases.iter().map(|s| into_v32(s)).collect::<Vec<Vec<u32>>>();

        for case in cases.iter() {
            assert_eq!(case, &undo_code_span_escapes(&escape_code_spans(case)));
        }

    }

}