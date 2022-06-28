pub struct RenderOption {
    link_handler: Box<dyn FnMut(&str) -> &str>
}

impl Default for RenderOption {

    fn default() -> Self {
        RenderOption {
            link_handler: Box::new(default_link_handler)
        }
    }

}

fn default_link_handler(link: &str) -> &str {
    todo!()
}