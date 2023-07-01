pub mod render_option;
pub mod render_result;

use crate::ast::{AST, line::code_to_lines};
use crate::container::metadata::parse_metadata;
use crate::escape::{escape_backslashes, preprocess, render_html_escapes};
use crate::utils::from_v32;
use render_option::RenderOption;
use render_result::RenderResult;

pub fn render_to_html_with_default_options(content: &str) -> String {
    render_to_html(content, RenderOption::default()).content
}

pub fn render_to_html(content: &str, mut options: RenderOption) -> RenderResult {
    let mut u32_content = preprocess(content);
    u32_content = escape_backslashes(&u32_content);

    let mut metadata = None;

    let mut lines = code_to_lines(&u32_content);

    if options.parse_metadata {

        if let Some((parsed_metadata, end_index)) = parse_metadata(&lines) {
            metadata = Some(parsed_metadata);
            lines = lines[end_index + 1..].to_vec();
        }

    }

    let mut ast = AST::from_lines(lines, &mut options);

    let html = ast.to_html();

    let clean_html = render_html_escapes(&html);

    let fenced_code_contents = ast.doc_data.fenced_code_contents.iter().map(
        |(index, content)| (*index, from_v32(content))
    ).collect();

    RenderResult {
        content: from_v32(&clean_html),
        has_collapsible_table: ast.doc_data.has_collapsible_table,
        has_tooltip: ast.doc_data.tooltip_count > 0,
        has_sidebar: !ast.sidebar.is_empty(),
        metadata,
        fenced_code_contents
    }

}
