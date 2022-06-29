use super::parse::{get_code_span_marker_end_index, is_code_span_marker_begin};

// it's always guaranteed that `index < content.len()`
// if the decorator is multi characters long, start_index points the first character of the starting deco, and end_index points the last one. 

pub enum Bool {
    False,
    True(usize)
}

pub fn is_code_span(content: &[u16], index: usize) -> Bool {

    if !is_code_span_start_or_end(content, index) {
        return Bool::False;
    }

    let mut end_index = index + 1;

    while end_index < content.len() {

        if is_code_span_start_or_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index += 1;
    }

    Bool::False
}

fn is_code_span_start_or_end(content: &[u16], index: usize) -> bool {    
    content[index] == '`' as u16 && (index == 0 || content[index - 1] != '`' as u16) && (index == content.len() - 1 || content[index + 1] != '`' as u16)
}

pub fn is_italic(content: &[u16], index: usize) -> Bool {

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

fn is_italic_start(content: &[u16], index: usize) -> bool {
    content[index] == '*' as u16 && index + 1 < content.len() && content[index + 1] != ' ' as u16 && content[index + 1] != '*' as u16 && (index == 0 || content[index - 1] != '*' as u16)
}

fn is_italic_end(content: &[u16], index: usize) -> bool {
    content[index] == '*' as u16 && index > 0 && content[index - 1] != ' ' as u16 && content[index - 1] != '*' as u16 && (index + 1 == content.len() || content[index + 1] != '*' as u16)
}

pub fn is_bold(content: &[u16], index: usize) -> Bool {

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

fn is_bold_start(content: &[u16], index: usize) -> bool {
    content[index] == '*' as u16 && index + 2 < content.len() && content[index + 1] == '*' as u16 && content[index + 2] != ' ' as u16 && content[index + 2] != '*' as u16
}

fn is_bold_end(content: &[u16], index: usize) -> bool {
    content[index] == '*' as u16 && index > 1 && content[index - 1] == '*' as u16 && content[index - 2] != ' ' as u16 && content[index - 2] != '*' as u16
}

pub fn is_bold_italic(content: &[u16], index: usize) -> Bool {

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

fn is_bold_italic_start(content: &[u16], index: usize) -> bool {
    content[index] == '*' as u16 && index + 3 < content.len() && content[index + 1] == '*' as u16 && content[index + 2] == '*' as u16 && content[index + 3] != ' ' as u16 && content[index + 3] != '*' as u16
}

fn is_bold_italic_end(content: &[u16], index: usize) -> bool {
    content[index] == '*' as u16 && index > 2 && content[index - 1] == '*' as u16 && content[index - 2] == '*' as u16 && content[index - 3] != ' ' as u16 && content[index - 3] != '*' as u16
}

pub fn is_deletion(content: &[u16], index: usize) -> Bool {

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

fn is_deletion_start(content: &[u16], index: usize) -> bool {
    content[index] == '~' as u16 && index + 2 < content.len() && content[index + 1] == '~' as u16 && content[index + 2] != ' ' as u16 && content[index + 2] != '_' as u16
}

fn is_deletion_end(content: &[u16], index: usize) -> bool {
    content[index] == '~' as u16 && index > 1 && content[index - 1] == '~' as u16 && content[index - 2] != ' ' as u16 && content[index - 2] != '_' as u16
}

pub fn is_underline(content: &[u16], index: usize) -> Bool {

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

fn is_underline_start(content: &[u16], index: usize) -> bool {
    content[index] == '~' as u16 && index + 2 < content.len() && content[index + 1] == '_' as u16 && content[index + 2] != ' ' as u16
}

fn is_underline_end(content: &[u16], index: usize) -> bool {
    content[index] == '~' as u16 && index > 1 && content[index - 1] == '_' as u16 && content[index - 2] != ' ' as u16
}

pub fn is_superscript(content: &[u16], index: usize) -> Bool {

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

fn is_superscript_start(content: &[u16], index: usize) -> bool {
    content[index] == '^' as u16 && index + 1 < content.len() && content[index + 1] != '^' as u16 && content[index + 1] != ' ' as u16
}

fn is_superscript_end(content: &[u16], index: usize) -> bool {
    content[index] == '^' as u16 && index > 0 && content[index - 1] != '^' as u16 && content[index - 1] != ' ' as u16
}

pub fn is_subscript(content: &[u16], index: usize) -> Bool {

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

fn is_subscript_start(content: &[u16], index: usize) -> bool {
    content[index] == '~' as u16 && index + 1 < content.len() && content[index + 1] != '~' as u16 && content[index + 1] != ' ' as u16 && (index == 0 || content[index - 1] != '~' as u16)
}

fn is_subscript_end(content: &[u16], index: usize) -> bool {
    content[index] == '~' as u16 && index > 0 && content[index - 1] != '~' as u16 && content[index - 1] != ' ' as u16 && (index == content.len() - 1 || content[index + 1] != '~' as u16)
}

pub fn is_deletion_subscript(content: &[u16], index: usize) -> Bool {

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

fn is_deletion_subscript_start(content: &[u16], index: usize) -> bool {
    content[index] == '~' as u16 && index + 3 < content.len() && content[index + 1] == '~' as u16 && content[index + 2] == '~' as u16 && content[index + 3] != ' ' as u16 && content[index + 3] != '~' as u16 && content[index + 3] != '_' as u16
}

fn is_deletion_subscript_end(content: &[u16], index: usize) -> bool {
    content[index] == '~' as u16 && index > 2 && content[index - 1] == '~' as u16 && content[index - 2] == '~' as u16 && content[index - 3] != ' ' as u16 && content[index - 3] != '~' as u16 && content[index - 3] != '_' as u16
}