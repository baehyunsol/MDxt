use yaml_rust::Yaml;

pub struct RenderResult {
    pub content: String,
    pub has_math: bool,
    pub metadata: Option<Yaml>
}