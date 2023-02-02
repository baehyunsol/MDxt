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

            MacroType::Box => (arguments[0].len() == 1) && is_valid_box_arguments(&arguments),

            MacroType::Char => arguments.len() == 1 && arguments[0].len() == 2 && (
                to_int(&arguments[0][1]).is_some() || CHAR_NAMES.contains(&arguments[0][1])
            ),

            MacroType::HTML => {
                let mut result = true;

                for argument in arguments[1..].iter() {
                    result = argument.len() == 2
                        && (argument[0] == into_v32("class")
                        || argument[0] == into_v32("id"));

                    if !result {
                        break;
                    }

                }

                result
            },

            // for `[[tooltip = aabb]]`, it doesn't check whether `aabb` is valid or not,
            // -> it cannot check that
            MacroType::Tooltip => arguments.len() == 1 && arguments[0].len() == 2,

            MacroType::Highlight => arguments.len() == 1 && arguments[0].len() == 2 && COLOR_NAMES.contains(&arguments[0][1]),

            MacroType::Icon => arguments[0].len() == 2 && ICONS.contains_key(&arguments[0][1]) && (
                arguments.len() == 1 || arguments.len() == 2 && arguments[1][0] == into_v32("size") && match to_int(&arguments[1][1]) {
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

        if (argument[0] == into_v32("noborder") || argument[0] == into_v32("inline")) && argument.len() == 1 {
            //
        }

        else if (argument[0] == into_v32("width") || argument[0] == into_v32("height")) && argument.len() == 2 {

            if argument[1] != into_v32("tiny")
                && argument[1] != into_v32("small")
                && argument[1] != into_v32("medium")
                && argument[1] != into_v32("big")
                && argument[1] != into_v32("giant")
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