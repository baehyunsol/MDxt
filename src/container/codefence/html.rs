use super::FencedCode;
use super::syntect::{highlight_syntax, is_syntax_available};
use crate::utils::{from_v16, into_v16, log10};
use std::collections::HashMap;

// `<` in code, `\` in code
// with/without syntax highlights

// TODO: inline::testbench::check_validity_(FencedCode.content) 

impl FencedCode {

    pub fn to_html(&self, class_prefix: &str) -> Vec<u16> {

        let rows = if is_syntax_available(&self.language) {
            let lines = highlight_syntax(&self.get_raw_content(), &self.language, class_prefix);

            lines.iter().enumerate().map(
                |(index, line)| render_line(line, index, &self.line_num, &self.highlights, class_prefix)
            ).collect::<Vec<Vec<u16>>>()
        } else {
            self.content.split(|c| *c == '\n' as u16).enumerate().map(
                |(index, line)| render_line(line, index, &self.line_num, &self.highlights, class_prefix)
            ).collect::<Vec<Vec<u16>>>()
        };

        let copy_button = if self.copy_button {
            into_v16(
                &format!(
                    "<button class=\"{}copy-fenced-code\" onclick=\"copy_code_to_clipboard({})\">Copy</button>",
                    class_prefix,
                    self.index
                )
            )
        } else {
            vec![]
        };

        // so that each index has the same width
        let line_num_width = match self.line_num {
            None => String::new(),
            Some(n) => format!(" {}line-num-width-{}", class_prefix, log10(n + rows.len()))
        };

        vec![
            into_v16(&format!("<pre class=\"{}fenced-code-block{}\"><code>", class_prefix, line_num_width)),
            rows.concat(),
            into_v16("</code>"),
            copy_button,
            into_v16("</pre>"),
        ].concat()
    }

}

fn render_line(line: &[u16], mut curr_line: usize, line_num: &Option<usize>, highlights: &Vec<usize>, class_prefix: &str) -> Vec<u16> {

    let line_num = match line_num {
        None => {
            curr_line += 1;  // markdown index starts with 1, and Rust starts with 0.
            into_v16(&format!("<span class=\"{}code-fence-code\">", class_prefix))
        },
        Some(n) => {
            curr_line += n;
            into_v16(&format!("<span class=\"{}code-fence-index\">{}</span><span class=\"{}code-fence-code\">", class_prefix, curr_line, class_prefix))
        }
    };

    let highlight_or_not = if highlights.contains(&curr_line) {
        format!(" class=\"{}highlight code-fence-row\"", class_prefix)
    } else {
        format!(" class=\"{}code-fence-row\"", class_prefix)
    };

    vec![
        into_v16(&format!("<span{}>", highlight_or_not)),
        line_num,
        line.to_vec(),
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