use super::{Macro, MacroType, character::{DIRECT_MAPPINGS, INDIRECT_MAPPINGS}, get_macro_name};
use crate::inline::{DecorationType, InlineNode, InlineMacro};
use crate::render::render_option::RenderOption;
use crate::utils::{into_v16, to_int};
use crate::ast::doc_data::DocData;

impl Macro {

    // all the validity checks are done before this function
    // this function assumes that everything is valid
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
                deco_type: DecorationType::Macro(InlineMacro::Color(get_macro_name(arguments))),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Size => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Size(get_macro_name(arguments))),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Highlight => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Highlight(arguments[0][1].clone())),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Alignment => InlineNode::Decoration {
                deco_type: DecorationType::Macro(InlineMacro::Alignment(get_macro_name(arguments))),
                content: InlineNode::from_mdxt(content, doc_data, render_option).to_vec()
            },

            MacroType::Box => InlineNode::Decoration {
                deco_type: DecorationType::Macro({
                    let (border, inline, width, height) = parse_box_arguments(&arguments);

                    InlineMacro::Box { border, inline, width, height }
                }),
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

            MacroType::Icon => InlineNode::Decoration {
                deco_type: DecorationType::Macro({
                    let name = arguments[0][1].clone();
                    let size = if arguments.len() > 1 {
                        to_int(&arguments[1][1]).unwrap().min(u16::MAX as u32) as u16
                    } else {
                        32 as u16
                    };

                    InlineMacro::Icon { name, size }
                }),
                content: vec![]
            }
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

    (get_macro_name(arguments), classes.join(&[' ' as u16][..]), ids.join(&[' ' as u16][..]))
}

// all the validity checks are done before this function
// this function assumes that everything is valid
pub fn parse_box_arguments(arguments: &Vec<Vec<Vec<u16>>>) -> (bool, bool, Vec<u16>, Vec<u16>) {  // (HasBorder, Inline, Width, Height)
    let mut no_border = false;
    let mut inline = false;
    let mut width = vec![];
    let mut height = vec![];

    for argument in arguments[1..].iter() {

        if argument[0] == into_v16("noborder") {
            no_border = true;
        }

        else if argument[0] == into_v16("inline") {
            inline = true;
        }

        else if argument[0] == into_v16("width") {
            width = argument[1].clone();
        }

        else if argument[0] == into_v16("height") {
            height = argument[1].clone();
        }

    }

    (!no_border, inline, width, height)
}