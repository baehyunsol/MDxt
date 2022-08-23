use crate::inline::macros::{
    parse_arguments,
    predicate::read_macro,
};
use crate::utils::{get_bracket_end_index, into_v16, remove_whitespaces, to_int};

pub fn try_parse_macro(content: &[u16]) -> (bool, Option<usize>) {  // (no_bullet, Option<start_index>)  // I'll define a struct for it when it gets more complicated
    let mut no_bullet = false;
    let mut start_index = None;

    let macros = remove_whitespaces(content);
    let macros = macros[2..].to_vec();  // remove `!`s.

    let mut index = 0;

    while index < macros.len() {
        let macro_content = match read_macro(&macros, index) {
            Some(c) => c,
            _ => break
        };

        let arguments = parse_arguments(&macro_content);

        for argument in arguments.iter() {

            if argument[0] == into_v16("nobullet") {
                no_bullet = true;
            }

            else if argument.len() == 2 && argument[0] == into_v16("start") {

                match to_int(&argument[1]) {
                    Some(n) if n > 0 => { start_index = Some(n as usize); }
                    _ => {}
                }

            }

        }

        index = get_bracket_end_index(&macros, index).unwrap() + 1;
    }

    (no_bullet, start_index)
}