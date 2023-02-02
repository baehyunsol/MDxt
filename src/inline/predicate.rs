use crate::escape::BACKSLASH_ESCAPE_OFFSET;
use super::parse::{get_code_span_marker_end_index, is_code_span_marker_begin};

// it's always guaranteed that `index < content.len()`
// if the decorator is multi characters long, start_index points the first character of the starting deco, and end_index points the last one. 

#[derive(Debug, PartialEq)]
pub enum Bool {
    False,
    True(usize)  // end_index
}

pub fn is_code_span(content: &[u32], index: usize) -> Bool {

    if content[index] != '`' as u32 {
        return Bool::False;
    }

    // it should not count a single span multiple times
    // with out this branch, it would return true 5 times for '`````a`````'
    else if index > 0 && content[index - 1] == '`' as u32 {
        return Bool::False;
    }

    let backtick_string_size = count_code_span_start(content, index);
    let mut end_index = index + backtick_string_size;

    while end_index < content.len() {
        let (end_backtick_string_size, code_span_end_index) = count_code_span_end(content, end_index);

        if end_backtick_string_size == backtick_string_size {
            return Bool::True(code_span_end_index)
        }

        end_index += 1;
    }

    Bool::False
}

pub fn count_code_span_start(content: &[u32], mut index: usize) -> usize {
    let mut result = 0;

    while index != content.len() && content[index] == '`' as u32 {
        index += 1;
        result += 1;
    }

    result
}

fn count_code_span_end(content: &[u32], mut index: usize) -> (usize, usize) {  // (backtick_string_size, end_index)

    // '``' is not a code span, but '`\`' is.
    if index > 0 && content[index - 1] == '`' as u32 && content[index] != '`' as u32 + BACKSLASH_ESCAPE_OFFSET {
        return (0, 0);
    }

    let mut result = 0;

    loop {

        if index == content.len() || content[index] != '`' as u32 && content[index] != '`' as u32 + BACKSLASH_ESCAPE_OFFSET {
            break;
        }

        index += 1;
        result += 1;
    }

    (result, index - 1)
}

pub fn is_italic(content: &[u32], index: usize) -> Bool {

    if !is_italic_start(content, index) {
        return Bool::False;
    }

    let mut end_index = index + 1;

    while end_index < content.len() {

        if is_code_span_marker_begin(content, end_index) {
            end_index = get_code_span_marker_end_index(content, end_index);
            continue;
        }

        if is_italic_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index += 1;
    }

    Bool::False
}

fn is_italic_start(content: &[u32], index: usize) -> bool {
    content[index] == '*' as u32
    && index + 1 < content.len()
    && content[index + 1] != ' ' as u32
    && content[index + 1] != '*' as u32
    && (index == 0 || content[index - 1] != '*' as u32)
}

fn is_italic_end(content: &[u32], index: usize) -> bool {
    content[index] == '*' as u32
    && index > 0
    && content[index - 1] != ' ' as u32
    && content[index - 1] != '*' as u32
    && (index + 1 == content.len() || content[index + 1] != '*' as u32)
}

pub fn is_bold(content: &[u32], index: usize) -> Bool {

    if !is_bold_start(content, index) {
        return Bool::False;
    }

    let mut end_index = index + 2;

    while end_index < content.len() {

        if is_code_span_marker_begin(content, end_index) {
            end_index = get_code_span_marker_end_index(content, end_index);
            continue;
        }

        if is_bold_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index += 1;
    }

    Bool::False
}

fn is_bold_start(content: &[u32], index: usize) -> bool {
    content[index] == '*' as u32
    && index + 2 < content.len()
    && content[index + 1] == '*' as u32
    && content[index + 2] != ' ' as u32
    && content[index + 2] != '*' as u32
}

fn is_bold_end(content: &[u32], index: usize) -> bool {
    content[index] == '*' as u32
    && index > 1
    && content[index - 1] == '*' as u32
    && content[index - 2] != ' ' as u32
    && content[index - 2] != '*' as u32
}

pub fn is_bold_italic(content: &[u32], index: usize) -> Bool {

    if !is_bold_italic_start(content, index) {
        return Bool::False;
    }

    let mut end_index = index + 4;

    while end_index < content.len() {

        if is_code_span_marker_begin(content, end_index) {
            end_index = get_code_span_marker_end_index(content, end_index);
            continue;
        }

        if is_bold_italic_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index += 1;
    }

    Bool::False
}

fn is_bold_italic_start(content: &[u32], index: usize) -> bool {
    content[index] == '*' as u32
    && index + 3 < content.len()
    && content[index + 1] == '*' as u32
    && content[index + 2] == '*' as u32
    && content[index + 3] != ' ' as u32
    && content[index + 3] != '*' as u32
}

fn is_bold_italic_end(content: &[u32], index: usize) -> bool {
    content[index] == '*' as u32
    && index > 2
    && content[index - 1] == '*' as u32
    && content[index - 2] == '*' as u32
    && content[index - 3] != ' ' as u32
    && content[index - 3] != '*' as u32
}

pub fn is_deletion(content: &[u32], index: usize) -> Bool {

    if !is_deletion_start(content, index) {
        return Bool::False;
    }

    let mut end_index = index + 3;

    while end_index < content.len() {

        if is_code_span_marker_begin(content, end_index) {
            end_index = get_code_span_marker_end_index(content, end_index);
            continue;
        }

        if is_deletion_end(content, end_index) && end_index - index > 3 {
            return Bool::True(end_index);
        }

        end_index += 1;
    }

    Bool::False
}

fn is_deletion_start(content: &[u32], index: usize) -> bool {
    content[index] == '~' as u32
    && index + 2 < content.len()
    && content[index + 1] == '~' as u32
    && content[index + 2] != ' ' as u32
    && content[index + 2] != '_' as u32
}

fn is_deletion_end(content: &[u32], index: usize) -> bool {
    content[index] == '~' as u32
    && index > 1
    && content[index - 1] == '~' as u32
    && content[index - 2] != ' ' as u32
    && content[index - 2] != '_' as u32
}

pub fn is_underline(content: &[u32], index: usize) -> Bool {

    if !is_underline_start(content, index) {
        return Bool::False;
    }

    let mut end_index = index + 3;

    while end_index < content.len() {

        if is_code_span_marker_begin(content, end_index) {
            end_index = get_code_span_marker_end_index(content, end_index);
            continue;
        }

        if is_underline_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index += 1;
    }

    Bool::False
}

fn is_underline_start(content: &[u32], index: usize) -> bool {
    content[index] == '~' as u32
    && index + 2 < content.len()
    && content[index + 1] == '_' as u32
    && content[index + 2] != ' ' as u32
}

fn is_underline_end(content: &[u32], index: usize) -> bool {
    content[index] == '~' as u32
    && index > 1
    && content[index - 1] == '_' as u32
    && content[index - 2] != ' ' as u32
}

pub fn is_superscript(content: &[u32], index: usize) -> Bool {

    if !is_superscript_start(content, index) {
        return Bool::False;
    }

    let mut end_index = index + 1;

    while end_index < content.len() {

        if is_code_span_marker_begin(content, end_index) {
            end_index = get_code_span_marker_end_index(content, end_index);
            continue;
        }

        if is_superscript_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index += 1;
    }

    Bool::False
}

fn is_superscript_start(content: &[u32], index: usize) -> bool {
    content[index] == '^' as u32
    && index + 1 < content.len()
    && content[index + 1] != '^' as u32
    && content[index + 1] != ' ' as u32
}

fn is_superscript_end(content: &[u32], index: usize) -> bool {
    content[index] == '^' as u32
    && index > 0
    && content[index - 1] != '^' as u32
    && content[index - 1] != ' ' as u32
}

pub fn is_subscript(content: &[u32], index: usize) -> Bool {

    if !is_subscript_start(content, index) {
        return Bool::False;
    }

    let mut end_index = index + 1;

    while end_index < content.len() {

        if is_code_span_marker_begin(content, end_index) {
            end_index = get_code_span_marker_end_index(content, end_index);
            continue;
        }

        if is_subscript_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index += 1;
    }

    Bool::False
}

fn is_subscript_start(content: &[u32], index: usize) -> bool {
    content[index] == '~' as u32
    && index + 1 < content.len()
    && content[index + 1] != '~' as u32
    && content[index + 1] != ' ' as u32
    && (index == 0 || content[index - 1] != '~' as u32)
}

fn is_subscript_end(content: &[u32], index: usize) -> bool {
    content[index] == '~' as u32
    && index > 0
    && content[index - 1] != '~' as u32
    && content[index - 1] != ' ' as u32
    && (index == content.len() - 1 || content[index + 1] != '~' as u32)
}

pub fn is_deletion_subscript(content: &[u32], index: usize) -> Bool {

    if !is_deletion_subscript_start(content, index) {
        return Bool::False;
    }

    let mut end_index = index + 4;

    while end_index < content.len() {

        if is_code_span_marker_begin(content, end_index) {
            end_index = get_code_span_marker_end_index(content, end_index);
            continue;
        }

        if is_deletion_subscript_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index += 1;
    }

    Bool::False
}

fn is_deletion_subscript_start(content: &[u32], index: usize) -> bool {
    content[index] == '~' as u32
    && index + 3 < content.len()
    && content[index + 1] == '~' as u32
    && content[index + 2] == '~' as u32
    && content[index + 3] != ' ' as u32
    && content[index + 3] != '~' as u32
    && content[index + 3] != '_' as u32
}

fn is_deletion_subscript_end(content: &[u32], index: usize) -> bool {
    content[index] == '~' as u32
    && index > 2
    && content[index - 1] == '~' as u32
    && content[index - 2] == '~' as u32
    && content[index - 3] != ' ' as u32
    && content[index - 3] != '~' as u32 && content[index - 3] != '_' as u32
}
