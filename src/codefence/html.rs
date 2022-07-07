use super::FencedCode;
use super::syntect::{is_syntax_available, highlight_syntax};
use crate::utils::into_v16;
use crate::escape::{undo_html_escapes, undo_backslash_escapes};

// `<` in code, `\` in code
// with/withoud syntax highlights

impl FencedCode {

    pub fn to_html(&self) -> Vec<u16> {

        let rows = if is_syntax_available(&self.language) {
            let lines = highlight_syntax(&undo_backslash_escapes(&undo_html_escapes(&self.content)), &self.language);

            lines.iter().enumerate().map(
                |(index, line)| render_line(line, index, &self.line_num, &self.highlights)
            ).collect::<Vec<Vec<u16>>>()
        } else {
            self.content.split(|c| *c == '\n' as u16).enumerate().map(
                |(index, line)| render_line(line, index, &self.line_num, &self.highlights)
            ).collect::<Vec<Vec<u16>>>()
        };

        let opening = into_v16("<pre><code><table><tbody>");
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
        "class=\"highlight\""
    } else {
        ""
    };

    vec![
        into_v16(&format!("<tr {}>", highlight)),
        line_num,
        line.to_vec(),
        into_v16("</td></tr>")
    ].concat()
}