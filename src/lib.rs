mod ast;
mod container;
mod color;
mod escape;
mod file_ext;
mod inline;
// mod obfuscate;  // WIP
mod render;
mod utils;

#[cfg(test)]
mod testbench;

#[cfg(test)]
pub const PRINT_TEST_PAGES: bool = false;  // use it only when you need `.md` files

pub use render::{
    render_to_html,
    render_to_html_with_default_options,
    render_option::RenderOption,
    render_result::RenderResult
};

// Don't hide these: see commit 4a01db5
pub use color::{Color, colors};

pub use container::table::macros::collapsible_table_javascript;
pub use inline::macros::tooltip::tooltip_javascript;

/// `Reference.md` is an example mdxt file.
/// See how it works.
pub fn render_reference() -> String {
    use std::fs::File;
    use std::io::Read;

    let mut s = String::new();
    let mut f = File::open("reference.md").unwrap();
    f.read_to_string(&mut s).unwrap();

    let raw_html = render_to_html_with_default_options(&s);

    let mut f = File::open("./styles/markdown.css").unwrap();
    let mut css = String::new();
    f.read_to_string(&mut css).unwrap();

    format!(
"
<!DOCTYPE html>
<html>
<head>
    <title>MDxt Reference</title>
    <style>{css}</style>
</head>
<body style=\"padding-left: 16px\">
    <article class=\"markdown\">{raw_html}</article>
</body>
</html>
",
    )
}