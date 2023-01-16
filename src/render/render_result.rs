use std::collections::HashMap;
use yaml_rust::Yaml;

pub struct RenderResult {
    pub content: String,

    /// This flag tells you if the document has a collapsible table.
    pub has_collapsible_table: bool,

    /// This flag tells you if the document has a tooltip.
    pub has_tooltip: bool,

    pub metadata: Option<Yaml>,

    /// Some fenced codes have a `copy` button with them.
    /// Each button has an index of the fenced code.
    /// This hashmap maps the index and the content of the fenced code.
    /// Use this data when writing javascript for the copy buttons.
    pub fenced_code_contents: HashMap<usize, String>
}