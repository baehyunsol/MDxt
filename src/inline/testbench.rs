use crate::inline::InlineNode;
use crate::utils::{into_v16, from_v16};
use crate::escape::{escape_backslashes, render_backslash_escapes};
use crate::render::render_option::RenderOption;
use crate::ast::MdData;

fn samples() -> Vec<(String, String, bool)> {  // (test_case, answer, invertible)
    let result = vec![
        ("`*`*`*`, *`*`*`*", "<code class=\"short\">*</code><em><code class=\"short\">*</code>, *<code class=\"short\">*</code></em>`*", true),
        ("`*italic in a codespan, which is not rendered*` *`codespan in an italic, which is rendered`*", "<code class=\"short\">*italic in a codespan, which is not rendered*</code> <em><code class=\"short\">codespan in an italic, which is rendered</code></em>", true),
        ("^^super^^", "^<sup>super</sup>^", true),
        ("^a", "^a", true),
        ("", "", true), (" ", " ", true),
        ("^^", "^^", true),
        ("^ ^", "^ ^", true),
        ("^^^", "^^^", true),
        ("^\\^^", "<sup>&#94;</sup>", true),
        ("^^^^", "^^^^", true),
        ("~~", "~~", true),
        ("~ ~", "~ ~", true),
        ("~~~", "~~~", true),
        ("~\\~~", "<sub>&#126;</sub>", true),
        ("~~~~", "~~~~", true),
        ("**", "**", true),
        ("* *", "* *", true),
        ("***", "***", true),
        ("*\\**", "<em>&#42;</em>", true),
        ("****", "****", true),

        ("****abcde****", "*<em><strong>abcde</strong></em>*", true),
        ("`a` `a`", "<code class=\"short\">a</code> <code class=\"short\">a</code>", true),
        ("*abc*", "<em>abc</em>", true),
        ("*abc**", "*abc**", true),
        ("***abc**", "*<strong>abc</strong>", true),
        ("****abc***", "*<em><strong>abc</strong></em>", true),
        ("**abc***", "<strong>abc</strong>*", true),
        ("*abc *", "*abc *", true),
        ("*abc**def**ghi*", "<em>abc<strong>def</strong>ghi</em>", true),
        ("*abc **def** ghi*", "<em>abc <strong>def</strong> ghi</em>", true),
        ("*abc ** def ** ghi*", "<em>abc ** def ** ghi</em>", true),
        ("*abc*def*", "<em>abc</em>def*", true),
        ("*abc * def*", "<em>abc * def</em>", true),
        ("*abc ** def*", "<em>abc ** def</em>", true),
        ("**abc*def*ghi**", "<strong>abc<em>def</em>ghi</strong>", true),
        ("*abc**def*ghi**", "<em>abc**def</em>ghi**", true),
        ("*abc~~abcd~~abc*", "<em>abc<del>abcd</del>abc</em>", true),
        ("*abc~~abcd*abc~~", "<em>abc~~abcd</em>abc~~", true),
        ("*abc`abcd`abc*", "<em>abc<code class=\"short\">abcd</code>abc</em>", true),
        ("*abc`abcd*abc`", "*abc<code class=\"short\">abcd*abc</code>", true),
        ("*abc\\*", "*abc&#42;", true),

        ("`abc\\` \\`abc`", "<code class=\"short\">abc&#96; &#96;abc</code>", true),
        ("`a``b`", "<code class=\"short\">a``b</code>", true),
        ("*italic* **bold** ~_underline_~ ~subscript~ ^superscript^ `codespan` ~~deletion~~", "<em>italic</em> <strong>bold</strong> <u>underline</u> <sub>subscript</sub> <sup>superscript</sup> <code class=\"short\">codespan</code> <del>deletion</del>", true),
        ("~~deletion?~~~, ~~~deletion?~~", "<del>deletion?</del>~, <del>~deletion?</del>", true),
        ("~_~~del_and_underline~~_~", "<u><del>del_and_underline</del></u>", true),
        ("~~~_del_and_underline_~~~", "<del><u>del_and_underline</u></del>", true),
        ("~~~del_and_subscript~~~", "<del><sub>del_and_subscript</sub></del>", true),
        ("~~_underline_~~", "~<u>underline</u>~", true),
        ("~~_~underline_~~", "~<u>~underline</u>~", true),
        ("~_no_underline _~", "<sub>_no_underline _</sub>", true),

        ("[[]] [[ ]] empty macros", "[[]] [[ ]] empty macros", true),
        ("[[red]]This text is red and **bold**.[[/red]] [[center]] Some whitespaces  [[/center]]", "<div class=\"color_red\">This text is red and <strong>bold</strong>.</div> <div class=\"align_center\"> Some whitespaces  </div>", true),
        ("[[red]][[center]] Broken Macros! [[/cetner]]", "[[red]][[center]] Broken Macros! [[/cetner]]", true),
        ("[[char = 32]], [[char = 1307674368000]]", "&#32;, [[char = 1307674368000]]", false),
        ("[[red]][[center]]**This text is bold, center aligned and red.**[[/center]][[/red]]", "<div class=\"color_red\"><div class=\"align_center\"><strong>This text is bold, center aligned and red.</strong></div></div>", true),
        ("`[[red]]red in a codespan[[/red]]`, [[red]]`a codespan in red`[[/red]]", "<code class=\"short\">[[red]]red in a codespan[[/red]]</code>, <div class=\"color_red\"><code class=\"short\">a codespan in red</code></div>", true),
        ("[[math]] `a codespan inside a math` [[/math]] `[[math]] a math inside a codespan [[/math]]`", "\\( `a codespan inside a math` \\) <code class=\"short\">[[math]] a math inside a codespan [[/math]]</code>", true),
        ("`[[math]] a codespan before a math`[[/math]] [[math]] `a codespan after a math [[/math]]`", "<code class=\"short\">[[math]] a codespan before a math</code>[[/math]] \\( `a codespan after a math \\)`", true),
        ("[[math]] `a codespan after a math [[/math]]` `[[math]] a codespan before a math`[[/math]]", "\\( `a codespan after a math \\)` <code class=\"short\">[[math]] a codespan before a math</code>[[/math]]", true),
        ("[[math]] a * b * c = abc [[/math]]", "\\( a &#42; b &#42; c = abc \\)", false),
        ("[[highlight = red]] This text is highlighted! [[/highlight]]", "<div class=\"highlight_red\"> This text is highlighted! </div>", false),
        ("*inter-math inline element [[math]] F * G = int{-infty}{infty} F(theta)G(k - theta) d theta [[/math]]", "*inter-math inline element \\( F &#42; G = \\int\\limits _{-\\infty }^{\\infty } F(\\theta )G(k - \\theta ) d \\theta  \\)", false),
        ("[[highlight]] [[highlight = red]] [[/highlight]] [[highlight = invalid_color]] [[/highlight]]", "[[highlight]] <div class=\"highlight_red\"> </div> [[highlight = invalid_color]] [[/highlight]]", false)
    ];

    result.iter().map(|(case, answer, invertible)| (case.to_string(), answer.to_string(), *invertible)).collect()
}

#[test]
fn inline_render_test() {
    let test_cases = samples();
    let mut failures = vec![];
    let mut md_data = MdData::default();
    let mut render_option = RenderOption::default();

    for (case, answer, _) in test_cases.iter() {
        let rendered = render_backslash_escapes(
            &InlineNode::from_md(
                &escape_backslashes(&into_v16(case)),
                &mut md_data,
                &mut render_option
            ).to_html()
        );

        if rendered != into_v16(answer) {
            failures.push(format!(
                "inline_test: failed!! given md:  {}\ndesired html:  {}\nactual result:  {}",
                case,
                answer,
                from_v16(&rendered)
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

    assert!(md_data.has_math);
}

#[test]
fn inline_inversion_test() {
    let mut failures = vec![];
    let mut md_data = MdData::default();
    let mut render_option = RenderOption::default();

    for (case, _, invertible) in samples().iter() {

        if !invertible {
            continue;
        }

        let inverted = render_backslash_escapes(&InlineNode::from_md(
            &into_v16(case),
            &mut md_data,
            &mut render_option
        ).to_md());

        if inverted != into_v16(case) {
            failures.push(format!(
                "inline_test: failed!! given md:  {}\ninverted result:  {}",
                case,
                from_v16(&inverted)
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