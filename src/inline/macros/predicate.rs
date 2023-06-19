use super::{normalize_macro, parse_arguments, get_macro_name, MACROS};
use crate::inline::InlineNode;
use crate::utils::{get_bracket_end_index, remove_whitespaces};
use crate::render::render_option::RenderOption;
use crate::ast::doc_data::DocData;

// "[[div, id = def]]" -> "div,id=def"
pub fn read_macro(content: &[u32], index: usize) -> Option<Vec<u32>> {

    if content.len() > 0 && content[index] == '[' as u32 && index + 1 < content.len() && content[index + 1] == '[' as u32 {

        match get_bracket_end_index(content, index) {
            None => { return None; }
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
                _ => { return None; }
            }
        }

    }

    else {
        None
    }

}

pub fn check_and_parse_macro_inline(
    content: &[u32],
    index: usize,
    doc_data: &mut DocData,
    render_option: &RenderOption
) -> Option<(InlineNode, usize)> {  // (parsed_macro, last_index)

    match read_macro(content, index) {
        Some(macro_content) => {
            let macro_arguments = parse_arguments(&macro_content);
            let macro_name = get_macro_name(&macro_arguments);
            let macro_end_index = get_bracket_end_index(content, index).unwrap();

            match MACROS.get(&macro_name) {
                Some(macro_) if macro_.is_valid(&macro_arguments) => {

                    if !macro_.has_closing {

                        if macro_name == &[116, 111, 99] {  // [116, 111, 99] = into_v32("toc")
                            doc_data.has_toc = true;
                        }

                        Some((macro_.parse(&macro_arguments, &vec![], doc_data, render_option), macro_end_index))
                    }

                    else if doc_data.tooltip_enabled > 0 && (
                        macro_name == &[116, 111, 111, 108, 116, 105, 112]  // into_v32("tooltip")
                        || macro_name == &[47, 116, 111, 111, 108, 116, 105, 112]  // into_v32("/tooltip")
                    ) {
                        None
                    }

                    else {
                        let closing_macro = macro_.get_closing_macro();
                        let mut curr_index = macro_end_index + 1;
                        let mut macro_nest_stack = 0;

                        while curr_index < content.len() {

                            if let Some(macro_content) = read_macro(content, curr_index) {

                                if macro_content == closing_macro {

                                    if macro_nest_stack == 0 {
                                        return Some(
                                            (
                                                macro_.parse(
                                                    &macro_arguments,
                                                    &content[macro_end_index + 1..curr_index],
                                                    doc_data,
                                                    render_option
                                                ),
                                                get_bracket_end_index(content, curr_index).unwrap()
                                            )
                                        );
                                    }

                                    else {
                                        macro_nest_stack -= 1;
                                    }

                                }

                                else {
                                    let inner_macro_arguments = parse_arguments(&macro_content);
                                    let inner_macro_name = get_macro_name(&inner_macro_arguments);

                                    if inner_macro_name == macro_name {  // the same macro is nested inside
                                        macro_nest_stack += 1;
                                    }

                                }

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

pub fn is_valid_macro_character(chr: &u32) -> bool {
    '/' as u32 <= *chr && *chr <= '9' as u32
    || 'a' as u32 <= *chr && *chr <= 'z' as u32
    || 'A' as u32 <= *chr && *chr <= 'Z' as u32
    || ' ' as u32 == *chr || '_' as u32 == *chr
    || ',' as u32 == *chr || '=' as u32 == *chr
}

// !![[macro, ...]] [[another macro...]]
// These macros are used in special contexts (in tables and lists)
pub fn is_special_macro(content: &[u32]) -> bool {

    let mut prefix = Vec::with_capacity(4);

    for c in content.iter() {

        if *c == ' ' as u32 {
            continue;
        }

        else if *c == '!' as u32 || *c == '[' as u32 {
            prefix.push(*c);

            if prefix.len() == 4 {
                break;
            }

        }

        else {
            return false;
        }

    }

    if prefix != &[33, 33, 91, 91] {  // into_v32("!![[")
        return false;
    }

    // if the content following `!!` purely consists of macros, it's a macro row
    let macros = remove_whitespaces(&content);
    let macros = macros[2..].to_vec();  // remove `!`s.

    let mut index = 0;

    while index < macros.len() {

        match read_macro(&macros, index) {
            Some(content) if content.iter().all(is_valid_macro_character) => {
                index = get_bracket_end_index(&macros, index).unwrap() + 1;
            }
            _ => {
                return false;
            }
        }

    }

    true
}