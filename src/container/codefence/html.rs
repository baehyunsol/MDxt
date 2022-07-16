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

        let opening = into_v16("<pre><code><table><tbody>");

        if self.copy_button {
            rows.push(
                into_v16(
                    &format!(
                        "<tr><td><button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard({})\">Copy</button></td></tr>",
                        self.index
                    )
                )
            );
        }

        let closing = into_v16("</tbody></table></code></pre>");

        vec![
            opening,
            rows.concat(),
            closing
        ].concat()
    }

}

fn render_line(line: &[u16], mut curr_line: usize, line_num: &Option<usize>, highlight: &Vec<usize>) -> Vec<u16> {

    let line_num = match line_num {
        None => {
            curr_line += 1;  // markdown index starts with 1, and Rust starts with 0.
            into_v16("<td>")
        },
        Some(n) => {
            curr_line += n;
            into_v16(&format!("<td class=\"index\">{}</td><td>", curr_line))
        }
    };

    let highlight = if highlight.contains(&curr_line) {
        " class=\"highlight\""
    } else {
        ""
    };

    vec![
        into_v16(&format!("<tr{}>", highlight)),
        line_num,
        render_backslash_escapes(line),
        into_v16("</td></tr>")
    ].concat()
}

pub fn copy_button_javascript(codes: &HashMap<usize, Vec<u16>>) -> String {

    #[cfg(test)]
    assert!(codes.len() > 0);

    let mut codes = codes.iter().map(
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