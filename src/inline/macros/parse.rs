use super::{Macro, MacroType, character::{DIRECT_MAPPINGS, INDIRECT_MAPPINGS}};
use crate::inline::{DecorationType, InlineNode, InlineMacro};
use crate::render::render_option::RenderOption;
use crate::utils::{into_v16, to_int};
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
                deco_type: DecorationType::Macro({
                    let repeat = if arguments[0].len() == 1 {
                        1
                    } else {
                        to_int(&arguments[0][1]).unwrap() as usize
                    };

                    InlineMacro::Br { repeat }
                }),
                content: vec![]
            },

            MacroType::Blank => InlineNode::Decoration {
                deco_type: DecorationType::Macro({
                    let repeat = if arguments[0].len() == 1 {
                        1
                    } else {
                        to_int(&arguments[0][1]).unwrap() as usize
                    };

                    InlineMacro::Blank { repeat }
                }),
                content: vec![]
            },

            MacroType::Char => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Char(

                    // number or direct_name
                    // &#32; or &infin; 
                    if arguments[0][1][0] < 'A' as u16 || DIRECT_MAPPINGS.contains(&arguments[0][1]) {
                        arguments[0][1].clone()
                    }

                    else {
                        INDIRECT_MAPPINGS.get(&arguments[0][1]).unwrap().to_vec()
                    }
                )),
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

                // for now, `no border` is the only valid argument for the `Box` macro
                // so a valid `Box` macro with more than 1 argument has no border
                deco_type: DecorationType::Macro(InlineMacro::Box { border: arguments.len() == 1 }),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Toc => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Toc),
                content: vec![]
            },

            MacroType::Math => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Math(content.to_vec())),
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