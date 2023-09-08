use super::{
    get_macro_name, parse_arguments,
    Macro, MACROS, MacroType,
    predicate::read_macro, parse::{parse_html_tag, parse_box_arguments},
    super::math::render_math
};
use crate::RenderOption;
use crate::ast::{doc_data::DocData, line::Line, node::Node};
use crate::inline::macros::tooltip::load_tooltip_message;
use crate::utils::{from_v32, into_v32};

#[derive(Clone)]
pub struct MultiLineMacro {
    pub macro_type: MultiLineMacroType,
    pub is_closing: bool,
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
    Tooltip {
        container: Vec<Node>,  // outside
        label: Vec<u32>,  // inside, the actual message, which is `Vec<InlineNode>` will be loaded later
        index: usize
    },
    Sidebar,
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
            MultiLineMacroType::Tooltip { .. } | MultiLineMacroType::Sidebar => true,
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

    pub fn is_sidebar(&self) -> bool {

        match self {
            MultiLineMacroType::Sidebar => true,
            _ => false
        }

    }

    pub fn set_inner_nodes(&mut self, nodes: Vec<Node>) {

        // it's okay to use wildcard character because the above function is not using it
        match self {
            MultiLineMacroType::Tooltip{ container, .. } => {
                *container = nodes;
            }
            _ => {}
        }

    }

}

impl MultiLineMacro {

    // all the validity checks are done before this function
    // this function assumes that everything is valid
    pub fn from_line(line: &Line, doc_data: &mut DocData) -> Self {
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
            has_closing: _has_closing,  // supposed to be true
        } = MACROS.get(&macro_name).unwrap();

        #[cfg(test)]
        assert!(_has_closing);

        match macro_type {
            MacroType::Box => MultiLineMacro {
                macro_type: {
                    let (border, inline, width, height) = parse_box_arguments(&macro_arguments);

                    MultiLineMacroType::Box { border, inline, width, height }
                },
                is_closing,
            },
            MacroType::Color => MultiLineMacro {
                macro_type: MultiLineMacroType::Color(macro_name.to_vec()),
                is_closing,
            },
            MacroType::Size => MultiLineMacro {
                macro_type: MultiLineMacroType::Size(macro_name.to_vec()),
                is_closing,
            },
            MacroType::Sidebar => MultiLineMacro {
                macro_type: MultiLineMacroType::Sidebar,
                is_closing,
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
            },
            MacroType::Alignment => MultiLineMacro {
                macro_type: MultiLineMacroType::Alignment(macro_name.to_vec()),
                is_closing,
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
                }
            },

            MacroType::Tooltip => MultiLineMacro {
                macro_type: if !is_closing {
                    let label = macro_arguments[0][1].clone();
                    let index = doc_data.add_tooltip();

                    MultiLineMacroType::Tooltip {
                        container: vec![],  // will be handled by another function
                        label,
                        index
                    }
                } else {
                    MultiLineMacroType::Tooltip {
                        container: vec![],
                        label: vec![],
                        index: 0
                    }
                },
                is_closing,
            },

            // it's handled by another ParseState
            MacroType::Math => unreachable!(),

            // macros that do not have closing tags
            MacroType::Toc | MacroType::Blank | MacroType::Br
            | MacroType::Char | MacroType::Icon => unreachable!("{macro_type:?}")
        }

    }

    pub fn to_html(&self, toc_rendered: &Vec<u32>, render_option: &RenderOption, doc_data: &mut DocData) -> Vec<u32> {
        let class_prefix = &render_option.class_prefix;

        if self.is_closing {

            match &self.macro_type {
                MultiLineMacroType::HTML { tag, .. } => vec![
                    vec![60, 47],  // into_v32("</")
                    tag.clone(),
                    vec![62],  // into_v32(">")
                ].concat(),

                MultiLineMacroType::Box { .. } | MultiLineMacroType::Color(_)
                | MultiLineMacroType::Size(_) | MultiLineMacroType::LineHeight(_)
                | MultiLineMacroType::Alignment(_) | MultiLineMacroType::Highlight(_)
                | MultiLineMacroType::Tooltip { .. } => vec![60, 47, 100, 105, 118, 62],  // into_v32("</div>")

                // it doesn't need any closing tag because `render_math` generates both opening and closing tags
                MultiLineMacroType::Math(_) => vec![],

                // this variant should be completely ignored by this function!
                MultiLineMacroType::Sidebar => vec![]
            }

        }

        else {

            match &self.macro_type {
                MultiLineMacroType::Box { border, inline, width, height } => into_v32(&format!(
                    "<div class=\"{class_prefix}box{}{}{}{}\">",
                    if !border {
                        format!(" {class_prefix}no-border")
                    } else {
                        String::new()
                    },
                    if *inline {
                        format!(" {class_prefix}inline")
                    } else {
                        String::new()
                    },
                    if !width.is_empty() {
                        format!(" {class_prefix}width-{}", from_v32(&width))
                    } else {
                        String::new()
                    },
                    if !height.is_empty() {
                        format!(" {class_prefix}height-{}", from_v32(&height))
                    } else {
                        String::new()
                    }
                )),
                MultiLineMacroType::Color(color) => vec![
                    into_v32(&format!("<div class=\"{class_prefix}color-")),
                    color.clone(),
                    vec![34, 62],  // into_v32("\">")
                ].concat(),
                MultiLineMacroType::Size(size) => vec![
                    into_v32(&format!("<div class=\"{class_prefix}size-")),
                    size.clone(),
                    vec![34, 62],  // into_v32("\">")
                ].concat(),
                MultiLineMacroType::LineHeight(height) => vec![
                    into_v32(&format!("<div class=\"{class_prefix}line-height-")),
                    height.clone(),
                    vec![34, 62],  // into_v32("\">")
                ].concat(),
                MultiLineMacroType::Alignment(align) => vec![
                    into_v32(&format!("<div class=\"{class_prefix}align-")),
                    align.clone(),
                    vec![34, 62],  // into_v32("\">")
                ].concat(),
                MultiLineMacroType::Highlight(highlight) => vec![
                    into_v32(&format!("<div class=\"{class_prefix}highlight-")),
                    highlight.clone(),
                    vec![34, 62],  // into_v32("\">")
                ].concat(),
                // Node::to_html requires (toc_rendered: Vec<u32>) and (render_option: RenderOption)
                MultiLineMacroType::Tooltip { container, index, label } => {
                    let mut inner_html_buffer = vec![];

                    for node in container.iter() {
                        node.to_html(toc_rendered, render_option, doc_data, &mut inner_html_buffer);
                    }

                    let message = load_tooltip_message(&label, doc_data, render_option);

                    vec![
                        into_v32(&format!(
                            "<div class=\"{class_prefix}tooltip-container\" id=\"tooltip-container-{index}\">",
                        )),
                        inner_html_buffer.concat(),
                        into_v32(&format!(
                            "<div class=\"{class_prefix}tooltip-message\" id=\"tooltip-message-{index}\">",
                        )),
                        message.iter().map(
                            |node| node.to_html(toc_rendered, class_prefix)
                        ).collect::<Vec<Vec<u32>>>().concat(),

                        // the closing tag is handled by another MultiLineMacroType::Tooltip
                        // into_v32("</div>") -> [60, 47, 100, 105, 118, 62]
                        vec![60, 47, 100, 105, 118, 62],
                    ].concat()
                },
                MultiLineMacroType::HTML{ tag, class, id } => {
                    let mut result = vec![];

                    result.push(vec![60]);  // into_v32("<") -> [60]
                    result.push(tag.clone());

                    if !class.is_empty() {
                        result.push(into_v32(&format!(" class=\"{class_prefix}")));
                        result.push(class.clone());
                        result.push(vec![34]);  // into_v32("\"") -> [34]
                    }

                    if !id.is_empty() {
                        result.push(vec![32, 105, 100, 61, 34]);  // into_v32(" id=\"") -> [32, 105, 100, 61, 34]
                        result.push(id.clone());
                        result.push(vec![34]);  // into_v32("\"") -> [34]
                    }

                    result.push(vec![62]);  // into_v32(">") -> [62]

                    result.concat()
                },
                MultiLineMacroType::Math(math) => {
                    render_math(math)
                },
                MultiLineMacroType::Sidebar => {
                    // handled by AST::to_html
                    vec![]
                }
            }

        }

    }

}