use lazy_static::lazy_static;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::easy::HighlightLines;
use crate::utils::into_v16;

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
    let syntax_reference = SYNTAX_SET.find_syntax_by_token(&String::from_utf16_lossy(language)).unwrap();
    let mut highlighter = HighlightLines::new(syntax_reference, &THEME);

    for line_u16 in content.split(|c| *c == '\n' as u16) {
        let mut line_u16 = line_u16.to_vec();
        line_u16.push('\n' as u16);
        let curr_line = &String::from_utf16_lossy(&line_u16);
        let styled_line = highlighter.highlight(curr_line, &SYNTAX_SET);

        // render `styled_line` to css classes
        todo!()
        panic!("{:?}", styled_line);
    }

    panic!()
}

pub fn is_syntax_available(language: &[u16]) -> bool {
    SYNTAX_SET.find_syntax_by_token(&String::from_utf16_lossy(language)).is_some()
}
