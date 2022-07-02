use crate::link::predicate::is_valid_link_label;

pub fn is_valid_footnote_label(content: &[u16]) -> bool {
    content[0] == '^' as u16 && is_valid_link_label(&content[1..content.len()])
}