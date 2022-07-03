use super::{Macro, MacroType};
use crate::render::render_option::RenderOption;
use crate::inline::{InlineNode, DecorationType, InlineMacro};
use crate::utils::to_int;
use crate::ast::MdData;

impl Macro {

    pub fn parse(
        &self,
        arguments: &Vec<Vec<Vec<u16>>>,
        content: &[u16],
        md_data: &MdData,
        render_option: &RenderOption
    ) -> InlineNode {

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
                deco_type: DecorationType::Macro(InlineMacro::Char(to_int(&arguments[0][1]).unwrap() as u16)),
                content: vec![]
            },

            MacroType::Color => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Color(arguments[0][0].clone())),
                content: InlineNode::from_md(content, md_data, render_option).to_vec()
            },

            MacroType::Size => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Size(arguments[0][0].clone())),
                content: InlineNode::from_md(content, md_data, render_option).to_vec()
            },

            MacroType::Alignment => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Alignment(arguments[0][0].clone())),
                content: InlineNode::from_md(content, md_data, render_option).to_vec()
            },

            MacroType::Box => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Box),
                content: InlineNode::from_md(content, md_data, render_option).to_vec()
            },

            MacroType::Toc => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Toc),
                content: vec![]
            },

            MacroType::Icon | MacroType::Math => todo!()
        }

    }

}
