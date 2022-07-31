use super::FencedCode;
use super::syntect::{is_syntax_available, highlight_syntax};
use crate::utils::{into_v16, from_v16};
use crate::escape::{render_backslash_escapes, undo_html_escapes};
use std::collections::HashMap;

// `<` in code, `\` in code
// with/without syntax highlights

impl FencedCode {

    pub fn to_html(&self) -> Vec<u16> {

        let mut rows = if is_syntax_available(&self.language) {
            let lines = highlight_syntax(&undo_html_escapes(&self.content), &self.language);

            lines.iter().enumerate().map(
                |(index, line)| render_line(line, index, &self.line_num, &self.highlights)
            ).collect::<Vec<Vec<u16>>>()
        } else {
            self.content.split(|c| *c == '\n' as u16).enumerate().map(
                |(index, line)| render_line(line, index, &self.line_num, &self.highlights)
            ).collect::<Vec<Vec<u16>>>()
        };

        let copy_button = if self.copy_button {
            into_v16(
                &format!(
                    "<button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard({})\">Copy</button>",
                    self.index
                )
            )
        } else {
            vec![]
        };

        vec![
            into_v16("<pre><code>"),
            rows.concat(),
            into_v16("</code>"),
            copy_button,
            into_v16("</pre>"),
        ].concat()
    }

}

fn render_line(line: &[u16], mut curr_line: usize, line_num: &Option<usize>, highlights: &Vec<usize>) -> Vec<u16> {

    let line_num = match line_num {
        None => {
            curr_line += 1;  // markdown index starts with 1, and Rust starts with 0.
            into_v16("<span class=\"code_fence_code\">")
        },
        Some(n) => {
            curr_line += n;
            into_v16(&format!("<span class=\"code_fence_index\">{}</span><span class=\"code_fence_code\">", curr_line))
        }
    };

    let highlight_or_not = if highlights.contains(&curr_line) {
        " class=\"highlight code_fence_row\""
    } else {
        " class=\"code_fence_row\""
    };

    vec![
        into_v16(&format!("<span{}>", highlight_or_not)),
        line_num,
        render_backslash_escapes(line),
        into_v16("</span></span>\n")
    ].concat()
}

pub fn copy_button_javascript(codes: &HashMap<usize, Vec<u16>>) -> String {

    #[cfg(test)]
    assert!(codes.len() > 0);

    let codes = codes.iter().map(
        |(index, code)| (*index, from_v16(code))
    ).collect::<Vec<(usize, String)>>();

    let max_index = match codes.iter().map(|(index, _)| index).max() {
        None => 0,
        Some(n) => *n
    };

    let mut codes_array = vec![String::new(); max_index + 1];

    for (index, code) in codes.into_iter() {
        codes_array[index] = code;
    }

    let codes_array_formatted = format!(
        "[{}]",
        codes_array.iter().map(|c| format!("{:?}", c)).collect::<Vec<String>>().join(", ")
    );

    let result = format!("
const fenced_code_block_contents = {};

function copy_code_to_clipboard(index) {}
    navigator.clipboard.writeText(fenced_code_block_contents[index]);
{}",
    codes_array_formatted, "{", "}"
    );

    result
}