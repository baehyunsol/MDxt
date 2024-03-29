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

use crate::inline::macros::{
    parse_arguments,
    predicate::read_macro,
};
use crate::utils::{get_bracket_end_index, into_v32, remove_whitespaces};

pub struct TableMacros {
    pub collapsible: bool,
    pub default_hidden: bool,
    pub headless: bool,
    pub id: Option<Vec<u32>>,
    pub classes: Vec<Vec<u32>>,
    pub sort: bool,
}

pub fn try_parse_macro(content: &[u32]) -> TableMacros {
    let mut collapsible = false;
    let mut default_hidden = false;
    let mut headless = false;
    let mut id = None;
    let mut classes = vec![];
    let mut sort = false;

    let macros = remove_whitespaces(content);
    let macros = macros[3..(macros.len() - 1)].to_vec();  // remove `!`s and `|`s.

    let mut index = 0;

    while index < macros.len() {
        let macro_content = match read_macro(&macros, index) {
            Some(c) => c,
            _ => break
        };

        let arguments = parse_arguments(&macro_content);

        for argument in arguments.iter() {
            if argument[0] == into_v32("collapsible") {
                collapsible = true;
            }

            else if argument[0] == into_v32("headless") {
                headless = true;
            }

            else if argument[0] == into_v32("sort") {
                sort = true;
            }

            else if argument.len() == 2 && argument[0] == into_v32("id") {
                id = Some(argument[1].clone());
            }

            else if argument.len() == 2 && argument[0] == into_v32("class") {
                classes.push(argument[1].clone());
            }

            else if argument.len() == 2 && argument[0] == into_v32("default") {
                if argument[1] == into_v32("shown") {
                    default_hidden = false;
                }

                else if argument[1] == into_v32("hidden") {
                    default_hidden = true;
                }
            }
        }

        index = get_bracket_end_index(&macros, index).unwrap() + 1;
    }

    // if both `collapsible` and `headless` are set, `headless` is ignored
    if collapsible && headless {
        headless = false;
    }

    TableMacros { collapsible, default_hidden, headless, id, classes, sort }
}

/// You can also write your own.
///
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
