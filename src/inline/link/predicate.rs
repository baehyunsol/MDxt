// https://github.github.com/gfm/#links

// a valid link following a `!` is a valid image
// [t][l], [l], [t](d) where t is link_text, l is link_label and d is link_destination

use super::normalize_link_label;
use crate::inline::parse::undo_code_span_escapes;
use crate::escape::{render_html_escapes, render_backslash_escapes};
use crate::utils::{drop_while, get_bracket_end_index, get_parenthesis_end_index};
use std::collections::HashMap;

// [foo](address)
pub fn read_direct_link(
    content: &[u32],
    index: usize,
    link_references: &HashMap<Vec<u32>, Vec<u32>>
) -> Option<(Vec<u32>, Vec<u32>, usize)> {  // Option<(link_text, link_destination, last_index)>

    if content[index] == '[' as u32 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => if bracket_end_index + 1 >= content.len()
                || content[bracket_end_index + 1] != '(' as u32
            {
                None
            } else {

                match get_parenthesis_end_index(content, bracket_end_index + 1) {
                    Some(parenthesis_end_index) => {
                        let link_text = &content[index + 1..bracket_end_index];

                        // TODO: `render_backslash_escapes` vs `undo_backslash_escapes`?
                        let link_destination = undo_code_span_escapes(
                            &render_backslash_escapes(
                                &render_html_escapes(&content[bracket_end_index + 2..parenthesis_end_index])
                            )
                        );

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
pub fn read_reference_link(content: &[u32], index: usize, link_references: &HashMap<Vec<u32>, Vec<u32>>) -> Option<(Vec<u32>, Vec<u32>, usize)> {  // Option<(link_text, link_label, last_index)>

    if content[index] == '[' as u32 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => if bracket_end_index + 1 >= content.len()
                || content[bracket_end_index + 1] != '[' as u32
            {
                None
            } else {

                match get_bracket_end_index(content, bracket_end_index + 1) {
                    Some(second_bracket_end_index) => {
                        let link_text = &content[index + 1..bracket_end_index];  // `foo` in `[foo][bar]`
                        let mut link_label = &content[bracket_end_index + 2..second_bracket_end_index];  // `bar` in `[foo][bar]`

                        if link_label.len() > 2 && link_label[0] == '[' as u32 {
                            // `[link][[br]]` is a shortcut reference link followed by a macro
                            match get_bracket_end_index(&link_label, 0) {
                                Some(i) if i == link_label.len() - 1 => {
                                    return None;
                                }
                                _ => {}
                            }

                        }

                        if second_bracket_end_index == bracket_end_index + 2 {  // `[foo][]`, collapsed reference link
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
pub fn read_shortcut_reference_link(content: &[u32], index: usize, link_references: &HashMap<Vec<u32>, Vec<u32>>) -> Option<(Vec<u32>, usize)> {  // Option<(link_text, last_index)>

    if content[index] == '[' as u32 {

        match get_bracket_end_index(content, index) {
            Some(bracket_end_index) => {

                // if a shortcut reference is followed by a balanced [] or (), the reference is invalid
                if bracket_end_index + 1 == content.len()
                    || (content[bracket_end_index + 1] != '(' as u32
                    && content[bracket_end_index + 1] != '[' as u32)
                {
                    //
                }

                else if content[bracket_end_index + 1] == '(' as u32 {

                    if let Some(_) = get_parenthesis_end_index(content, bracket_end_index + 1) {
                        return None;
                    }

                }

                else {  // content[bracket_end_index + 1] == '[' as u32

                    // `[link][[br]]` is a shortcut reference link followed by a macro
                    if let Some(second_bracket_end_index) = get_bracket_end_index(content, bracket_end_index + 1) {

                        if second_bracket_end_index == bracket_end_index + 2 || content[bracket_end_index + 2] != '[' as u32 {
                            return None;
                        }

                        match get_bracket_end_index(content, bracket_end_index + 2) {
                            Some(second_inner_bracket_end_index) if second_inner_bracket_end_index + 1 == second_bracket_end_index => {
                                // may be a shortcut reference link
                            }
                            _ => { return None; }
                        }

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
pub fn read_link_reference(content: &[u32]) -> (Vec<u32>, Vec<u32>) {  // (link_label, link_destination)

    let bracket_end_index = get_bracket_end_index(content, 0).unwrap();
    let link_label = content[1..bracket_end_index].to_vec();

    // TODO: `render_backslash_escapes` vs `undo_backslash_escapes`?
    let link_destination = render_backslash_escapes(
        &render_html_escapes(&drop_while(&content[(bracket_end_index + 2)..], ' ' as u32))
    );

    (link_label, link_destination)
}

fn is_valid_link_text(content: &[u32], link_references: &HashMap<Vec<u32>, Vec<u32>>) -> bool {

    !contains_link(content, link_references)  // it makes sure that the links are not nested
    && (content.is_empty()
    || if content[0] == '[' as u32
        && content[content.len() - 1] == ']' as u32
    {

        // [...] -> link
        // [[...]] -> macro
        // [[[...]]] -> macro in a link
        content.len() > 4 && content[1] == '[' as u32 && content[content.len() - 2] == ']' as u32
    } else {
        true
    })
}

pub fn is_valid_link_label(content: &[u32]) -> bool {

    // [...] -> link
    // [[...]] -> macro
    // [[[...]]] -> macro in a link
    if content.len() > 1 && content[0] == '[' as u32 && content[content.len() - 1] == ']' as u32 {
        content.len() > 4 && content[1] == '[' as u32 && content[content.len() - 2] == ']' as u32
    }

    else {
        true
    }

}

pub fn is_valid_link_destination(content: &[u32]) -> bool {
    content.iter().all(is_valid_url_character)
}

fn contains_link(content: &[u32], link_references: &HashMap<Vec<u32>, Vec<u32>>) -> bool {

    match content.iter().position(|c| *c == '[' as u32) {
        // this function is used to remove nested links
        // so images are fine
        Some(index)
            if index == 0 || content[index - 1] != '!' as u32 =>
        {
            read_direct_link(content, index, link_references).is_some()
            || read_reference_link(content, index, link_references).is_some()
            || read_shortcut_reference_link(content, index, link_references).is_some()
            || contains_link(&content[(index + 1)..], link_references)
        },
        _ => false,
    }

}

fn is_valid_url_character(character: &u32) -> bool {
    '-' as u32 <= *character && *character <= ';' as u32
    || 'a' as u32 <= *character && *character <= 'z' as u32
    || '?' as u32 <= *character && *character <= 'Z' as u32
    || '가' as u32 <= *character && *character <= '힣' as u32  // korean
    || 'ㄱ' as u32 <= *character && *character <= 'ㅣ' as u32  // korean
    || 'ぁ' as u32 <= *character && *character <= 'ヺ' as u32  // japanese
    || '!' as u32 == *character || '=' as u32 == *character
    || '+' as u32 == *character || '&' as u32 == *character
    || '%' as u32 == *character || '_' as u32 == *character
    || '#' as u32 == *character || '$' as u32 == *character
    || '+' as u32 == *character || '(' as u32 == *character 
    || ')' as u32 == *character
}
