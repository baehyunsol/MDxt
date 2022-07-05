use crate::inline::link::predicate::is_valid_link_destination;

#[derive(Clone)]
pub struct RenderOption {
    pub link_handler: fn(&[u16]) -> Vec<u16>,
    pub header_anchor: bool,
    pub is_macro_enabled: bool
}

impl Default for RenderOption {

    fn default() -> Self {
        RenderOption {
            link_handler: default_link_handler,
            header_anchor: true,
            is_macro_enabled: true
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