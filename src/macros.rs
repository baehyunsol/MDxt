mod predicate;

#[cfg(test)]
mod testbench;

use crate::utils::{into_v16, lowercase};
use std::collections::HashMap;

// print("\n".join([str((i, chr(i))) for i in range(128)]))

struct Macro {
    pub name: Vec<u16>,
    macro_type: MacroType,
    no_closing: bool,
    no_args: bool,
}

enum MacroType {
    Color, Size, Alignment,
    Box, Toc, Blank, Br, Char, Icon, Math
}

impl Macro {

    pub fn get_all_macros() -> HashMap<Vec<u16>, Macro> {
        let macros = vec![
            Self::new_color("red"),
            Self::new_color("green"),
            Self::new_color("blue"),
            Self::new_size("tiny"),
            Self::new_size("small"),
            Self::new_size("medium"),
            Self::new_size("big"),
            Self::new_size("giant"),
            Self::new_alignment("center"),
            Self::new_alignment("left"),
            Self::new_alignment("right"),
            Self::new("box", MacroType::Box, false, true),
            Self::new("toc", MacroType::Toc, true, true),
            Self::new("blank", MacroType::Blank, true, true),
            Self::new("br", MacroType::Br, true, true),
            Self::new("char", MacroType::Char, true, false),
            Self::new("math", MacroType::Math, false, true),
        ];

        let mut result = HashMap::new();

        for macro_ in macros.into_iter() {
            result.insert(macro_.name.clone(), macro_);
        }

        result
    }

    fn new(name: &str, macro_type: MacroType, no_closing: bool, no_args: bool) -> Self {
        Macro {
            name: into_v16(name), macro_type, no_closing, no_args
        }
    }

    fn new_color(name: &str) -> Self {
        Macro {
            name: into_v16(name),
            macro_type: MacroType::Color,
            no_closing: false,
            no_args: true
        }
    }

    fn new_size(name: &str) -> Self {
        Macro {
            name: into_v16(name),
            macro_type: MacroType::Size,
            no_closing: false,
            no_args: true
        }
    }

    fn new_alignment(name: &str) -> Self {
        Macro {
            name: into_v16(name),
            macro_type: MacroType::Alignment,
            no_closing: false,
            no_args: true
        }
    }

}

fn normalize_macro(content: &[u16]) -> Vec<u16> {
    content.iter().filter(|c| **c != ' ' as u16 && **c != '_' as u16).map(|c| lowercase(*c)).collect::<Vec<u16>>()
}

// [[icon = github, size = 32]] => [['icon', 'github'], ['size', '32']]
fn parse_arguments(content: &[u16]) -> Vec<Vec<Vec<u16>>> {
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

fn get_macro_name(arguments: &Vec<Vec<Vec<u16>>>) -> Vec<u16> {
    arguments[0][0].clone()
}