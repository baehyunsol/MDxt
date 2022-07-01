use super::{Macro, MacroType};

impl Macro {

    pub fn is_valid(&self, arguments: &Vec<Vec<Vec<u16>>>) -> bool {
        
        match self.macro_type {

            // no args, only a name
            MacroType::Br | MacroType::Blank | MacroType::Color |
            MacroType::Size | MacroType::Alignment | MacroType::Toc |
            MacroType::Box | MacroType::Math => arguments.len() == 1 && arguments[0].len() == 1,

            //_ => todo!()
        }

    }

}