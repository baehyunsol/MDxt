use crate::ast::{AST, line::code_to_lines};
use crate::container::header::normalize_header;
use crate::escape::BACKSLASH_ESCAPE_OFFSET;
use crate::inline::macros::predicate::read_macro;
use crate::utils::into_v32;

impl AST {

    pub fn render_toc(&mut self) {
        let toc_mdxt = headers_to_toc_mdxt(&self.doc_data.headers, self.render_option.header_anchor);
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

fn headers_to_toc_mdxt(headers: &Vec<(usize, Vec<u32>)>, has_anchors: bool) -> Vec<u32> {
    let mut stack = [0; 6];
    let mut cursor = 0;

    let mut result = vec![];

    // into_v32("[[div, class=toc]]\n\n- !![[no bullet]]\n")
    result.push(vec![
        91, 91, 100, 105, 118, 44, 32,
        99, 108, 97, 115, 115, 61, 116, 111, 99, 93, 93,
        10, 10, 45, 32, 33, 33,
        91, 91, 110, 111, 32,
        98, 117, 108, 108, 101, 116, 93, 93, 10
    ]);

    for (level, content) in headers.iter() {
        let level = *level - 1;  // header's level starts with 1, but stack's index starts with 0

        if level == cursor {
            stack[level] += 1;
        }

        else if level > cursor {

            while cursor < level {
                cursor += 1;
                stack[cursor] = 1;
            }

            result.push(vec![' ' as u32; level * 2]);
            result.push(vec![45, 32, 33, 33, 91, 91, 110, 111, 32, 98, 117, 108, 108, 101, 116, 93, 93, 10]);  // into_v32("- !![[no bullet]]\n")
        }

        else {

            while cursor > level {
                stack[cursor] = 0;
                cursor -= 1;
            }

            stack[cursor] += 1;
        }

        let index_anchor = if has_anchors {
            vec![
                vec![91],  // into_v32("["),
                stack_to_index(&stack),
                vec![93, 40, 35],  // into_v32("](#"),
                normalize_header(&content),
                vec![41],  // into_v32(")"),
            ].concat()
        }

        else {
            stack_to_index(&stack)
        };

        let element = vec![
            vec![' ' as u32; level * 2],
            vec![45, 32],  // into_v32("- "),
            index_anchor,
            vec![32],  // into_v32(" ")
            remove_recursive_toc(&content),
            vec![10],  // into_v32("\n")
        ].concat();

        result.push(element);
    }

    result.push(vec![10, 91, 91, 47, 100, 105, 118, 93, 93, 10]);  // into_v32("\n[[/div]]\n")
    result.concat()
}

fn stack_to_index(stack: &[usize; 6]) -> Vec<u32> {
    let mut result = Vec::with_capacity(6);
    let mut index = 0;

    while index < 6 && stack[index] > 0 {
        result.push(into_v32(&format!("{}.", stack[index])));
        index += 1;
    }

    result.concat()
}

fn remove_recursive_toc(content: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(content.len() + 8);  // some extra capacity

    for (index, c) in content.iter().enumerate() {

        if *c == '[' as u32 {

            if let Some(inner_macro) = read_macro(content, index) {

                // into_v32("toc") -> [116, 111, 99]
                if inner_macro == &[116, 111, 99] {
                    result.push(*c + BACKSLASH_ESCAPE_OFFSET);
                }

                else {
                    result.push(*c);
                }

            }

            else {
                result.push(*c);
            }

        }

        else {
            result.push(*c);
        }

    }

    result
}