// https://github.github.com/gfm/#links

// a valid link following a `!` is a valid image

use super::normalize_link_label;
use crate::utils::{drop_while, get_bracket_end_index, get_parenthesis_end_index};
use std::collections::HashMap;

// [foo](address)
pub fn read_direct_link(
    content: &[u16],
    index: usize,
    link_references: &HashMap<Vec<u16>, Vec<u16>>
) -> Option<(Vec<u16>, Vec<u16>, usize)> {  // Option<(link_text, link_destination, last_index)>

    if content[index] == '[' as u16 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => if bracket_end_index + 1 >= content.len()
                || content[bracket_end_index + 1] != '(' as u16
            {
                None
            } else {

                match get_parenthesis_end_index(content, bracket_end_index + 1) {
                    Some(parenthesis_end_index) => {
                        let link_text = &content[index + 1..bracket_end_index];
                        let link_destination = &content[bracket_end_index + 2..parenthesis_end_index];

                        if is_valid_link_text(link_text, link_references) {
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
pub fn read_reference_link(content: &[u16], index: usize, link_references: &HashMap<Vec<u16>, Vec<u16>>) -> Option<(Vec<u16>, Vec<u16>, usize)> {  // Option<(link_text, link_label, last_index)>

    if content[index] == '[' as u16 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => if bracket_end_index + 1 >= content.len()
                || content[bracket_end_index + 1] != '[' as u16
            {
                None
            } else {

                match get_bracket_end_index(content, bracket_end_index + 1) {
                    Some(second_bracket_end_index) => {
                        let link_text = &content[index + 1..bracket_end_index];
                        let mut link_label = &content[bracket_end_index + 2..second_bracket_end_index];

                        if second_bracket_end_index == bracket_end_index + 2 {  // collapsed reference link
                            link_label = link_text;
                        }

                        if is_valid_link_text(link_text, link_references) && link_references.contains_key(&normalize_link_label(link_label)) && is_valid_link_label(link_label) {
                            Some((link_text.to_vec(), link_label.to_vec(), second_bracket_end_index))
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

// [foo]
// in case `foo` is a valid reference and `bar` is not, `[foo][bar]` is an invalid reference.
// if parenthesises or brackets follow `[foo]`, it's not regarded as a shortcut reference whether or not the references are valid.
pub fn read_shortcut_reference_link(content: &[u16], index: usize, link_references: &HashMap<Vec<u16>, Vec<u16>>) -> Option<(Vec<u16>, usize)> {  // Option<(link_text, last_index)>

    if content[index] == '[' as u16 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => {

                // if a shortcut reference is followed by a balanced [] or (), the reference is invalid
                if bracket_end_index + 1 == content.len()
                    || (content[bracket_end_index + 1] != '(' as u16
                    && content[bracket_end_index + 1] != '[' as u16)
                {
                    //
                }

                else if content[bracket_end_index + 1] == '(' as u16 {

                    match get_parenthesis_end_index(content, bracket_end_index + 1) {
                        Some(_) => {return None;}
                        _ => {},
                    }

                }

                else {  // content[bracket_end_index + 1] == '[' as u16

                    match get_bracket_end_index(content, bracket_end_index + 1) {
                        Some(_) => {return None;}
                        _ => {},
                    }

                }

                let link_text = &content[index + 1..bracket_end_index];

                if link_references.contains_key(&normalize_link_label(link_text))
                    && is_valid_link_label(link_text)
                    && is_valid_link_text(link_text, link_references)
                {
                    Some((link_text.to_vec(), bracket_end_index))
                }

                else {
                    None
                }

            }

            None => None
        }

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

fn is_valid_link_text(content: &[u16], link_references: &HashMap<Vec<u16>, Vec<u16>>) -> bool {

    !contains_link(content, link_references)  // it makes sure that the links are not nested
    && (content.len() == 0
    || if content[0] == '[' as u16
        && content[content.len() - 1] == ']' as u16
    {

        // [...] -> link
        // [[...]] -> macro
        // [[[...]]] -> macro in a link
        content.len() > 4 && content[1] == '[' as u16 && content[content.len() - 2] == ']' as u16
    } else {
        true
    })
}

pub fn is_valid_link_label(content: &[u16]) -> bool {
    true
}

pub fn is_valid_link_destination(content: &[u16]) -> bool {
    content.iter().all(is_valid_url_character)
}

fn contains_link(content: &[u16], link_references: &HashMap<Vec<u16>, Vec<u16>>) -> bool {

    match content.iter().position(|c| *c == '[' as u16) {
        // this function is used to remove nested links
        // so images are fine
        Some(index)
            if index == 0 || content[index - 1] != '!' as u16 =>
        {
            read_direct_link(content, index, link_references).is_some()
            || read_reference_link(content, index, link_references).is_some()
            || read_shortcut_reference_link(content, index, link_references).is_some()
            || contains_link(&content[index + 1..content.len()], link_references)
        },
        _ => false,
    }

}

fn is_valid_url_character(character: &u16) -> bool {
    '-' as u16 <= *character && *character <= ';' as u16
    || 'a' as u16 <= *character && *character <= 'z' as u16
    || '?' as u16 <= *character && *character <= 'Z' as u16
    || '가' as u16 <= *character && *character <= '힣' as u16  // korean
    || 'ㄱ' as u16 <= *character && *character <= 'ㅣ' as u16  // korean
    || 'ぁ' as u16 <= *character && *character <= 'ヺ' as u16  // japanese
    || '!' as u16 == *character || '=' as u16 == *character
    || '+' as u16 == *character || '&' as u16 == *character
    || '%' as u16 == *character || '_' as u16 == *character
    || '#' as u16 == *character || '$' as u16 == *character
    || '+' as u16 == *character
}
