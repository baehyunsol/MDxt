use crate::ast::{AST, line::code_to_lines};
use crate::container::header::normalize_header;
use crate::escape::BACKSLASH_ESCAPE_OFFSET;
use crate::utils::into_v32;

impl AST {

    pub fn render_toc(&mut self) {

        let mut toc_mdxt = Vec::with_capacity(self.doc_data.headers.len());
        toc_mdxt.push(into_v32("\n[[div, class=toc]]\n"));

        for (level, content) in self.doc_data.headers.iter() {

            // I know that `if { for { ... } } else { ... } ` is more efficient than `for { if { ... } ... }`,
            // but I prefer more readable and pretty code
            let element = if self.render_option.header_anchor {
                header_to_link(&content)
            }

            else if content.len() != 0 {
                content.clone()
            } else {
                into_v32("[[blank]]")
            };

            toc_mdxt.push(vec![
                vec![' ' as u32; level * 2],
                into_v32("1. "),
                element
            ].concat());
        }

        toc_mdxt.push(into_v32("\n[[/div]]\n"));

        let toc_mdxt = toc_mdxt.join(&['\n' as u32][..]);
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
fn header_to_link(content: &[u32]) -> Vec<u32> {
    vec![
        into_v32("["),
        content.iter().map(
            |c| if *c == '[' as u32 || *c == ']' as u32 {
                BACKSLASH_ESCAPE_OFFSET + *c
            }
    
            else {
                *c
            }
        ).collect::<Vec<u32>>(),
        into_v32("](#"),
        normalize_header(&content),
        into_v32(")")
    ].concat()
}