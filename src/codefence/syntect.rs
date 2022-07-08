use lazy_static::lazy_static;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme, Color, Style};
use syntect::easy::HighlightLines;
use crate::utils::{into_v16, from_v16};
use crate::escape::escape_htmls;

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    pub static ref THEME: Theme = {
        let theme_set = ThemeSet::load_defaults();

        theme_set.themes["base16-eighties.dark"].clone()
    };
}

pub fn highlight_syntax(content: &[u16], language: &[u16]) -> Vec<Vec<u16>> {

    #[cfg(test)]
    assert!(is_syntax_available(language));

    // it assumes that the given language is available
    let syntax_reference = SYNTAX_SET.find_syntax_by_token(&from_v16(language)).unwrap();
    let mut highlighter = HighlightLines::new(syntax_reference, &THEME);
    let mut result = vec![];

    for line_u16 in content.split(|c| *c == '\n' as u16) {
        let mut line_u16 = line_u16.to_vec();
        // line_u16.push('\n' as u16);  // can I remove this line?
        let curr_line = &from_v16(&line_u16);
        let styled_line = highlighter.highlight(curr_line, &SYNTAX_SET);

        result.push(styled_line.iter().map(
            |(Style {foreground, ..}, content)| classify_style_to_css(&foreground, content)
        ).collect::<Vec<Vec<u16>>>().concat());
    }

    result
}

pub fn is_syntax_available(language: &[u16]) -> bool {
    SYNTAX_SET.find_syntax_by_token(&from_v16(language)).is_some()
}

fn classify_style_to_css(color: &Color, content: &str) -> Vec<u16> {

    // convert syntect's palette to its own
    let color = match color {
        Color { r: 211, g: 208, b: 200, .. } => "white",
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

    vec![
        into_v16(&format!("<span class=\"color_{}\">", color)),
        escape_htmls(&into_v16(content)),
        into_v16("</span>")
    ].concat()
}