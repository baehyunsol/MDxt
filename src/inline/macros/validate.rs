use super::{character::CHAR_NAMES, Macro, MacroType};
use crate::color::COLOR_NAMES;
use crate::container::icon::ICONS;
use crate::utils::{into_v16, to_int};

impl Macro {

    pub fn is_valid(&self, arguments: &Vec<Vec<Vec<u16>>>) -> bool {

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

            MacroType::Box => (arguments.len() == 1 && arguments[0].len() == 1)
            || (arguments.len() == 2 && arguments[1].len() == 1 && arguments[1][0] == into_v16("noborder")),

            MacroType::Char => arguments.len() == 1 && arguments[0].len() == 2 && (
                to_int(&arguments[0][1]).is_some() || CHAR_NAMES.contains(&arguments[0][1])
            ),

            MacroType::HTML => {
                let mut result = true;

                for argument in arguments[1..].iter() {
                    result = argument.len() == 2
                        && (argument[0] == into_v16("class")
                        || argument[0] == into_v16("id"));

                    if !result {
                        break;
                    }

                }

                result
            },

            MacroType::Highlight => arguments.len() == 1 && arguments[0].len() == 2 && COLOR_NAMES.contains(&arguments[0][1]),

            MacroType::Icon => arguments[0].len() == 2 && ICONS.contains_key(&arguments[0][1]) && (
                arguments.len() == 1 || arguments.len() == 2 && arguments[1][0] == into_v16("size") && match to_int(&arguments[1][1]) {
                    Some(n) if n < u16::MAX as u32 => true,
                    _ => false
                }
            )
        }

    }

}
