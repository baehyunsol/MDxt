use super::{Macro, MacroType};
use crate::inline::{InlineNode, DecorationType, InlineMacro};

impl Macro {

    pub fn parse(&self, arguments: &Vec<Vec<Vec<u16>>>, content: &[u16]) -> InlineNode {

        match self.macro_type {

            MacroType::Br => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Br),
                content: vec![]
            },

            MacroType::Blank => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Blank),
                content: vec![]
            },

            MacroType::Char => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Char(todo!())),
                content: vec![]
            },

            _ => todo!()
        }

    }

}