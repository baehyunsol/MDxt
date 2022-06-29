// https://github.github.com/gfm/#links

// a valid link following a `!` is a valid image

use crate::utils::{get_bracket_end_index, get_parenthesis_end_index, drop_while};

// [foo](address)
pub fn read_direct_link(content: &[u16], index: usize) -> Option<(Vec<u16>, Vec<u16>, usize)> {  // Option<(link_text, link_destination, last_index)>

    if content[index] == '[' as u16 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => if bracket_end_index + 1 >= content.len() || content[bracket_end_index + 1] != '(' as u16 {
                None
            } else {

                match get_parenthesis_end_index(content, bracket_end_index + 1) {
                    Some(parenthesis_end_index) => {
                        let link_text = &content[index + 1..bracket_end_index];
                        let link_destination = &content[bracket_end_index + 2..parenthesis_end_index];

                        if is_valid_link_text(link_text) && is_valid_link_destination(link_destination) {
                            Some((link_text.to_vec(), link_destination.to_vec(), parenthesis_end_index))
                        }

                        else {
                            None
                        }

                    },
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

// [foo][bar]
// [foo][]
pub fn read_reference_link(content: &[u16], index: usize) -> Option<(Vec<u16>, Vec<u16>, usize)> {  // Option<(link_text, link_label, last_index)>

    if content[index] == '[' as u16 {
        todo!()
    }

    else {
        None
    }

}

// [foo]
// in case `foo` is a valid reference and `bar` is not, `[foo][bar]` is an invalid reference.
// if parenthesises or brackets follow `[foo]`, it's not regarded as a shortcut reference whether or not the references are valid.
pub fn read_shortcut_reference_link(content: &[u16], index: usize) -> Option<(Vec<u16>, usize)> {  // Option<(link_text, last_index)>

    if content[index] == '[' as u16 {
        todo!()
    }

    else {
        None
    }

}

// [foo]: address
// it assumes that the content is a valid link_reference_definition
pub fn read_link_reference(content: &[u16]) -> (Vec<u16>, Vec<u16>) {  // (link_label, link_destination)

    let bracket_end_index = get_bracket_end_index(content, 0).unwrap();
    let link_label = content[1..bracket_end_index].to_vec();
    let link_destination = drop_while(&content[bracket_end_index + 2..content.len()], ' ' as u16);

    (link_label, link_destination)
}

fn is_valid_link_text(content: &[u16]) -> bool {
    // it makes sure that the links are not nested
    !contains_link(content)

    // the syntax below is for macros, not links
    && (content[0] != '[' as u16 || content[content.len() - 1] != ']' as u16)
}

pub fn is_valid_link_label(content: &[u16]) -> bool {
    todo!()
}

pub fn is_valid_link_destination(content: &[u16]) -> bool {
    todo!()
}

fn contains_link(content: &[u16]) -> bool {

    match content.iter().position(|c| *c == '[' as u16) {
        // images are fine
        // that's because this function is used to remove nested links
        Some(index) if index == 0 || content[index - 1] != '!' as u16 => read_direct_link(content, index).is_some()
        || read_reference_link(content, index).is_some()
        || read_shortcut_reference_link(content, index).is_some()
        || contains_link(&content[index + 1..content.len()]),
        _ => false,
    }

}