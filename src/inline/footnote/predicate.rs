use super::Footnote;
use crate::inline::link::normalize_link_label;
use crate::utils::get_bracket_end_index;
use std::collections::HashMap;

pub fn is_valid_footnote_label(content: &[u32]) -> bool {
    content[0] == '^' as u32 && content[1..].iter().all(is_valid_footnote_character)
}

pub fn read_footnote(content: &[u32], index: usize, footnote_references: &HashMap<Vec<u32>, Footnote>) -> Option<usize> {  // footnote_index

    if content[index] == '[' as u32 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => {
                let footnote_label = normalize_link_label(&content[index + 1..bracket_end_index]);

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

fn is_valid_footnote_character(c: &u32) -> bool {

    '0' as u32 <= *c && *c <= '9' as u32
    || 'a' as u32 <= *c && *c <= 'z' as u32
    || 'A' as u32 <= *c && *c <= 'Z' as u32
    || '가' as u32 <= *c && *c <= '힣' as u32  // Korean
    || 'ㄱ' as u32 <= *c && *c <= 'ㅣ' as u32  // Korean
    || 'ぁ' as u32 <= *c && *c <= 'ヺ' as u32  // Japanese
}