pub struct RenderOption {
    pub link_handler: Box<dyn FnMut(&[u16]) -> Vec<u16>>
}

impl Default for RenderOption {

    fn default() -> Self {
        RenderOption {
            link_handler: Box::new(default_link_handler)
        }
    }

}

fn default_link_handler(link: &[u16]) -> Vec<u16> {
    todo!()
}