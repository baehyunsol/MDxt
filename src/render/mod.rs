pub mod render_option;
pub mod render_result;

use crate::ast::{AST, line::code_to_lines};
use crate::container::metadata::parse_metadata;
use crate::escape::{escape_backslashes, escape_htmls, remove_invalid_characters, render_html_escapes};
use crate::utils::{from_v16, into_v16};
use render_option::RenderOption;
use render_result::RenderResult;

pub fn render_to_html_with_default_options(content: &String) -> String {
    render_to_html(content, RenderOption::default()).content
}

pub fn render_to_html(content: &String, mut options: RenderOption) -> RenderResult {
    let mut u16_content = into_v16(content);

    u16_content = remove_invalid_characters(&u16_content);
    u16_content = escape_backslashes(&u16_content);
    u16_content = escape_htmls(&u16_content);

    let mut metadata = None;

    let mut lines = code_to_lines(&u16_content);

    if options.parse_metadata {

        match parse_metadata(&lines) {
            Some((parsed_metadata, end_index)) => {
                metadata = Some(parsed_metadata);
                lines = lines[end_index + 1..].to_vec();
            },
            _ => {}
        }

    }

    let mut ast = AST::from_lines(lines, &mut options);

    let html = ast.to_html();

    let clean_html = render_html_escapes(&html);

    let fenced_code_contents = ast.doc_data.fenced_code_contents.iter().map(
        |(index, content)| (*index, from_v16(content))
    ).collect();

    RenderResult {
        content: from_v16(&clean_html),
        has_collapsible_table: ast.doc_data.has_collapsible_table,
        has_tooltip: ast.doc_data.tooltip_count > 0,
        metadata,
        fenced_code_contents
    }

}
