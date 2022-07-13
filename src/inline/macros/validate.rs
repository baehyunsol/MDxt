use super::{Macro, MacroType};
use crate::utils::{to_int, into_v16};
use crate::color::COLOR_NAMES;

impl Macro {

    pub fn is_valid(&self, arguments: &Vec<Vec<Vec<u16>>>) -> bool {
        
        match self.macro_type {

            // no args, only a name
            MacroType::Br | MacroType::Blank | MacroType::Color |
            MacroType::Size | MacroType::Alignment | MacroType::Toc |
            MacroType::Box | MacroType::Math => arguments.len() == 1 && arguments[0].len() == 1,

            MacroType::Char => arguments.len() == 1 && arguments[0].len() == 2 && match to_int(&arguments[0][1]) {
                Some(n) => n < u16::MAX as u32,
                _ => false
            },

            MacroType::HTML => {
                let mut result = true;

                for argument in arguments[1..].iter() {
                    result = argument.len() == 2 && (argument[0] == into_v16("class") || argument[0] == into_v16("id"));

                    if !result {
                        break;
                    }

                }

                result
            },

            MacroType::Highlight => arguments.len() == 1 && arguments[0].len() == 2 && COLOR_NAMES.contains(&arguments[0][1]),

            MacroType::Icon => todo!()
        }

    }

}
