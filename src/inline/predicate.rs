use super::parse::{get_code_span_marker_end_index, is_code_span_marker_begin};

// it's always guaranteed that `index < content.len()`
// `is_XX_start` searches from `0` to `content.len() - 1`
// `is_XX_end` searches from `content.len() - 1` to `0`

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