use super::{character::CHAR_NAMES, Macro, MacroType};
use crate::color::COLOR_NAMES;
use crate::container::icon::ICONS;
use crate::utils::{into_v32, to_int};

impl Macro {

    pub fn is_valid(&self, arguments: &Vec<Vec<Vec<u32>>>) -> bool {

        match self.macro_type {

            // no args, only a name
            MacroType::Color | MacroType::Size
            | MacroType::Alignment | MacroType::Toc
            | MacroType::Math => arguments.len() == 1 && arguments[0].len() == 1,

            MacroType::Br | MacroType::Blank => {
                arguments.len() == 1 && (
                    arguments[0].len() == 1
                    || arguments[0].len() == 2 && to_int(&arguments[0][1]).is_some()
                )
            },

            MacroType::Box => arguments[0].len() == 1 && is_valid_box_arguments(&arguments),

            MacroType::Char => arguments.len() == 1 && arguments[0].len() == 2 && (
                to_int(&arguments[0][1]).is_some() || CHAR_NAMES.contains(&arguments[0][1])
            ),

            MacroType::Sidebar => arguments.len() == 1 || (
                // into_v32("default")
                arguments.len() == 2 && arguments[1][0] == [100, 101, 102, 97, 117, 108, 116] && (
                    arguments[1][1] == [104, 105, 100, 100, 101, 110] ||  // into_v32("hidden")
                    arguments[1][1] == [115, 104, 111, 119, 110]  // into_v32("shown")
                )
            ),

            MacroType::HTML => {
                let mut result = true;

                for argument in arguments[1..].iter() {
                    result = argument.len() == 2
                        && (argument[0] == [99, 108, 97, 115, 115]  // into_v32("class")
                        || argument[0] == [105, 100]);  // into_v32("id")

                    if !result {
                        break;
                    }

                }

                result
            },

            MacroType::LineHeight => arguments.len() == 1 && arguments[0].len() == 2 && [
                vec![116, 105, 110, 121],            // into_v32("tiny")
                vec![115, 109, 97, 108, 108],        // into_v32("small")
                vec![109, 101, 100, 105, 117, 109],  // into_v32("medium")
                vec![98, 105, 103],                  // into_v32("big")
                vec![103, 105, 97, 110, 116],        // into_v32("giant")
            ].contains(&arguments[0][1]),

            // for `[[tooltip = aabb]]`, it doesn't check whether `aabb` is valid or not,
            // -> it cannot check that
            MacroType::Tooltip => arguments.len() == 1 && arguments[0].len() == 2,

            MacroType::Highlight => arguments.len() == 1 && arguments[0].len() == 2 && COLOR_NAMES.contains(&arguments[0][1]),

            MacroType::Icon => arguments[0].len() == 2 && ICONS.contains_key(&arguments[0][1]) && (
                arguments.len() == 1  // no size
                || arguments.len() == 2 && arguments[1].len() == 2 && arguments[1][0] == into_v32("size") && match to_int(&arguments[1][1]) {
                    Some(n) if n < u32::MAX => true,
                    _ => false
                }
            )
        }

    }

}

fn is_valid_box_arguments(arguments: &Vec<Vec<Vec<u32>>>) -> bool {
    let mut result = true;

    for argument in arguments[1..].iter() {

        if (
            argument[0] == [110, 111, 98, 111, 114, 100, 101, 114] ||  // into_v32("noborder")
            argument[0] == [105, 110, 108, 105, 110, 101]  // into_v32("inline")
        ) && argument.len() == 1 {
            //
        }

        else if (
            argument[0] == [119, 105, 100, 116, 104] ||    // into_v32("width")
            argument[0] == [104, 101, 105, 103, 104, 116]  // into_v32("height")
        ) && argument.len() == 2 {

            if argument[1] != [116, 105, 110, 121]                // into_v32("tiny")
                && argument[1] != [115, 109, 97, 108, 108]        // into_v32("small")
                && argument[1] != [109, 101, 100, 105, 117, 109]  // into_v32("medium")
                && argument[1] != [98, 105, 103]                  // into_v32("big")
                && argument[1] != [103, 105, 97, 110, 116]        // into_v32("giant")
            {
                result = false;
                break;
            }

        }

        else {
            result = false;
            break;
        }

    }

    result
}
