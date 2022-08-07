mod ast;
mod escape;
mod container;
mod color;
mod inline;
// mod obfuscate;  // WIP
mod render;
mod utils;

#[cfg(test)]
mod testbench;

pub use render::{
    render_to_html,
    render_to_html_with_default_options,
    render_option::RenderOption,
    render_result::RenderResult
};

pub use color::{COLORS, Color};
pub use container::table::macros::collapsible_table_javascript;

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
</head>
<body>
    <style>{}</style>
    <article class=\"markdown\">{}</article>
</body>
</html>
",
        css,
        raw_html
    )
}