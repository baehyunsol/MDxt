use crate::inline::InlineNode;
use crate::utils::into_v16;
use crate::escape::{escape_backslashes, render_backslash_escapes};
use crate::render::render_option::RenderOption;
use std::collections::HashMap;

fn samples() -> Vec<(String, String)> {  // (test_case, answer)
    let result = vec![
        ("`*`*`*`, *`*`*`*", "<code class=\"short\">*</code><em><code class=\"short\">*</code>, *<code class=\"short\">*</code></em>`*"),
        ("`*italic in a codespan, which is not rendered*` *`codespan in an italic, which is rendered`*", "<code class=\"short\">*italic in a codespan, which is not rendered*</code> <em><code class=\"short\">codespan in an italic, which is rendered</code></em>"),
        ("^^super^^", "^<sup>super</sup>^"),
        ("", ""), (" ", " "),
        ("^^", "^^"),
        ("^^^", "^^^"),
        ("^\\^^", "<sup>&#94;</sup>"),
        ("^^^^", "^^^^"),
        ("~~", "~~"),
        ("~~~", "~~~"),
        ("~\\~~", "<sub>&#126;</sub>"),
        ("~~~~", "~~~~"),
        ("**", "**"),
        ("***", "***"),
        ("*\\**", "<em>&#42;</em>"),
        ("****", "****"),

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
        ("*italic* **bold** ~_underline_~ ~subscript~ ^superscript^ `codespan` ~~deletion~~", "<em>italic</em> <strong>bold</strong> <u>underline</u> <sub>subscript</sub> <sup>superscript</sup> <code class=\"short\">codespan</code> <del>deletion</del>"),
        ("~~deletion?~~~, ~~~deletion?~~", "<del>deletion?</del>~, <del>~deletion?</del>"),
        ("~_~~del_and_underline~~_~", "<u><del>del_and_underline</del></u>"),
        ("~~~_del_and_underline_~~~", "<del><u>del_and_underline</u></del>"),
        ("~~~del_and_subscript~~~", "<del><sub>del_and_subscript</sub></del>"),
        ("~~_underline_~~", "~<u>underline</u>~"),
        ("~~_~underline_~~", "~<u>~underline</u>~"),
        ("~_no_underline _~", "<sub>_no_underline _</sub>"),

        ("[[red]]This text is red and **bold**.[[/red]] [[center]] Some whitespaces  [[/center]]", "<div class=\"color_red\">This text is red and <strong>bold</strong>.</div> <div class=\"align_center\"> Some whitespaces  </div>"),
        ("[[red]][[center]] Broken Macros! [[/cetner]]", "[[red]][[center]] Broken Macros! [[/cetner]]"),
        ("[[char = 32]], [[char = 1307674368000]]", "&#32;, [[char = 1307674368000]]")
    ];

    result.iter().map(|(case, answer)| (case.to_string(), answer.to_string())).collect()
}

#[test]
fn inline_render_test() {

    let test_cases = samples();
    let mut failures = vec![];
    let link_references = HashMap::new();
    let mut render_option = RenderOption::default();

    for (case, answer) in test_cases.iter() {
        let rendered = render_backslash_escapes(
            &InlineNode::from_md(&escape_backslashes(&into_v16(case)),
            &link_references,
            &mut render_option).to_html()
        );

        if rendered != into_v16(answer) {
            failures.push(format!(
                "inline_test: failed!! given md:  {}\ndesired html:  {}\nactual result:  {}",
                case,
                answer,
                String::from_utf16(&rendered).unwrap()
            ));
        }

    }

    if failures.len() > 0 {
        panic!(
            "Inline render test: {} case(s) out of {} cases have failed!\n\n{}",
            failures.len(),
            test_cases.len(),
            failures.join("\n\n")
        );
    }

}

#[test]
fn inline_inversion_test() {

    let mut failures = vec![];
    let link_references = HashMap::new();
    let mut render_option = RenderOption::default();

    for (case, _) in samples().iter() {
        let inverted = InlineNode::from_md(&into_v16(case), &link_references, &mut render_option).to_md();

        if inverted != into_v16(case) {
            failures.push(format!(
                "inline_test: failed!! given md:  {}\ninverted result:  {}",
                case,
                String::from_utf16(&inverted).unwrap()
            ));
        }

    }

    if failures.len() > 0 {
        panic!(
            "Inline inversion test: {} case(s) out of {} cases have failed!\n\n{}",
            failures.len(),
            samples().len(),
            failures.join("\n\n")
        );
    }

}