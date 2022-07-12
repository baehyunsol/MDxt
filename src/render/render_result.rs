use yaml_rust::Yaml;

pub struct RenderResult {
    pub content: String,
    pub has_math: bool,
    pub has_collapsible_table: bool,
    pub metadata: Option<Yaml>
}