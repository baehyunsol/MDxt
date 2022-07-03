use crate::escape::{undo_html_escapes, BACKSLASH_ESCAPE_MARKER};
use crate::utils::{into_v16, get_curly_brace_end_index, is_alphabet};
use lazy_static::lazy_static;

lazy_static! {
    static ref LATEX_SYMBOLS: Vec<Vec<u16>> = {
        LATEX_SYMBOLS_RAW.clone().into_iter().map(into_v16).collect()
    };

    static ref LATEX_1ARG_FUNCS: Vec<Vec<u16>> = {
        LATEX_1ARG_FUNCS_RAW.clone().into_iter().map(into_v16).collect()
    };

    static ref LATEX_2ARG_FUNCS: Vec<Vec<u16>> = {
        LATEX_2ARG_FUNCS_RAW.clone().into_iter().map(into_v16).collect()
    };
}

pub fn render_math(content: &[u16]) -> Vec<u16> {

    vec![
        into_v16("\\("),
        translate_to_latex(content),
        into_v16("\\)")
    ].concat()
}

pub fn translate_to_latex(content: &[u16]) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len());
    let mut curr_word = vec![];
    let mut index = 0;
    let mut content = escape_special_characters(content);  // `escape_special_characters` has to be called before any other inline parses, even before table parses
    content.push(' ' as u16);  // so that it handles the last word

    while index < content.len() {

        if is_alphabet(content[index]) {
            curr_word.push(content[index]);
        }

        else {

            if curr_word.len() > 0 {

                if LATEX_SYMBOLS.contains(&curr_word) {

                    if curr_word == into_v16("leftcurlybrace") {
                        result.push(into_v16("\\{ "));
                    }

                    else if curr_word == into_v16("rightcurlybrace") {
                        result.push(into_v16("\\} "));}

                    else {
                        result.push(
                            vec![
                                into_v16("\\"),
                                curr_word,
                                into_v16(" ")
                            ].concat()
                        );
                    }
                }

                else if LATEX_1ARG_FUNCS.contains(&curr_word) {
                    let (arguments, indexes) = get_arguments(&content, index, vec![], vec![]);

                    if arguments.len() > 0 {

                        if curr_word == into_v16("lim") {
                            let argument = translate_to_latex(&arguments[0]);

                            result.push(vec![
                                into_v16("\\lim\\limits_{"),
                                argument,
                                into_v16("}")
                            ].concat());
                        }

                        else {
                            let argument = if curr_word == into_v16("text") {
                                arguments[0].clone()
                            } else {
                                translate_to_latex(&arguments[0])
                            };

                            let func_name = if curr_word == into_v16("sup") {
                                into_v16("^")
                            }

                            else if curr_word == into_v16("sub") {
                                into_v16("_")
                            }

                            else {
                                vec![
                                    into_v16("\\"),
                                    curr_word
                                ].concat()
                            };

                            result.push(vec![
                                func_name,
                                into_v16("{"),
                                argument,
                                into_v16("}"),
                            ].concat());

                        }

                        curr_word = vec![];
                        index = indexes[0] + 1;
                    }

                    else {
                        result.push(curr_word);
                        result.push(vec![content[index]]);
                        curr_word = vec![];
                        index += 1;
                    }

                    continue;
                }

                else if LATEX_2ARG_FUNCS.contains(&curr_word) {
                    let (arguments, indexes) = get_arguments(&content, index, vec![], vec![]);

                    if arguments.len() == 0 {
                        result.push(curr_word);
                        result.push(vec![content[index]]);
                        curr_word = vec![];
                        index += 1;
                    }

                    else if arguments.len() == 1 {

                        if curr_word == into_v16("sqrt") {
                            let argument = translate_to_latex(&arguments[0]);

                            result.push(vec![
                                into_v16("\\sqrt{"),
                                argument,
                                into_v16("}")
                            ].concat());

                            curr_word = vec![];
                            index = indexes[0] + 1;
                        }

                        else {
                            result.push(curr_word);
                            result.push(vec![content[index]]);
                            curr_word = vec![];
                            index += 1;
                        }

                    }

                    else if arguments.len() >= 2 {
                        let argument1 = translate_to_latex(&arguments[0]);
                        let argument2 = translate_to_latex(&arguments[1]);

                        if curr_word == into_v16("frac") || curr_word == into_v16("cfrac") {
                            result.push(vec![
                                into_v16("\\"),
                                curr_word,
                                into_v16("{"),
                                argument1,
                                into_v16("}{"),
                                argument2,
                                into_v16("}")
                            ].concat());
                        }

                        else if curr_word == into_v16("sqrt") {
                            result.push(vec![
                                into_v16("\\"),
                                curr_word,
                                into_v16("["),
                                argument1,
                                into_v16("]{"),
                                argument2,
                                into_v16("}"),
                            ].concat());
                        }

                        else {
                            result.push(vec![
                                vec!['\\' as u16],
                                curr_word,
                                into_v16("\\limits _{"),
                                argument1,
                                into_v16("}^{"),
                                argument2,
                                into_v16("}"),
                            ].concat());
                        }

                        curr_word = vec![];
                        index = indexes[1] + 1;
                    }

                    continue;
                }

                else {
                    result.push(curr_word);
                }

                curr_word = vec![];
            }

            result.push(vec![content[index]]);
        }

        index += 1;
    }

    result.pop();  // it pops the space character that I temporalily pushed at the beginning of this function
    result.concat()
}


fn get_arguments(content: &Vec<u16>, index: usize, mut current_args: Vec<Vec<u16>>, mut arg_end_indexes: Vec<usize>) -> (Vec<Vec<u16>>, Vec<usize>) {

    if index >= content.len() {
        (current_args, arg_end_indexes)
    }

    else if content[index] == '{' as u16 {

        match get_curly_brace_end_index(content, index) {
            None => (current_args, arg_end_indexes),
            Some(i) => {
                current_args.push(content[index + 1..i].to_vec());
                arg_end_indexes.push(i);

                get_arguments(content, i + 1, current_args, arg_end_indexes)
            }
        }

    }

    else if content[index] == ' ' as u16 {
        get_arguments(content, index + 1, current_args, arg_end_indexes)
    }

    else {
        (current_args, arg_end_indexes)
    }

}

// This escape only works inside `[[math]]` macros
// I don't want other inline elements to interrupt math formulas.
fn escape_special_characters(content: &[u16]) -> Vec<u16> {

    let content = undo_html_escapes(content);
    let mut result = Vec::with_capacity(content.len() + content.len() / 6);

    for c in content.iter() {

        if *c == '<' as u16 {
            result.push(' ' as u16);
            result.push('l' as u16);
            result.push('t' as u16);
            result.push(' ' as u16);
        }

        else if *c == '>' as u16 {
            result.push(' ' as u16);
            result.push('g' as u16);
            result.push('t' as u16);
            result.push(' ' as u16);
        }

        else if into_v16("*~[|]^`").contains(c) {
            result.push(BACKSLASH_ESCAPE_MARKER);
            result.push(u16::MAX - c);
        }

        else {
            result.push(*c);
        }

    }

    result
}

const LATEX_SYMBOLS_RAW: [&str;103] = ["lt", "gt", "leq", "geq", "ll", "gg", "equiv", "subset", "supset", "approx", "in", "ni", "subseteq", "supseteq", "cong", "simeq", "notin", "propto", "neq", "therefore", "because", "pm", "mp", "times", "div", "star", "cap", "cup", "vee", "wedge", "cdot", "diamond", "bullet", "oplus", "ominus", "otimes", "oslash", "odot", "circ", "exists", "nexists", "forall", "neg", "land", "lor", "rightarrow", "leftarrow", "iff", "top", "bot", "varnothing", "quad", "backslash", "leftcurlybrace", "rightcurlybrace", "alpha", "beta", "gamma", "Gamma", "delta", "Delta", "epsilon", "zeta", "eta", "theta", "Theta", "iota", "kappa", "lambda", "Lambda", "mu", "nu", "xi", "Xi", "pi", "Pi", "rho", "sigma", "Sigma", "tau", "upsilon", "Upsilon", "phi", "Phi", "chi", "psi", "Psi", "omega", "Omega", "partial", "nabla", "infty", "cos", "sin", "tan", "cosh", "sinh", "tanh", "angle", "leftrightarrow", "sqcap", "sqcup", "space"];
const LATEX_1ARG_FUNCS_RAW: [&str;13] = ["sub", "sup", "hat", "bar", "dot", "tilde", "vec", "check", "overleftarrow", "overrightarrow", "underline", "text", "lim"];
const LATEX_2ARG_FUNCS_RAW: [&str;9] = ["frac", "cfrac", "sqrt", "int", "oint", "iint", "iiint", "sum", "prod"];

#[cfg(test)]
mod tests {

    #[test]
    fn math_test1() {
        let orig = crate::utils::into_v16("sub{sub{-3}}sup {-x}{4} neq infty text{1234}");
        let rendered = String::from_utf16_lossy(&crate::math::translate_to_latex(&orig));

        assert_eq!(rendered, "_{_{-3}}^{-x}{4} \\neq  \\infty  \\text{1234}".to_string());
    }

    #[test]
    fn math_test2() {
        let orig = crate::utils::into_v16("sqrt{1 + sqrt{2 + 3}} leq sqrt{3}{5 + frac   {2 + 5} {3 + 7}}");
        let rendered = String::from_utf16_lossy(&crate::math::translate_to_latex(&orig));

        assert_eq!(rendered, "\\sqrt{1 + \\sqrt{2 + 3}} \\leq  \\sqrt[3]{5 + \\frac{2 + 5}{3 + 7}}".to_string());
    }

    #[test]
    fn math_test3() {
        let orig = crate::utils::into_v16("frac {3} sum {3} sqrt {3}");
        let rendered = String::from_utf16_lossy(&crate::math::translate_to_latex(&orig));

        assert_eq!(rendered, "frac {3} sum {3} \\sqrt{3}".to_string());
    }

    #[test]
    fn math_test4() {
        let orig = crate::utils::into_v16("int{0}{infty} e sup{-x} dx");
        let rendered = String::from_utf16_lossy(&crate::math::translate_to_latex(&orig));

        assert_eq!(rendered, "\\int\\limits _{0}^{\\infty } e ^{-x} dx".to_string());
    }

    #[test]
    fn math_test5() {
        let orig = crate::utils::into_v16("(vec{a} neq vec {b}) = (hat{a} neq hat {b})");
        let rendered = String::from_utf16_lossy(&crate::math::translate_to_latex(&orig));

        assert_eq!(rendered, "(\\vec{a} \\neq  \\vec{b}) = (\\hat{a} \\neq  \\hat{b})".to_string());
    }

    #[test]
    fn math_test6() {
        let orig = crate::utils::into_v16("sum{n=1}{infty} frac{1}{n sup{2}} < 10");
        let rendered = String::from_utf16_lossy(&crate::math::translate_to_latex(&orig));

        assert_eq!(rendered, "\\sum\\limits _{n=1}^{\\infty } \\frac{1}{n ^{2}}  \\lt   10".to_string());
    }

    #[test]
    fn math_test7() {
        let orig = crate::utils::into_v16("text{delta} delta");
        let rendered = String::from_utf16_lossy(&crate::math::translate_to_latex(&orig));

        assert_eq!(rendered, "\\text{delta} \\delta ".to_string());
    }

}