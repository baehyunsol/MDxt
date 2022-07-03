mod ast;
mod render;
mod utils;
mod inline;
mod escape;
mod link;
mod macros;
mod footnote;
mod math;

#[cfg(test)]
mod testbench;

pub use render::{
    render_to_html,
    render_to_html_with_default_options
};