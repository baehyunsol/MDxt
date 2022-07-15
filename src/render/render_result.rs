use yaml_rust::Yaml;
use std::collections::HashMap;

pub struct RenderResult {
    pub content: String,
    pub has_math: bool,
    pub has_collapsible_table: bool,
    pub metadata: Option<Yaml>,

    /// Some fenced codes have a `copy` button with them.
    /// Each button has an index of the fenced code.
    /// This hashmap maps the index and the content of the fenced code.
    /// Use this data when writing javascript for the copy buttons.
    pub fenced_code_contents: HashMap<usize, String>
}