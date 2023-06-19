use super::FencedCode;
use super::syntect::{highlight_syntax, is_syntax_available};
use crate::escape::escape_htmls;
use crate::utils::{from_v32, into_v32, log10};
use std::collections::HashMap;

// `<` in code, `\` in code
// with/without syntax highlights

impl FencedCode {

    pub fn to_html(&self, class_prefix: &str) -> Vec<u32> {

        let rows = if is_syntax_available(&self.language) {
            let lines = highlight_syntax(&self.get_raw_content(), &self.language, class_prefix);

            lines.iter().enumerate().map(
                |(index, line)| render_line(line, index, &self.line_num, &self.highlights, class_prefix)
            ).collect::<Vec<Vec<u32>>>()
        } else {
            self.content.split(|c| *c == '\n' as u32).enumerate().map(
                |(index, line)| render_line(
                    &escape_htmls(line),  // see test case A in super::testbench::code_fence_samples()
                    index, &self.line_num, &self.highlights, class_prefix)
            ).collect::<Vec<Vec<u32>>>()
        };

        let copy_button = if self.copy_button {
            into_v32(
                &format!(
                    "<button class=\"{class_prefix}copy-fenced-code\" onclick=\"copy_code_to_clipboard({})\">Copy</button>",
                    self.index
                )
            )
        } else {
            vec![]
        };

        // so that each index has the same width
        let line_num_width = match self.line_num {
            None => String::new(),
            Some(n) => format!(" {class_prefix}line-num-width-{}", log10(n + rows.len()))
        };

        vec![
            into_v32(&format!("<pre class=\"{class_prefix}fenced-code-block{line_num_width}\"><code>")),
            rows.concat(),
            vec![60, 47, 99, 111, 100, 101, 62],  // into_v32("</code>"),
            copy_button,
            vec![60, 47, 112, 114, 101, 62],  // into_v32("</pre>"),
        ].concat()
    }

}

fn render_line(line: &[u32], mut curr_line: usize, line_num: &Option<usize>, highlights: &Vec<usize>, class_prefix: &str) -> Vec<u32> {

    let line_num = match line_num {
        None => {
            curr_line += 1;  // markdown index starts with 1, and Rust starts with 0.
            into_v32(&format!("<span class=\"{class_prefix}code-fence-code\">"))
        },
        Some(n) => {
            curr_line += n;
            into_v32(&format!("<span class=\"{class_prefix}code-fence-index\">{curr_line}</span><span class=\"{class_prefix}code-fence-code\">"))
        }
    };

    let highlight_or_not = if highlights.contains(&curr_line) {
        format!(" class=\"{class_prefix}highlight code-fence-row\"")
    } else {
        format!(" class=\"{class_prefix}code-fence-row\"")
    };

    vec![
        into_v32(&format!("<span{highlight_or_not}>")),
        line_num,
        line.to_vec(),

        // into_v32("</span></span>\n") -> [60, 47, 115, 112, 97, 110, 62, 60, 47, 115, 112, 97, 110, 62, 10]
        vec![60, 47, 115, 112, 97, 110, 62, 60, 47, 115, 112, 97, 110, 62, 10]
    ].concat()
}

pub fn copy_button_javascript(codes: &HashMap<usize, Vec<u32>>) -> String {

    #[cfg(test)]
    assert!(codes.len() > 0);

    let codes = codes.iter().map(
        |(index, code)| (*index, from_v32(code))
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
const fenced_code_block_contents = {codes_array_formatted};

function copy_code_to_clipboard(index) {}
    navigator.clipboard.writeText(fenced_code_block_contents[index]);
{}", "{", "}"
    );

    result
}