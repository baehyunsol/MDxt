use crate::container::header::normalize_header;
use crate::ast::{AST, line::code_to_lines};
use crate::utils::into_v16;
use crate::escape::BACKSLASH_ESCAPE_MARKER;

impl AST {

    pub fn render_toc(&mut self) {

        let mut toc_mdxt = Vec::with_capacity(self.doc_data.headers.len());

        for (level, content) in self.doc_data.headers.iter() {

            // I know that `if { for { ... } } else { ... } ` is more efficient than `for { if { ... } ... }`,
            // I prefer more readable and pretty code
            let element = if self.render_option.header_anchor {
                header_to_link(&content)
            }

            else if content.len() != 0 {
                content.clone()
            } else {
                into_v16("[[blank]]")
            };

            toc_mdxt.push(vec![
                vec![' ' as u16; level * 2],
                into_v16("1. "),
                element
            ].concat());
        }

        let toc_mdxt = toc_mdxt.join(&['\n' as u16][..]);
        let lines = code_to_lines(&toc_mdxt);

        let mut result = AST::from_lines(lines, &self.render_option);
        result.doc_data = self.doc_data.clone();
        result.doc_data.has_toc = false;  // to prevent infinite recursion
        result.parse_inlines();
        result.doc_data.has_toc = self.doc_data.has_toc;

        self.doc_data = result.doc_data.clone();
        self.toc = result.nodes.clone();
    }

}

// it escapes all the square brackets in the content
// because the square brackets inside a link label are very likely to make problems
// ex: `## ]123` -> `    - []123](#123)` is not a valid link
fn header_to_link(content: &[u16]) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len() * 5 / 4);

    for c in content.iter() {

        if *c == '[' as u16 || *c == ']' as u16 {
            result.push(BACKSLASH_ESCAPE_MARKER);
            result.push(u16::MAX - c);
        }

        else {
            result.push(*c);
        }

    }

    vec![
        into_v16("["),
        result,
        into_v16("](#"),
        normalize_header(&content),
        into_v16(")")
    ].concat()
}