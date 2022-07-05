use crate::utils::{into_v16, is_numeric};

pub fn is_valid_info_string(content: &[u16]) -> bool {
    content.iter().all(is_valid_info_string_character)
}

// a-z A-Z `,` `(` `)` ` ` `_`
fn is_valid_info_string_character(chr: &u16) -> bool {
    '0' as u16 <= *chr && *chr <= '9' as u16 ||
    'a' as u16 <= *chr && *chr <= 'z' as u16 ||
    'A' as u16 <= *chr && *chr <= 'Z' as u16 ||
    '(' as u16 == *chr || ')' as u16 == *chr ||
    ' ' as u16 == *chr || ',' as u16 == *chr ||
    '_' as u16 == *chr
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