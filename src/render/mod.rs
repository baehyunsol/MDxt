pub mod render_option;
mod render_result;

use render_option::RenderOption;
use render_result::RenderResult;
use crate::escape::{escape_backslashes, escape_htmls};
use crate::utils::{into_v16, from_v16};
use crate::ast::{AST, line::code_to_lines};

pub fn render_to_html_with_default_options(content: &String) -> String {
    render_to_html(content, RenderOption::default()).content
}

pub fn render_to_html(content: &String, mut options: RenderOption) -> RenderResult {

    let mut u16_content = into_v16(content);
    u16_content = escape_backslashes(&u16_content);
    u16_content = escape_htmls(&u16_content);

    let lines = code_to_lines(&u16_content);
    let mut ast = AST::from_lines(lines, &mut options);

    let mut html = ast.to_html();

    #[cfg(test)]
    if html.iter().any(|c| *c > 60000) {
        panic!(
            "A character that's not supposed to be in the result is found\n{}",
            from_v16(&html)
        )
    }

    RenderResult {
        content: from_v16(&html),
        has_math: ast.md_data.has_math
    }

}
