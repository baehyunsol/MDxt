use crate::utils::{get_bracket_end_index, lowercase_and_remove_spaces};

pub fn read_macro(content: &[u16], index: usize) -> Option<Vec<u16>> {

    if content[index] == '[' as u16 && index + 1 < content.len() && content[index + 1] == '[' as u16 {

        match get_bracket_end_index(content, index) {
            None => {return None;}
            Some(end_index1) => match get_bracket_end_index(content, index + 1) {
                Some(end_index2) if end_index2 + 1 == end_index1 && content[index + 2..end_index2].iter().all(is_valid_macro_character) => {
                    Some(lowercase_and_remove_spaces(&content[index + 2..end_index2].to_vec()))
                }
                _ => {return None;}
            }
        }

    }

    else {
        None
    }

}

fn is_valid_macro_character(chr: &u16) -> bool {
    '/' as u16 <= *chr && *chr <= '9' as u16 ||
    'a' as u16 <= *chr && *chr <= 'z' as u16 ||
    'A' as u16 <= *chr && *chr <= 'Z' as u16 ||
    ' ' as u16 == *chr ||
    '_' as u16 == *chr ||
    ',' as u16 == *chr ||
    '=' as u16 == *chr
}