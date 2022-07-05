use super::Footnote;
use crate::inline::link::normalize_link;
use crate::utils::get_bracket_end_index;
use std::collections::HashMap;

pub fn is_valid_footnote_label(content: &[u16]) -> bool {
    content[0] == '^' as u16 && content[1..content.len()].iter().all(is_valid_footnote_character)
}

pub fn read_footnote(content: &[u16], index: usize, footnote_references: &HashMap<Vec<u16>, Footnote>) -> Option<usize> {  // footnote_index

    if content[index] == '[' as u16 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => {
                let footnote_label = normalize_link(&content[index + 1..bracket_end_index]);

                match footnote_references.get(&footnote_label) {
                    Some(Footnote {index, ..}) => Some(*index),
                    None => None
                }

            },
            None => None
        }

    }

    else {
        None
    }

}

fn is_valid_footnote_character(c: &u16) -> bool {

    '0' as u16 <= *c && *c <= '9' as u16 ||
    'a' as u16 <= *c && *c <= 'z' as u16 ||
    'A' as u16 <= *c && *c <= 'Z' as u16 ||
    '가' as u16 <= *c && *c <= '힣' as u16 ||  // korean
    'ㄱ' as u16 <= *c && *c <= 'ㅣ' as u16 ||  // korean
    'ぁ' as u16 <= *c && *c <= 'ヺ' as u16  // japanese
}