use crate::utils::{get_parenthesis_end_index, into_v32, is_numeric};

pub fn is_valid_info_string(content: &[u32]) -> bool {
    content.iter().all(is_valid_info_string_character)
}

// a-z A-Z `,` `(` `)` ` ` `_` `.` `-`
fn is_valid_info_string_character(chr: &u32) -> bool {
    '0' as u32 <= *chr && *chr <= '9' as u32
    || 'a' as u32 <= *chr && *chr <= 'z' as u32
    || 'A' as u32 <= *chr && *chr <= 'Z' as u32
    || '(' as u32 == *chr || ')' as u32 == *chr
    || ' ' as u32 == *chr || ',' as u32 == *chr
    || '_' as u32 == *chr || '.' as u32 == *chr
    || '-' as u32 == *chr
}

// TODO: allow whitespaces
pub fn is_line_num(content: &[u32]) -> bool {

    if content.len() == 8 {
        content == into_v32("line_num")
    }

    else if content.len() > 10 {
        content[0..9] == into_v32("line_num(")
        && content.last() == Some(&(')' as u32))
        && content[9..content.len() - 1].iter().all(is_numeric)
    }

    else {
        false
    }

}

// TODO: allow whitespaces
pub fn is_highlight(content: &[u32]) -> bool {
    content.len() > 11
    && content[0..10] == into_v32("highlight(")
    && content.last() == Some(&(')' as u32))
    && content[10..content.len() - 1].iter().all(|c| is_numeric(c) || *c == ',' as u32)
}

// TODO: allow whitespaces
pub fn is_html_attribute(content: &[u32]) -> bool {
    content.len() > 4
    && (
        content[0..3] == into_v32("id(")
        || content.len() > 7
        && content[0..6] == into_v32("class(")
    ) && content.last() == Some(&(')' as u32))
}

// TODO: allow whitespaces
pub fn is_copy_button(content: &[u32]) -> bool {
    content == into_v32("copy_button")
    || content == into_v32("copy_button(true)")
    || content == into_v32("copy_button(false)")
}

// `rust, line_num` -> [`rust`, `line_num`]
// `rust, highlight(2, 3)` -> [`rust`, `highlight(2, 3)`]
pub fn parse_arguments(content: &[u32]) -> Vec<Vec<u32>> {

    let mut index = 0;
    let mut last_index = 0;
    let mut result = vec![];

    while index < content.len() {

        if content[index] == ',' as u32 {
            result.push(content[last_index..index].to_vec());
            last_index = index + 1;
        }

        else if content[index] == '('  as u32 {

            if let Some(n) = get_parenthesis_end_index(content, index) {
                index = n;
                continue;
            }

        }

        index += 1;
    }

    if last_index < content.len() {
        result.push(content[last_index..].to_vec());
    }

    result
}