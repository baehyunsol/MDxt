mod ast;
mod render;
mod utils;
mod inline;
mod escape;
mod container;
mod color;

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
pub use inline::math::mathjax_javascript;