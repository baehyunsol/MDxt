use super::{
    get_macro_name, parse_arguments,
    Macro, MACROS, MacroType,
    predicate::read_macro, parse::{parse_html_tag, parse_box_arguments},
    super::math::render_math
};
use crate::ast::{line::Line, node::Node};
use crate::utils::{from_v32, into_v32};

#[derive(Clone)]
pub struct MultiLineMacro {
    pub macro_type: MultiLineMacroType,
    pub is_closing: bool,

    // all the multiline-macros have a unique id: a pair of opening and closing macro have the same id
    pub id: u64
}

#[derive(Clone)]
pub enum MultiLineMacroType {
    Box {
        border: bool,
        inline: bool,
        width: Vec<u32>,
        height: Vec<u32>,
    },
    Color(Vec<u32>),
    Size(Vec<u32>),
    LineHeight(Vec<u32>),
    Alignment(Vec<u32>),
    Highlight(Vec<u32>),
    Math(Vec<u32>),
    Tooltip(Vec<Node>),
    HTML {
        tag: Vec<u32>,
        class: Vec<u32>,
        id: Vec<u32>
    }
}

impl MultiLineMacroType {

    pub fn has_inner_nodes(&self) -> bool {

        // don't use wildcard character
        match self {
            MultiLineMacroType::Tooltip(_) => true,
            MultiLineMacroType::Box{ .. } |
            MultiLineMacroType::Color(_) |
            MultiLineMacroType::Size(_) |
            MultiLineMacroType::LineHeight(_) |
            MultiLineMacroType::Highlight(_) |
            MultiLineMacroType::Alignment(_) |
            MultiLineMacroType::Math(_) |
            MultiLineMacroType::HTML{ .. } => false
        }

    }

    pub fn set_inner_nodes(&mut self, nodes: Vec<Node>) {

        // it's okay to use wildcard character because the above function is not using it
        match self {
            MultiLineMacroType::Tooltip(nodes_) => {
                *nodes_ = nodes;
            }
            _ => {}
        }

    }

}

impl MultiLineMacro {

    // all the validity checks are done before this function
    // this function assumes that everything is valid
    pub fn from_line(line: &Line, id: u64) -> Self {
        let macro_content = read_macro(&line.content, 0).unwrap();
        let macro_arguments = parse_arguments(&macro_content);
        let mut macro_name = get_macro_name(&macro_arguments);
        let mut is_closing = false;

        if macro_name[0] == '/' as u32 {
            macro_name = macro_name[1..].to_vec();
            is_closing = true;
        }

        let Macro {
            name: macro_name,
            macro_type,
            has_closing  // supposed to be true
        } = MACROS.get(&macro_name).unwrap();

        #[cfg(test)]
        assert!(has_closing);

        match macro_type {
            MacroType::Box => MultiLineMacro {
                macro_type: {
                    let (border, inline, width, height) = parse_box_arguments(&macro_arguments);

                    MultiLineMacroType::Box { border, inline, width, height }
                },
                is_closing,
                id
            },
            MacroType::Color => MultiLineMacro {
                macro_type: MultiLineMacroType::Color(macro_name.to_vec()),
                is_closing,
                id
            },
            MacroType::Size => MultiLineMacro {
                macro_type: MultiLineMacroType::Size(macro_name.to_vec()),
                is_closing,
                id
            },
            MacroType::LineHeight => MultiLineMacro {
                macro_type: MultiLineMacroType::LineHeight(

                    if is_closing {
                        vec![]
                    }

                    else {
                        macro_arguments[0][1].clone()
                    }

                ),
                is_closing,
                id
            },
            MacroType::Alignment => MultiLineMacro {
                macro_type: MultiLineMacroType::Alignment(macro_name.to_vec()),
                is_closing,
                id
            },
            MacroType::Highlight => MultiLineMacro {
                macro_type: MultiLineMacroType::Highlight(

                    if is_closing {
                        vec![]
                    }

                    else {
                        macro_arguments[0][1].clone()
                    }

                ),
                is_closing,
                id
            },
            MacroType::HTML => {
                let (tag, class, html_id) = if is_closing {
                    (macro_name.clone(), vec![], vec![])
                }
                
                else {
                    parse_html_tag(&macro_arguments)
                };

                MultiLineMacro {
                    macro_type: MultiLineMacroType::HTML { tag, class, id: html_id },
                    is_closing,
                    id
                }
            },

            MacroType::Tooltip => MultiLineMacro {
                macro_type: MultiLineMacroType::Tooltip(vec![]),  // will be handled by another function
                is_closing,
                id
            },

            // it's handled by another ParseState
            MacroType::Math => unreachable!(),

            // macros that do not have closing tags
            MacroType::Toc | MacroType::Blank | MacroType::Br
            | MacroType::Char | MacroType::Icon => unreachable!("{macro_type:?}")
        }

    }

    pub fn to_html(&self, class_prefix: &str) -> Vec<u32> {
        
        if self.is_closing {

            match &self.macro_type {
                MultiLineMacroType::HTML { tag, .. } => vec![
                    into_v32("</"),
                    tag.clone(),
                    into_v32(">")
                ].concat(),

                MultiLineMacroType::Box { .. } | MultiLineMacroType::Color(_)
                | MultiLineMacroType::Size(_) | MultiLineMacroType::LineHeight(_)
                | MultiLineMacroType::Alignment(_) | MultiLineMacroType::Highlight(_)
                | MultiLineMacroType::Tooltip(_) => into_v32("</div>"),

                // it doesn't need any closing tag because `render_math` generates both opening and closing tags
                MultiLineMacroType::Math(_) => vec![]
            }

        }

        else {

            match &self.macro_type {
                MultiLineMacroType::Box { border, inline, width, height } => into_v32(&format!(
                    "<div class=\"{}box{}{}{}{}\">",
                    class_prefix,
                    if !border {
                        format!(" {}no-border", class_prefix)
                    } else {
                        String::new()
                    },
                    if *inline {
                        format!(" {}inline", class_prefix)
                    } else {
                        String::new()
                    },
                    if width.len() > 0 {
                        format!(" {}width-{}", class_prefix, from_v32(&width))
                    } else {
                        String::new()
                    },
                    if height.len() > 0 {
                        format!(" {}height-{}", class_prefix, from_v32(&height))
                    } else {
                        String::new()
                    }
                )),
                MultiLineMacroType::Color(color) => vec![
                    into_v32(&format!("<div class=\"{}color-", class_prefix)),
                    color.clone(),
                    into_v32("\">")
                ].concat(),
                MultiLineMacroType::Size(size) => vec![
                    into_v32(&format!("<div class=\"{}size-", class_prefix)),
                    size.clone(),
                    into_v32("\">")
                ].concat(),
                MultiLineMacroType::LineHeight(height) => vec![
                    into_v32(&format!("<div class=\"{}line-height-", class_prefix)),
                    height.clone(),
                    into_v32("\">")
                ].concat(),
                MultiLineMacroType::Alignment(align) => vec![
                    into_v32(&format!("<div class=\"{}align-", class_prefix)),
                    align.clone(),
                    into_v32("\">")
                ].concat(),
                MultiLineMacroType::Highlight(highlight) => vec![
                    into_v32(&format!("<div class=\"{}highlight-", class_prefix)),
                    highlight.clone(),
                    into_v32("\">")
                ].concat(),
                MultiLineMacroType::Tooltip(nodes) => todo!("{:?}", nodes.len()),  // what do I do...
                MultiLineMacroType::HTML{ tag, class, id } => {
                    let mut result = vec![];

                    result.push(into_v32("<"));
                    result.push(tag.clone());

                    if class.len() > 0 {
                        result.push(into_v32(&format!(" class=\"{}", class_prefix)));
                        result.push(class.clone());
                        result.push(into_v32("\""));
                    }

                    if id.len() > 0 {
                        result.push(into_v32(" id=\""));
                        result.push(id.clone());
                        result.push(into_v32("\""));
                    }

                    result.push(into_v32(">"));

                    result.concat()
                },
                MultiLineMacroType::Math(math) => {
                    render_math(math)
                }
            }

        }

    }

}