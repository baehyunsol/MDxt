fn samples() -> Vec<(String, String)> {  // (test_case, answer)
    let result = vec![
        ("`*`*`*`, *`*`*`*", "<code class=\"short\">*</code><em><code class=\"short\">*</code>, *<code class=\"short\">*</code></em>`*"),
        ("`*italic in a codespan, which is not rendered*` *`codespan in an italic, which is rendered`*", "<code class=\"short\">*italic in a codespan, which is not rendered*</code> <em><code class=\"short\">codespan in an italic, which is rendered</code></em>"),
        ("**", "**"),
        ("***", "***"),
        ("****", "****"),
        ("~~deletion?~~~, ~~~deletion?~~", "WIP"),
        ("****abcde****", "*<em><strong>abcde</strong></em>*"),
        ("`a` `a`", "<code class=\"short\">a</code> <code class=\"short\">a</code>"),
        ("*abc*", "<em>abc</em>"),
        ("*abc**", "*abc**"),
        ("***abc**", "*<strong>abc</strong>"),
        ("****abc***", "*<em><strong>abc</strong></em>"),
        ("**abc***", "<strong>abc</strong>*"),
        ("*abc *", "*abc *"),
        ("*abc**def**ghi*", "<em>abc<strong>def</strong>ghi</em>"),
        ("*abc **def** ghi*", "<em>abc <strong>def</strong> ghi</em>"),
        ("*abc ** def ** ghi*", "<em>abc ** def ** ghi</em>"),
        ("*abc*def*", "<em>abc</em>def*"),
        ("*abc * def*", "<em>abc * def</em>"),
        ("*abc ** def*", "<em>abc ** def</em>"),
        ("**abc*def*ghi**", "<strong>abc<em>def</em>ghi</strong>"),
        ("*abc**def*ghi**", "<em>abc**def</em>ghi**"),
        ("*abc~~abcd~~abc*", "<em>abc<del>abcd</del>abc</em>"),
        ("*abc~~abcd*abc~~", "<em>abc~~abcd</em>abc~~"),
        ("*abc`abcd`abc*", "<em>abc<code class=\"short\">abcd</code>abc</em>"),
        ("*abc`abcd*abc`", "*abc<code class=\"short\">abcd*abc</code>"),
        ("*abc\\*", "*abc&#42;"),
        ("`abc\\` \\`abc`", "<code class=\"short\">abc&#96; &#96;abc</code>"),
        ("`a``b`", "<code class=\"short\">a``b</code>"),
        ("*italic* **bold** ~_underline_~ ~subscript~ ^superscript^ `codespan` ~~deletion~~", "<em>italic</em> <strong>bold</strong> WIP"),
        ("~_~~del_and_underline~~_~", "WIP"),
        ("~_no_underline _~", "~_no_underline _~"),
        ("[[red]]This text is red and **bold**.[[/red]] [[center]] Some whitespaces  [[/center]]", "WIP"),
        ("[[red]][[center]] Broken Macros! [[/cetner]]", "[[red]][[center]] Broken Macros! [[/cetner]]")
    ];

    result.iter().map(|(case, answer)| (case.to_string(), answer.to_string())).collect()
}

mod tests {

    #[test]
    fn inline_test() {
        use super::samples;
        use crate::inline::InlineNode;
        use crate::utils::into_v16;
        use crate::escape::{escape_backslashes, render_backslash_escapes};

        let mut failures = vec![];

        for (case, answer) in samples().iter() {

            if render_backslash_escapes(&InlineNode::from_md(&escape_backslashes(&into_v16(case))).to_html()) != into_v16(answer) {
                failures.push(format!(
                    "inline_test: failed!! given md:  {}\ndesired html:  {}\nactual result:  {}",
                    case,
                    answer,
                    String::from_utf16(&render_backslash_escapes(&InlineNode::from_md(&escape_backslashes(&into_v16(case))).to_html())).unwrap()
                ));
            }

        }

        if failures.len() > 0 {
            panic!(
                "Inline test: {} case(s) out of {} cases have failed!\n\n{}",
                failures.len(),
                samples().len(),
                failures.join("\n\n")
            );
        }

    }

}