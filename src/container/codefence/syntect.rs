use crate::escape::escape_htmls;
use crate::utils::{from_v16, inclusive_split, into_v16};
use lazy_static::lazy_static;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    pub static ref THEME: Theme = {
        let theme_set = ThemeSet::load_defaults();

        theme_set.themes["base16-eighties.dark"].clone()
    };
}

pub fn highlight_syntax(content: &[u16], language: &[u16], class_prefix: &str) -> Vec<Vec<u16>> {

    #[cfg(test)]
    assert!(is_syntax_available(language));

    // it assumes that the given language is available
    let syntax_reference = SYNTAX_SET.find_syntax_by_token(&from_v16(language)).unwrap();
    let mut highlighter = HighlightLines::new(syntax_reference, &THEME);
    let mut result = vec![];

    for line_u16 in inclusive_split(&content, '\n' as u16).into_iter() {
        let mut line_u16 = line_u16.to_vec();

        let curr_line = &from_v16(&line_u16);

        match highlighter.highlight_line(curr_line, &SYNTAX_SET) {
            Ok(styled_line) => {
                result.push(styled_line.iter().map(
                    |(Style {foreground, ..}, content)|
                    classify_style_to_css(&foreground, content, class_prefix)
                ).collect::<Vec<Vec<u16>>>().concat());
            }
            Err(_) => {
                result.push(classify_style_to_css(&Color::WHITE, curr_line, class_prefix));
            }
        }

    }

    result
}

pub fn is_syntax_available(language: &[u16]) -> bool {
    SYNTAX_SET.find_syntax_by_token(&from_v16(language)).is_some()
}

fn classify_style_to_css(color: &Color, content: &str, class_prefix: &str) -> Vec<u16> {

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
            panic!("Uninitialized Color: (r: {} g: {} b: {})", r, g, b)
        } else {
            "white"
        }
    };

    let content_v16 = into_v16(content);

    if content_v16.len() == 0 || content_v16.iter().all(|c| *c == ' ' as u16 || *c == '\n' as u16) {
        content_v16
    }

    else {
        vec![
            into_v16(&format!("<span class=\"{}color-{}\">", class_prefix, color)),
            escape_htmls(&content_v16),
            into_v16("</span>")
        ].concat()
    }

}