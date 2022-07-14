pub mod predicate;
pub mod toc;
mod parse;
mod validate;

#[cfg(test)]
mod testbench;

use crate::utils::{into_v16, from_v16, lowercase};
use crate::color::COLOR_NAMES;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MACROS: HashMap<Vec<u16>, Macro> = Macro::get_all_macros();
}

#[derive(Debug)]
pub struct Macro {
    pub name: Vec<u16>,
    macro_type: MacroType,
    no_closing: bool
}

#[derive(Debug)]
enum MacroType {
    Color, Size, Alignment, Highlight,
    Box, Toc, Blank, Br, Char, Icon, Math,
    HTML
}

impl Macro {

    pub fn get_all_macros() -> HashMap<Vec<u16>, Macro> {
        let mut macros = vec![
            Self::new_size("tiny"),
            Self::new_size("small"),
            Self::new_size("medium"),
            Self::new_size("big"),
            Self::new_size("giant"),
            Self::new_alignment("center"),
            Self::new_alignment("left"),
            Self::new_alignment("right"),
            Self::new("highlight", MacroType::Highlight, false),
            Self::new("box", MacroType::Box, false),
            Self::new("toc", MacroType::Toc, true),
            Self::new("blank", MacroType::Blank, true),
            Self::new("br", MacroType::Br, true),
            Self::new("char", MacroType::Char, true),
            Self::new("math", MacroType::Math, false),
            Self::new("icon", MacroType::Icon, true),
            Self::new("div", MacroType::HTML, false),
            Self::new("span", MacroType::HTML, false),
            Self::new("anchor", MacroType::HTML, false),
            Self::new("button", MacroType::HTML, false),
        ];

        for color in COLOR_NAMES.iter() {
            macros.push(Self::new_color(&from_v16(color)));
        }

        let mut result = HashMap::new();

        for macro_ in macros.into_iter() {
            result.insert(macro_.name.clone(), macro_);
        }

        result
    }

    fn new(name: &str, macro_type: MacroType, no_closing: bool) -> Self {
        Macro {
            name: into_v16(name), macro_type, no_closing
        }
    }

    fn new_color(name: &str) -> Self {
        Macro {
            name: into_v16(name),
            macro_type: MacroType::Color,
            no_closing: false
        }
    }

    fn new_size(name: &str) -> Self {
        Macro {
            name: into_v16(name),
            macro_type: MacroType::Size,
            no_closing: false
        }
    }

    fn new_alignment(name: &str) -> Self {
        Macro {
            name: into_v16(name),
            macro_type: MacroType::Alignment,
            no_closing: false
        }
    }

    fn get_closing_macro(&self) -> Vec<u16> {
        vec![
            into_v16("/"),
            self.name.clone()
        ].concat()
    }

}

fn normalize_macro(content: &[u16]) -> Vec<u16> {
    content.iter().filter(|c| **c != ' ' as u16 && **c != '_' as u16).map(lowercase).collect::<Vec<u16>>()
}

// [[icon = github, size = 32]] => [['icon', 'github'], ['size', '32']]
pub fn parse_arguments(content: &[u16]) -> Vec<Vec<Vec<u16>>> {
    content.split(
        |c| *c == ',' as u16
    ).map(
        |arg|
        arg.split(
            |a| *a == '=' as u16
        ).map(
            |a| a.to_vec()
        ).collect()
    ).collect()
}

pub fn get_macro_name(arguments: &Vec<Vec<Vec<u16>>>) -> Vec<u16> {
    arguments[0][0].clone()
}