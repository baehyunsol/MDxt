mod ast;
mod render;
mod utils;
mod inline;
mod escape;
mod table;
mod codefence;
mod color;

#[cfg(test)]
mod testbench;

pub use render::{
    render_to_html,
    render_to_html_with_default_options
};

pub use color::{COLORS, Color};