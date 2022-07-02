use crate::link::{predicate::is_valid_link_label, normalize_link};
use crate::inline::InlineNode;
use crate::utils::get_bracket_end_index;
use std::collections::HashMap;

pub fn is_valid_footnote_label(content: &[u16]) -> bool {
    content[0] == '^' as u16 && is_valid_link_label(&content[1..content.len()])
}

pub fn read_footnote(content: &[u16], index: usize, footnote_references: &HashMap<Vec<u16>, (usize, InlineNode)>) -> Option<usize> {  // footnote_index

    if content[index] == '[' as u16 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => {
                let footnote_label = normalize_link(&content[index + 1..bracket_end_index]);

                match footnote_references.get(&footnote_label) {
                    Some((n, _)) => Some(*n),
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