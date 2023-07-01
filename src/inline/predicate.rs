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

    while content.get(index) == Some(&('`' as u32)) {
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

macro_rules! inline_predicate {
    ($f_name: ident, $pred_start: ident, $pred_end: ident, $decorator_length: expr, $decorator_offset: expr) => (
        pub fn $f_name(content: &[u32], index: usize) -> Bool {

            if !$pred_start(content, index) {
                return Bool::False;
            }

            let mut end_index = index + $decorator_length + $decorator_offset;

            while end_index < content.len() {

                if is_code_span_marker_begin(content, end_index - $decorator_offset) {
                    end_index = get_code_span_marker_end_index(content, end_index);
                    continue;
                }

                if $pred_end(content, end_index) {
                    return Bool::True(end_index);
                }

                end_index += 1;
            }

            Bool::False
        }
    )
}

// 1 means italic is `*`, which is a 1-character long decorator
inline_predicate!(is_italic, is_italic_start, is_italic_end, 1, 0);

// 2 means bold is `**`, which is a 2-characters long decorator
inline_predicate!(is_bold, is_bold_start, is_bold_end, 2, 0);

// 3 means bold is `***`, which is a 3-characters long decorator
// in order to properly deal with `****`, it starts searching `end_index` with offset 1
inline_predicate!(is_bold_italic, is_bold_italic_start, is_bold_italic_end, 3, 1);

inline_predicate!(is_underline, is_underline_start, is_underline_end, 2, 1);
inline_predicate!(is_superscript, is_superscript_start, is_superscript_end, 1, 0);
inline_predicate!(is_subscript, is_subscript_start, is_subscript_end, 1, 0);
inline_predicate!(is_deletion_subscript, is_deletion_subscript_start, is_deletion_subscript_end, 3, 1);

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

    // in order not to parse `~~~`, it has to be 3 instead of 2
    let mut end_index = index + 3;

    while end_index < content.len() {

        // since we've added 3 instead of 2, we should start searching at `end_index - 1`
        if is_code_span_marker_begin(content, end_index - 1) {
            end_index = get_code_span_marker_end_index(content, end_index);
            continue;
        }

        // this function cannot be defined with macro due to this branch
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
