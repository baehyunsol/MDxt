use super::{Macro, MacroType};
use crate::inline::{InlineNode, DecorationType, InlineMacro};
use crate::render::render_option::RenderOption;
use crate::utils::{to_int, into_v16};
use crate::escape::render_backslash_escapes;
use crate::ast::doc_data::DocData;

impl Macro {

    pub fn parse(
        &self,
        arguments: &Vec<Vec<Vec<u16>>>,
        content: &[u16],
        doc_data: &mut DocData,
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
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Size => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Size(arguments[0][0].clone())),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Highlight => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Highlight(arguments[0][1].clone())),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Alignment => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Alignment(arguments[0][0].clone())),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Box => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Box),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Toc => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Toc),
                content: vec![]
            },

            MacroType::Math => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Math(render_backslash_escapes(&content))),
                content: vec![]
            },

            MacroType::HTML => InlineNode::Decoration {
                deco_type: DecorationType::Macro({
                    let (tag, class, id) = parse_html_tag(arguments);

                    InlineMacro::HTML { tag, class, id }
                }),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Icon => todo!()
        }

    }

}

pub fn parse_html_tag(arguments: &Vec<Vec<Vec<u16>>>) -> (Vec<u16>, Vec<u16>, Vec<u16>) {  // (tag, class, id)
    
    let mut classes = vec![];
    let mut ids = vec![];

    for argument in arguments[1..].iter() {

        if argument[0] == into_v16("class") {
            classes.push(argument[1].clone());
        }

        else if argument[0] == into_v16("id") {
            ids.push(argument[1].clone());
        }

        else {
            panic!("Something's wrong with the engine itself. Please create an issue on github..");
        }

    }

    (arguments[0][0].clone(), classes.join(&[' ' as u16][..]), ids.join(&[' ' as u16][..]))
}