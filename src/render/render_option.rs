use crate::container::header::normalize_header;
use crate::inline::link::predicate::is_valid_link_destination;
use crate::utils::{from_v32, into_v32};

#[derive(Clone)]
pub struct RenderOption {
    pub class_prefix: String,
    pub enable_youtube: bool,
    pub footnote_tooltip: bool,
    pub header_anchor: bool,
    pub javascript_for_collapsible_tables: bool,
    pub javascript_for_copy_buttons: bool,
    pub javascript_for_tooltips: bool,
    pub javascript_for_sidebar: bool,
    pub link_handler: fn(&str) -> String,
    pub parse_metadata: bool,
    pub xml: bool
}

impl Default for RenderOption {

    fn default() -> Self {
        RenderOption {
            class_prefix: String::new(),
            enable_youtube: true,
            footnote_tooltip: false,
            header_anchor: true,
            javascript_for_collapsible_tables: true,
            javascript_for_copy_buttons: true,
            javascript_for_tooltips: true,
            javascript_for_sidebar: true,
            link_handler: |s| s.to_string(),
            parse_metadata: true,
            xml: false
        }
    }

}

impl RenderOption {

    /// when rendering `[Lable](Link)` to html, `Link` goes through this function.
    /// The link first goes through the default handler then the given function.
    /// If you don't give any function, it'll only go through the default handler.
    /// The default handler does two things
    /// - reject links with invalid characters
    /// - normalize anchors
    ///  - make alphabets lowercase
    ///  - replace whitespaces with dashes
    pub fn set_link_handler(&mut self, link_handler: fn(&str) -> String) -> &mut Self {
        self.link_handler = link_handler;
        self
    }

    /// give `id` attributes to header tags
    pub fn set_header_anchor(&mut self, header_anchor: bool) -> &mut Self {
        self.header_anchor = header_anchor;
        self
    }

    /// It prefixes all the html classes.
    pub fn set_class_prefix(&mut self, class_prefix: String) -> &mut Self {
        self.class_prefix = class_prefix;
        self
    }

    /// If you want the output to be a well-formed xml, turn it on.
    /// It's not a polyglot markup, though.
    pub fn well_formed_xml(&mut self, well_formed_xml: bool) -> &mut Self {
        self.xml = well_formed_xml;
        self
    }

    pub fn parse_metadata(&mut self, parse_metadata: bool) -> &mut Self {
        self.parse_metadata = parse_metadata;
        self
    }

    /// It embeds javascript for collapsible tables in a `<script>`.
    pub fn embed_js_for_collapsible_tables(&mut self, javascript: bool) -> &mut Self {
        self.javascript_for_collapsible_tables = javascript;
        self
    }

    /// It embeds javascript for a sidebar in a `<script>`.
    pub fn embed_js_for_sidebar(&mut self, javascript: bool) -> &mut Self {
        self.javascript_for_sidebar = javascript;
        self
    }

    /// It embeds javascript for copy buttons of fenced code blocks in a `<script>`.
    pub fn embed_js_for_copy_buttons(&mut self, javascript: bool) -> &mut Self {
        self.javascript_for_copy_buttons = javascript;
        self
    }

    /// It embeds javascript for tooltips in a `<script>`.
    pub fn embed_js_for_tooltips(&mut self, javascript: bool) -> &mut Self {
        self.javascript_for_tooltips = javascript;
        self
    }

    pub fn embed_js_all(&mut self, javascript: bool) -> &mut Self {
        self.javascript_for_collapsible_tables = javascript;
        self.javascript_for_copy_buttons = javascript;
        self.javascript_for_tooltips = javascript;
        self.javascript_for_sidebar = javascript;

        self
    }

    /// Shows a tooltip message when hovering over a footnote ref.
    pub fn set_footnote_tooltip(&mut self, footnote_tooltip: bool) -> &mut Self {
        self.footnote_tooltip = footnote_tooltip;

        self
    }

    /// it's used internally by the engine
    pub fn handle_link(&self, link: &str) -> String {
        (self.link_handler)(&default_link_handler(link))
    }

}

// TODO: block javascript execution
fn default_link_handler(link: &str) -> String {

    let link_v32 = into_v32(link);

    if is_valid_link_destination(&link_v32) {

        if link_v32.len() > 0 && link_v32[0] == '#' as u32 {
            from_v32(&vec![
                vec![35],  // into_v32("#")
                normalize_header(&link_v32[1..])
            ].concat())
        }

        else {
            link.to_string()
        }

    }

    else {
        // if a mal-formed address is given, it rejects
        String::new()
    }

}