pub mod render_option;
mod render_result;

use render_option::RenderOption;
use render_result::RenderResult;
use crate::escape::{escape_backslashes, escape_htmls};
use crate::utils::into_v16;
use crate::ast::{AST, line::code_to_lines};

pub fn render(content: &String, mut options: RenderOption) -> RenderResult {

    let mut u16_content = into_v16(content);
    u16_content = escape_backslashes(&u16_content);
    u16_content = escape_htmls(&u16_content);

    let lines = code_to_lines(&u16_content);
    let mut ast = AST::from_lines(lines, &mut options);

    ast.parse_inlines(&mut options);
    todo!()

    /*
    #[cfg(test)]
    if result.iter().any(|c| c == BACKSLASH_ESCAPE_MARKER, CODESPAN_MARKER, ...) {
        panic!()
    }
    */
}