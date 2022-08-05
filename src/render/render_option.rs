use crate::container::header::normalize_header;
use crate::inline::link::predicate::is_valid_link_destination;
use crate::utils::{from_v16, into_v16};

#[derive(Clone)]
pub struct RenderOption {

    /// when rendering `[Lable](Link)` to html, `Link` goes through this function.
    /// The link first goes through the default handler then the given function.
    /// If you don't give any function, it'll only go through the default handler.
    /// The default handler does two things
    /// - reject links with invalid characters
    /// - normalize anchors
    ///  - make alphabets lowercase
    ///  - replace whitespaces with dashes
    pub link_handler: fn(&str) -> String,

    /// give `id` attributes to header tags
    pub header_anchor: bool,
    pub parse_metadata: bool,

    /// Javascript is required to render collapsible tables and math formulas.
    /// If this option is true, the engine will add javascript codes when needed.
    /// If you want to use your own script, turn this option off.
    pub javascript: bool,

    /// It prefixes all the html classes.
    pub class_prefix: String,

    /// If you want the output to be a well-formed xml, turn it on.
    /// It's not a polyglot markup, though.
    pub xml: bool
}

impl Default for RenderOption {

    fn default() -> Self {
        RenderOption {
            link_handler: |s| s.to_string(),
            header_anchor: true,
            parse_metadata: true,
            javascript: true,
            class_prefix: String::new(),
            xml: false
        }
    }

}

impl RenderOption {

    pub fn set_link_handler(&mut self, link_handler: fn(&str) -> String) -> &mut Self {
        self.link_handler = link_handler;
        self
    }

    pub fn set_header_anchor(&mut self, header_anchor: bool) -> &mut Self {
        self.header_anchor = header_anchor;
        self
    }

    pub fn set_class_prefix(&mut self, class_prefix: String) -> &mut Self {
        self.class_prefix = class_prefix;
        self
    }

    pub fn set_xml(&mut self, xml: bool) -> &mut Self {
        self.xml = xml;
        self
    }

    pub fn enable_metadata(&mut self, parse_metadata: bool) -> &mut Self {
        self.parse_metadata = parse_metadata;
        self
    }

    pub fn enable_javascript(&mut self, javascript: bool) -> &mut Self {
        self.javascript = javascript;
        self
    }

    /// it's used internally by the engine
    pub fn handle_link(&self, link: &str) -> String {
        (self.link_handler)(&default_link_handler(link))
    }

}

// TODO: block javascript execution
fn default_link_handler(link: &str) -> String {

    let link_v16 = into_v16(link);

    if is_valid_link_destination(&link_v16) {

        if link_v16.len() > 0 && link_v16[0] == '#' as u16 {
            from_v16(&vec![
                into_v16("#"),
                normalize_header(&link_v16[1..])
            ].concat())
        }

        else {
            link.to_string()
        }

    }

    else {
        String::new()
    }

}