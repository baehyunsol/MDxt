use crate::inline::link::predicate::is_valid_link_destination;
use crate::container::header::normalize_header;
use crate::utils::into_v16;

#[derive(Clone)]
pub struct RenderOption {
    pub link_handler: fn(&[u16]) -> Vec<u16>,
    pub header_anchor: bool,
    pub is_macro_enabled: bool,
    pub has_metadata: bool
}

impl Default for RenderOption {

    fn default() -> Self {
        RenderOption {
            link_handler: default_link_handler,
            header_anchor: true,
            is_macro_enabled: true,
            has_metadata: true
        }
    }

}

// TODO: block javascript execution
fn default_link_handler(link: &[u16]) -> Vec<u16> {

    if is_valid_link_destination(link) {

        if link.len() > 0 && link[0] == '#' as u16 {
            vec![
                into_v16("#"),
                normalize_header(&link[1..])
            ].concat()
        }

        else {
            link.to_vec()
        }

    }

    else {
        vec![]
    }

}