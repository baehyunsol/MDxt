use crate::escape::escape_htmls;
use crate::utils::{from_v32, inclusive_split, into_v32};
use lazy_static::lazy_static;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = {
        let default_set = SyntaxSet::load_defaults_newlines();
        let mut syntax_set_builder = default_set.into_builder();

        if let Err(e) = syntax_set_builder.add_from_folder("./extra_syntaxes", true) {
            println!("Error while reading `./extra_syntaxes`: {e:?}");
        }

        syntax_set_builder.build()
    };
    pub static ref THEME: Theme = {
        let theme_set = ThemeSet::load_defaults();

        theme_set.themes["base16-eighties.dark"].clone()
    };
}

pub fn highlight_syntax(content: &[u32], language: &[u32], class_prefix: &str) -> Vec<Vec<u32>> {

    #[cfg(test)]
    assert!(is_syntax_available(language));

    // it assumes that the given language is available
    let syntax_reference = SYNTAX_SET.find_syntax_by_token(&from_v32(language)).unwrap();
    let mut highlighter = HighlightLines::new(syntax_reference, &THEME);
    let mut result = vec![];

    // it needs `\n` characters to highlight syntax properly (eg: without `\n`, single line comments don't work)
    for line_u32 in inclusive_split(&content, '\n' as u32).into_iter() {
        let line_u32 = line_u32.to_vec();

        let curr_line = &from_v32(&line_u32);
        let mut curr_stack = vec![];

        match highlighter.highlight_line(curr_line, &SYNTAX_SET) {
            Ok(styled_line) => {

                for (style, content) in styled_line.into_iter() {
                    let curr_stack_len = curr_stack.len();

                    if curr_stack_len == 0 {
                        curr_stack.push((style.foreground.clone(), content.to_string()));
                    }

                    else if curr_stack[curr_stack_len - 1].0 == style.foreground {
                        curr_stack[curr_stack_len - 1].1 = format!("{}{content}", curr_stack[curr_stack_len - 1].1);
                    }

                    else if content.chars().all(|c| c == ' ') {
                        curr_stack[curr_stack_len - 1].1 = format!("{}{content}", curr_stack[curr_stack_len - 1].1);
                    }

                    else {
                        curr_stack.push((style.foreground.clone(), content.to_string()));
                    }

                }

                result.push(curr_stack.iter().map(
                    |(color, content)|
                    classify_style_to_css(color, content, class_prefix)
                ).collect::<Vec<Vec<u32>>>().concat());
            }
            Err(_) => {
                result.push(classify_style_to_css(&Color::WHITE, curr_line, class_prefix));
            }
        }

    }

    result
}

pub fn is_syntax_available(language: &[u32]) -> bool {
    SYNTAX_SET.find_syntax_by_token(&from_v32(language)).is_some()
}

fn classify_style_to_css(color: &Color, content: &str, class_prefix: &str) -> Vec<u32> {

    // convert syntect's palette to its own
    let color = match color {
        Color { r: 211, g: 208, b: 200, .. } => "white",
        Color { r: 45, g: 45, b: 45, .. } => "dark",
        Color { r: 242, g: 119, b: 122, .. } => "red",
        Color { r: 116, g: 115, b: 105, .. } => "gray",
        Color { r: 204, g: 153, b: 204, .. } => "violet",
        Color { r: 102, g: 153, b: 204, .. } => "aqua",
        Color { r: 249, g: 145, b: 87, .. } => "gold",
        Color { r: 153, g: 204, b: 153, .. } => "green",
        Color { r: 102, g: 204, b: 204, .. } => "emerald",
        Color { r: 210, g: 123, b: 83, .. } => "pink",
        Color { r: 255, g: 204, b: 102, .. } => "grassgreen",
        Color { r, g, b, .. } => if cfg!(test) {
            panic!("Uninitialized Color: (r: {r} g: {g} b: {b})")
        } else {
            "white"
        }
    };

    let content_v32 = into_v32(content);

    // though we've included `\n` characters for the sake of proper syntax highlighting,
    // the characters have to be erased before it's rendered to html.
    // otherwise the result html will have redundant `\n`s.

    // it doesn't touch empty pieces
    if content_v32.len() == 0 {
        vec![]
    }

    // it doesn't touch empty pieces
    else if content_v32.iter().all(|c| *c == ' ' as u32 || *c == '\n' as u32) {
        content_v32.into_iter().filter(|c| *c != '\n' as u32).collect()
    }

    else {
        vec![
            into_v32(&format!("<span class=\"{class_prefix}color-{color}\">")),
            escape_htmls(&content_v32).into_iter().filter(|c| *c != '\n' as u32).collect(),
            vec![60, 47, 115, 112, 97, 110, 62],  // into_v32("</span>")
        ].concat()
    }

}