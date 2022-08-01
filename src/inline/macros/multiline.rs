use crate::ast::line::Line;
use crate::utils::into_v16;
use super::{get_macro_name, parse_arguments, predicate::read_macro, parse::parse_html_tag, MACROS, Macro, MacroType};

#[derive(Clone)]
pub struct MultiLineMacro {
    macro_type: MultiLineMacroType,
    is_closing: bool
}

#[derive(Clone)]
enum MultiLineMacroType {
    Box { border: bool },
    Color(Vec<u16>),
    Size(Vec<u16>),
    Alignment(Vec<u16>),
    Highlight(Vec<u16>),
    HTML {
        tag: Vec<u16>,
        class: Vec<u16>,
        id: Vec<u16>
    }
}

impl MultiLineMacro {

    // all the validity checks are done before this function
    // this function assumes that everything is valid
    pub fn from_line(line: &Line) -> Self {
        let macro_content = read_macro(&line.content, 0).unwrap();
        let macro_arguments = parse_arguments(&macro_content);
        let mut macro_name = get_macro_name(&macro_arguments);
        let mut is_closing = false;

        if macro_name[0] == '/' as u16 {
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

                // for now, `no border` is the only valid argument for the `Box` macro
                // so a valid `Box` macro with more than 1 argument has no border
                macro_type: MultiLineMacroType::Box { border: macro_arguments.len() == 1 },
                is_closing
            },
            MacroType::Color => MultiLineMacro {
                macro_type: MultiLineMacroType::Color(macro_name.to_vec()),
                is_closing
            },
            MacroType::Size => MultiLineMacro {
                macro_type: MultiLineMacroType::Size(macro_name.to_vec()),
                is_closing
            },
            MacroType::Alignment => MultiLineMacro {
                macro_type: MultiLineMacroType::Alignment(macro_name.to_vec()),
                is_closing
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
                is_closing
            },
            MacroType::HTML => {
                let (tag, class, id) = if is_closing {
                    (macro_name.clone(), vec![], vec![])
                }
                
                else {
                    parse_html_tag(&macro_arguments)
                };

                MultiLineMacro {
                    macro_type: MultiLineMacroType::HTML { tag, class, id },
                    is_closing
                }
            },
            _ => {
                unreachable!()
            }
        }

    }

    pub fn to_html(&self, class_prefix: &str) -> Vec<u16> {
        
        if self.is_closing {

            match &self.macro_type {
                MultiLineMacroType::HTML { tag, .. } => vec![
                    into_v16("</"),
                    tag.clone(),
                    into_v16(">")
                ].concat(),
                _ => into_v16("</div>")
            }

        }

        else {

            match &self.macro_type {
                MultiLineMacroType::Box { border } => if *border {
                    into_v16(&format!("<div class=\"{}box\">", class_prefix))
                } else {
                    into_v16(&format!("<div class=\"{}box no-border\">", class_prefix))
                },
                MultiLineMacroType::Color(color) => vec![
                    into_v16(&format!("<div class=\"{}color-", class_prefix)),
                    color.clone(),
                    into_v16("\">")
                ].concat(),
                MultiLineMacroType::Size(size) => vec![
                    into_v16(&format!("<div class=\"{}size-", class_prefix)),
                    size.clone(),
                    into_v16("\">")
                ].concat(),
                MultiLineMacroType::Alignment(align) => vec![
                    into_v16(&format!("<div class=\"{}align-", class_prefix)),
                    align.clone(),
                    into_v16("\">")
                ].concat(),
                MultiLineMacroType::Highlight(highlight) => vec![
                    into_v16(&format!("<div class=\"{}highlight-", class_prefix)),
                    highlight.clone(),
                    into_v16("\">")
                ].concat(),
                MultiLineMacroType::HTML{ tag, class, id } => {
                    let mut result = vec![];

                    result.push(into_v16("<"));
                    result.push(tag.clone());

                    if class.len() > 0 {
                        result.push(into_v16(&format!(" class=\"{}", class_prefix)));
                        result.push(class.clone());
                        result.push(into_v16("\""));
                    }

                    if id.len() > 0 {
                        result.push(into_v16(" id=\""));
                        result.push(id.clone());
                        result.push(into_v16("\""));
                    }

                    result.push(into_v16(">"));

                    result.concat()
                }
            }

        }

    }

}