use super::{normalize_macro, parse_arguments, get_macro_name, MACROS};
use crate::utils::get_bracket_end_index;
use crate::inline::InlineNode;
use crate::render::render_option::RenderOption;
use crate::ast::MdData;

pub fn read_macro(content: &[u16], index: usize) -> Option<Vec<u16>> {

    if content[index] == '[' as u16 && index + 1 < content.len() && content[index + 1] == '[' as u16 {

        match get_bracket_end_index(content, index) {
            None => {return None;}
            Some(end_index1) => match get_bracket_end_index(content, index + 1) {
                Some(end_index2) if end_index2 + 1 == end_index1 && content[index + 2..end_index2].iter().all(is_valid_macro_character) => {
                    let macro_content = normalize_macro(&content[index + 2..end_index2]);
                    
                    if macro_content.len() > 0 {
                        Some(macro_content)
                    }

                    else {
                        None
                    }

                }
                _ => {return None;}
            }
        }

    }

    else {
        None
    }

}

pub fn check_and_parse_macro_inline(
    content: &[u16],
    index: usize,
    md_data: &MdData,
    render_option: &RenderOption
) -> Option<(InlineNode, usize)> {  // (parsed_macro, last_index)

    match read_macro(content, index) {
        Some(macro_content) => {
            let macro_arguments = parse_arguments(&macro_content);
            let macro_name = get_macro_name(&macro_arguments);
            let macro_end_index = get_bracket_end_index(content, index).unwrap();

            match MACROS.get(&macro_name) {
                Some(macro_) if macro_.is_valid(&macro_arguments) => {

                    if macro_.no_closing {
                        Some((macro_.parse(&macro_arguments, &vec![], md_data, render_option), macro_end_index))
                    }

                    else {
                        let closing_macro = macro_.get_closing_macro();
                        let mut curr_index = macro_end_index + 1;

                        while curr_index < content.len() {

                            match read_macro(content, curr_index) {
                                Some(macro_content) if macro_content == closing_macro => {
                                    return Some(
                                        (
                                            macro_.parse(
                                                &macro_arguments,
                                                &content[macro_end_index + 1..curr_index],
                                                md_data,
                                                render_option
                                            ),
                                            get_bracket_end_index(content, curr_index).unwrap()
                                        )
                                    );
                                },
                                _ => {}
                            }

                            curr_index += 1;
                        }

                        // the closing macro is not found
                        None
                    }

                },
                _ => None
            }

        },
        None => None
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