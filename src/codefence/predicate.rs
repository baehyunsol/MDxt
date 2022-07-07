use crate::utils::{into_v16, is_numeric, get_parenthesis_end_index};

pub fn is_valid_info_string(content: &[u16]) -> bool {
    content.iter().all(is_valid_info_string_character)
}

// a-z A-Z `,` `(` `)` ` ` `_` `.` `-`
fn is_valid_info_string_character(chr: &u16) -> bool {
    '0' as u16 <= *chr && *chr <= '9' as u16 ||
    'a' as u16 <= *chr && *chr <= 'z' as u16 ||
    'A' as u16 <= *chr && *chr <= 'Z' as u16 ||
    '(' as u16 == *chr || ')' as u16 == *chr ||
    ' ' as u16 == *chr || ',' as u16 == *chr ||
    '_' as u16 == *chr || '.' as u16 == *chr ||
    '-' as u16 == *chr
}

pub fn is_line_num(content: &[u16]) -> bool {

    if content.len() == 8 {
        content == into_v16("line_num")
    }

    else if content.len() > 10 {
        content[0..9] == into_v16("line_num(") && content[content.len() - 1] == ')' as u16 &&
        content[9..content.len() - 1].iter().all(is_numeric)
    }

    else {
        false
    }

}

pub fn is_highlight(content: &[u16]) -> bool {
    content.len() > 11 && content[0..10] == into_v16("highlight(") && content[content.len() - 1] == ')' as u16 &&
    content[10..content.len() - 1].iter().all(|c| is_numeric(c) || *c == ',' as u16)
}

// `rust, line_num` -> [`rust`, `line_num`]
// `rust, highlight(2, 3)` -> [`rust`, `highlight(2, 3)`]
pub fn parse_arguments(content: &[u16]) -> Vec<Vec<u16>> {

    let mut index = 0;
    let mut last_index = 0;
    let mut result = vec![];

    while index < content.len() {

        if content[index] == ',' as u16 {
            result.push(content[last_index..index].to_vec());
            last_index = index + 1;
        }

        else if content[index] == '('  as u16 {

            match get_parenthesis_end_index(content, index) {
                Some(n) => {
                    index = n;
                    continue;
                }
                _ => {}
            }

        }

        index += 1;
    }

    if last_index < content.len() {
        result.push(content[last_index..content.len()].to_vec());
    }

    result
}