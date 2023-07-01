mod entity;
mod parse;
mod validate;

#[cfg(test)]
mod testbench;

use super::macros::predicate::read_macro;
use super::parse::{get_code_span_marker_end_index, is_code_span_marker_begin, undo_code_span_escapes};
use crate::escape::{render_backslash_escapes_raw, undo_html_escapes, BACKSLASH_ESCAPE_OFFSET};
use crate::utils::{get_bracket_end_index, into_v32};
use entity::Entity;
use lazy_static::lazy_static;
use parse::md_to_math;
use std::collections::HashSet;

lazy_static! {

    pub static ref ZERO_ARG_FUNCTIONS: HashSet<Vec<u32>> = {
        let vec = vec![
            "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota", "kappa", "lambda", "mu", "nu", "xi", "omicron", "pi", "rho", "sigma", "tau", "upsilon", "phi", "chi", "psi", "omega",
            "Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Eta", "Theta", "Iota", "Kappa", "Lambda", "Mu", "Nu", "Xi", "Omicron", "Pi", "Rho", "Sigma", "Tau", "Upsilon", "Phi", "Chi", "Psi", "Omega",
            "times", "pm", "leftarrow", "uparrow", "rightarrow", "downarrow", "forall",
            "partial", "exist", "empty", "null", "triangle", "nabla", "in", "notin",
            "ni", "notni", "qed", "mp", "circ", "bullet", "prop", "inf", "infty", "infin",
            "and", "or", "cap", "cup", "therefore", "because", "simeq", "asymp", "ne", "neq",
            "equiv", "nequiv", "lt", "gt", "le", "leq", "ge", "geq", "llt", "ggt",
            "sub", "sup", "nsub", "nsup", "sube", "supe", "nsube", "nsupe",
            "oplus", "ominus", "otimes", "odiv", "odot", "dot", "star", "br",
            "lcb",  // left curly brace
            "rcb",  // right curly brace
        ];
        let mut result = HashSet::with_capacity(vec.len());

        #[cfg(test)]
        let vec_len = vec.len();

        for func in vec.into_iter() {
            result.insert(into_v32(func));
        }

        #[cfg(test)]
        assert_eq!(vec_len, result.len());

        result
    };

    pub static ref ONE_ARG_FUNCTIONS: HashSet<Vec<u32>> = {
        let vec = vec![
            "text", "sqrt", "lim", "limit",
            "hat", "bar", "dot", "tilde", "vec"
        ];
        let mut result = HashSet::with_capacity(vec.len());

        for func in vec.into_iter() {
            result.insert(into_v32(func));
        }

        result
    };

    pub static ref TWO_ARG_FUNCTIONS: HashSet<Vec<u32>> = {
        let vec = vec![
            "sum", "prod", "sqrt", "root",
            "sup", "sub",
            "frac", "cfrac", "bincoeff",
            "int", "oint", "iint", "iiint"
        ];
        let mut result = HashSet::with_capacity(vec.len());

        for func in vec.into_iter() {
            result.insert(into_v32(func));
        }

        result
    };

    pub static ref THREE_ARG_FUNCTIONS: HashSet<Vec<u32>> = {
        let vec = vec![
            "subsup"
        ];
        let mut result = HashSet::with_capacity(vec.len());

        for func in vec.into_iter() {
            result.insert(into_v32(func));
        }

        result
    };

    pub static ref FIVE_ARG_FUNCTIONS: HashSet<Vec<u32>> = {
        let vec = vec![
            "multiscript"
        ];
        let mut result = HashSet::with_capacity(vec.len());

        for func in vec.into_iter() {
            result.insert(into_v32(func));
        }

        result
    };

}

pub struct Math {
    entities: Vec<Entity>
}

impl Math {

    pub fn from_mdxt(content: &[u32]) -> Self {
        let entities = md_to_math(&render_backslash_escapes_raw(content));

        Math { entities }
    }

    pub fn to_math_ml(&self, xmlns: bool) -> Vec<u32> {
        let xmlns = if xmlns {
            " xmlns=\"http://www.w3.org/1998/Math/MathML\""
        } else {
            ""
        };

        vec![
            into_v32(&format!("<math{xmlns}>")),
            self.entities.iter().map(
                |entity| entity.to_math_ml()
            ).collect::<Vec<Vec<u32>>>().concat(),
            vec![60, 47, 109, 97, 116, 104, 62],  // into_v32("</math>")
        ].concat()
    }
}

// This escape only works inside `[[math]]` macros
// I don't want other inline elements to interrupt math formulas.
fn escape_special_characters(content: &[u32]) -> Vec<u32> {
    let content = undo_html_escapes(content);
    let mut result = Vec::with_capacity(content.len() + content.len() / 6);

    for c in content.iter() {

        // into_v32("<>*~[|]^`&")
        if [60, 62, 42, 126, 91, 124, 93, 94, 96, 38].contains(c) {
            result.push(BACKSLASH_ESCAPE_OFFSET + *c);
        }

        else {
            result.push(*c);
        }

    }

    undo_code_span_escapes(&result)
}

pub fn escape_inside_math_blocks(content: Vec<u32>) -> Vec<u32> {
    let mut result = vec![];
    let mut index = 0;
    let mut last_index = 0;

    while index < content.len() {

        if is_code_span_marker_begin(&content, index) {
            index = get_code_span_marker_end_index(&content, index);
            continue;
        }

        match read_macro(&content, index) {

            // it met `[[math]]`
            // into_v32("math") -> [109, 97, 116, 104]
            Some(macro_name) if macro_name == &[109, 97, 116, 104] => {
                let mut end_index = index + 5;

                // seek `[[/math]]`
                while end_index < content.len() {

                    match read_macro(&content, end_index) {
                        Some(macro_name) if macro_name == &[47, 109, 97, 116, 104] => {  // into_v32("/math") -> [47, 109, 97, 116, 104]
                            let math_begin_index = get_bracket_end_index(&content, index).unwrap() + 1;
                            let escaped_math = escape_special_characters(&content[math_begin_index..end_index]);

                            result.push(content[last_index..math_begin_index].to_vec());
                            result.push(escaped_math);

                            last_index = end_index;
                            index = end_index;
                            break;
                        }
                        _ => {}
                    }

                    end_index += 1;
                }

            },
            _ => {}
        }

        index += 1;
    }

    if result.is_empty() {
        content
    }

    else {
        result.push(content[last_index..].to_vec());
        result.concat()
    }
}

pub fn render_math(content: &[u32]) -> Vec<u32> {
    Math::from_mdxt(content).to_math_ml(true)
}
