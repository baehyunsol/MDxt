pub mod multiline;
pub mod predicate;
pub mod toc;
pub mod tooltip;
mod character;
mod parse;
mod validate;

#[cfg(test)]
mod testbench;

use crate::utils::{into_v32, from_v32, lowercase};
use crate::color::COLOR_NAMES;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref MACROS: HashMap<Vec<u32>, Macro> = Macro::get_all_macros();
}

#[derive(Debug)]
pub struct Macro {
    pub name: Vec<u32>,
    macro_type: MacroType,
    pub has_closing: bool
}

#[derive(Debug)]
enum MacroType {
    Color, Size, LineHeight, Alignment, Highlight,
    Box, Toc, Blank, Br, Char, Icon, Math,
    HTML, Tooltip, Sidebar
}

impl Macro {

    pub fn get_all_macros() -> HashMap<Vec<u32>, Macro> {
        let mut macros = vec![
            Self::new_size("tiny"),
            Self::new_size("small"),
            Self::new_size("medium"),
            Self::new_size("big"),
            Self::new_size("giant"),
            Self::new_alignment("center"),
            Self::new_alignment("left"),
            Self::new_alignment("right"),
            Self::new("lineheight", MacroType::LineHeight, true),
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
            Self::new("sidebar", MacroType::Sidebar, true)
        ];

        for color in COLOR_NAMES.iter() {
            macros.push(Self::new_color(&from_v32(color)));
        }

        let mut result = HashMap::new();

        for macro_ in macros.into_iter() {
            result.insert(macro_.name.clone(), macro_);
        }

        result
    }

    fn new(name: &str, macro_type: MacroType, has_closing: bool) -> Self {
        Macro {
            name: into_v32(name), macro_type, has_closing
        }
    }

    fn new_color(name: &str) -> Self {
        Macro {
            name: into_v32(name),
            macro_type: MacroType::Color,
            has_closing: true
        }
    }

    fn new_size(name: &str) -> Self {
        Macro {
            name: into_v32(name),
            macro_type: MacroType::Size,
            has_closing: true
        }
    }

    fn new_alignment(name: &str) -> Self {
        Macro {
            name: into_v32(name),
            macro_type: MacroType::Alignment,
            has_closing: true
        }
    }

    pub fn get_closing_macro(&self) -> Vec<u32> {
        vec![
            into_v32("/"),
            self.name.clone()
        ].concat()
    }

}

fn normalize_macro(content: &[u32]) -> Vec<u32> {
    content.iter().filter(|c| **c != ' ' as u32 && **c != '_' as u32).map(lowercase).collect::<Vec<u32>>()
}

// [[icon = github, size = 32]] => [['icon', 'github'], ['size', '32']]
pub fn parse_arguments(content: &[u32]) -> Vec<Vec<Vec<u32>>> {
    content.split(
        |c| *c == ',' as u32
    ).map(
        |arg|
        arg.split(
            |a| *a == '=' as u32
        ).map(
            |a| a.to_vec()
        ).collect()
    ).collect()
}

pub fn get_macro_name(arguments: &Vec<Vec<Vec<u32>>>) -> Vec<u32> {
    arguments[0][0].clone()
}