use crate::link::predicate::is_valid_link_destination;

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

// TODO: block javascript execution
fn default_link_handler(link: &[u16]) -> Vec<u16> {

    if is_valid_link_destination(link) {
        link.to_vec()
    }

    else {
        vec![]
    }

}