/*
Table-wide macros

| a          | b          | c          |
|------------|------------|------------|
|!![[collapsible, default=shown]]      |
| aa         | bb         | cc         |

1. A macro row is a valid table row.
1. A macro row starts with two bangs(!).
1. The two bangs are followed by macros.
1. Whitespaces are okay.
*/

use crate::ast::line::Line;
use crate::inline::macros::{
    predicate::{read_macro, is_valid_macro_character},
    parse_arguments
};
use crate::utils::{into_v16, remove_whitespaces, get_bracket_end_index};

pub fn is_macro_row(line: &Line) -> bool {

    #[cfg(test)]
    assert!(line.content[0] == '|' as u16);

    let mut prefix = Vec::with_capacity(4);

    for c in line.content[1..].iter() {

        if *c == ' ' as u16 {
            continue;
        }

        else if *c == '!' as u16 || *c == '[' as u16 {
            prefix.push(*c);

            if prefix.len() == 4 {
                break;
            }

        }

        else {
            return false;
        }

    }

    if prefix != into_v16("!![[") {
        return false;
    }

    // if the content following `!!` purely consists of macros, it's a macro row
    let macros = remove_whitespaces(&line.content);
    let macros = macros[3..macros.len() - 1].to_vec();  // remove `!`s and `|`s.

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

pub fn try_parse_macro(line: &Line) -> (bool, bool) {  // (collapsible, default_hidden)  // I'll define a struct for it when it gets more complicated

    let mut collapsible = false;
    let mut default_hidden = false;

    let macros = remove_whitespaces(&line.content);
    let macros = macros[3..macros.len() - 1].to_vec();  // remove `!`s and `|`s.

    let mut index = 0;

    while index < macros.len() {
        let macro_content = match read_macro(&macros, index) {
            Some(c) => c,
            _ => break
        };

        let arguments = parse_arguments(&macro_content);

        for argument in arguments.iter() {

            if argument[0] == into_v16("collapsible") {
                collapsible = true;
            }

            else if argument.len() == 2 && argument[0] == into_v16("default") {

                if argument[1] == into_v16("shown") {
                    default_hidden = false;
                }

                else if argument[1] == into_v16("hidden") {
                    default_hidden = true;
                }

            }

        }

        index = get_bracket_end_index(&macros, index).unwrap() + 1;
    }

    (collapsible, default_hidden)
}

/// You can also write your own.
/// ```javascript
/// function collapse_table(n) {
///     var head = document.getElementById("table-collapse-toggle-" + n);
///     head.classList.toggle("collapsed");
///
///     var content = document.getElementById("collapsible-table-" + n);
///     content.classList.toggle("invisible");
/// }
/// ```
pub fn collapsible_table_javascript() -> String {
"function collapse_table(n) {
    var head = document.getElementById(\"table-collapse-toggle-\" + n);
    head.classList.toggle(\"collapsed\");

    var content = document.getElementById(\"collapsible-table-\" + n);
    content.classList.toggle(\"invisible\");
}".to_string()
}