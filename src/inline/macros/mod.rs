pub mod multiline;
pub mod predicate;
pub mod toc;
pub mod tooltip;
mod character;
mod parse;
mod validate;

#[cfg(test)]
mod testbench;

use crate::utils::{into_v16, from_v16, lowercase};
use crate::color::COLOR_NAMES;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref MACROS: HashMap<Vec<u16>, Macro> = Macro::get_all_macros();
}

#[derive(Debug)]
pub struct Macro {
    pub name: Vec<u16>,
    macro_type: MacroType,
    pub has_closing: bool
}

#[derive(Debug)]
enum MacroType {
    Color, Size, Alignment, Highlight,
    Box, Toc, Blank, Br, Char, Icon, Math,
    HTML, Tooltip
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
            Self::new("highlight", MacroType::Highlight, true),
            Self::new("box", MacroType::Box, true),
            Self::new("toc", MacroType::Toc, false),
            Self::new("tooltip", MacroType::Tooltip, true),
            Self::new("blank", MacroType::Blank, false),
            Self::new("br", MacroType::Br, false),
            Self::new("char", MacroType::Char, false),
            Self::new("math", MacroType::Math, true),
            Self::new("icon", MacroType::Icon, false),
            Self::new("div", MacroType::HTML, true),
            Self::new("span", MacroType::HTML, true),
            Self::new("anchor", MacroType::HTML, true),
            Self::new("button", MacroType::HTML, true),
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

    fn new(name: &str, macro_type: MacroType, has_closing: bool) -> Self {
        Macro {
            name: into_v16(name), macro_type, has_closing
        }
    }

    fn new_color(name: &str) -> Self {
        Macro {
            name: into_v16(name),
            macro_type: MacroType::Color,
            has_closing: true
        }
    }

    fn new_size(name: &str) -> Self {
        Macro {
            name: into_v16(name),
            macro_type: MacroType::Size,
            has_closing: true
        }
    }

    fn new_alignment(name: &str) -> Self {
        Macro {
            name: into_v16(name),
            macro_type: MacroType::Alignment,
            has_closing: true
        }
    }

    pub fn get_closing_macro(&self) -> Vec<u16> {
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