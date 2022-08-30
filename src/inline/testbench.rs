use super::predicate::*;
use crate::inline::InlineNode;
use crate::utils::{into_v16, from_v16};
use crate::escape::escape_backslashes;
use crate::render::render_option::RenderOption;
use crate::ast::doc_data::DocData;

fn samples() -> Vec<(String, String)> {  // (test_case, answer)
    let result = vec![
        ("`*`*`*`, *`*`*`*", "<code class=\"short\">*</code><em><code class=\"short\">*</code>, *<code class=\"short\">*</code></em>`*"),
        ("`*italic in a code span, which is not rendered*` *`code span in an italic, which is rendered`*", "<code class=\"short\">*italic in a code span, which is not rendered*</code> <em><code class=\"short\">code span in an italic, which is rendered</code></em>"),
        ("^^super^^", "^<sup>super</sup>^"),
        ("^a", "^a"),
        ("", ""), (" ", " "),
        ("^^", "^^"),
        ("^ ^", "^ ^"),
        ("^^^", "^^^"),
        ("^\\^^", "<sup>&#94;</sup>"),
        ("^^^^", "^^^^"),
        ("~~", "~~"),
        ("~ ~", "~ ~"),
        ("~~~", "~~~"),
        ("~\\~~", "<sub>&#126;</sub>"),
        ("~~~~", "~~~~"),
        ("**", "**"),
        ("* *", "* *"),
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
        ("*abc``abcd``abc*", "<em>abc<code class=\"short\">abcd</code>abc</em>"),
        ("*abc`abcd*abc`", "*abc<code class=\"short\">abcd*abc</code>"),
        ("*abc``abcd*abc``", "*abc<code class=\"short\">abcd*abc</code>"),
        ("*abc\\*", "*abc&#42;"),

        ("``", "``"),
        ("` `", "<code class=\"short\"> </code>"),
        ("`abc\\` \\`abc`", "<code class=\"short\">abc\\</code> &#96;abc`"),
        ("`a``b`", "<code class=\"short\">a``b</code>"),
        ("`\\`", "<code class=\"short\">\\</code>"),
        ("`` ` `` `` ` ``", "<code class=\"short\">`</code> <code class=\"short\">`</code>"),
        ("``` `` ```", "<code class=\"short\">``</code>"),
        ("``` `code span?` ```", "<code class=\"short\">`code span?`</code>"),
        ("`\\no escape`", "<code class=\"short\">\\no escape</code>"),
        ("`no escape\\`", "<code class=\"short\">no escape\\</code>"),
        ("``no escape\\``", "<code class=\"short\">no escape\\</code>"),
        ("``not a code span`", "``not a code span`"),
        ("*italic* **bold** ~_underline_~ ~subscript~ ^superscript^ `code span` ~~deletion~~", "<em>italic</em> <strong>bold</strong> <u>underline</u> <sub>subscript</sub> <sup>superscript</sup> <code class=\"short\">code span</code> <del>deletion</del>"),
        ("~~deletion?~~~, ~~~deletion?~~", "<del>deletion?</del>~, <del>~deletion?</del>"),
        ("~_~~del_and_underline~~_~", "<u><del>del_and_underline</del></u>"),
        ("~~~_del_and_underline_~~~", "<del><u>del_and_underline</u></del>"),
        ("~~~del_and_subscript~~~", "<del><sub>del_and_subscript</sub></del>"),
        ("~~_underline_~~", "~<u>underline</u>~"),
        ("~~_~underline_~~", "~<u>~underline</u>~"),
        ("~_no_underline _~", "<sub>_no_underline _</sub>"),

        ("[[]] [[ ]] empty macros", "[[]] [[ ]] empty macros"),
        ("[[red]]This text is red and **bold**.[[/red]] [[center]] Some whitespaces  [[/center]]", "<span class=\"color-red\">This text is red and <strong>bold</strong>.</span> <span class=\"align-center\"> Some whitespaces  </span>"),
        ("[[red]][[center]] Broken Macros! [[/cetner]]", "[[red]][[center]] Broken Macros! [[/cetner]]"),
        ("[[char = 32]], [[char = 1307674368000]]", "&#32;, [[char = 1307674368000]]"),
        ("[[char = won]], [[char = euro]], [[char = therefore]]", "&#8361;, &euro;, &there4;"),
        ("[[red]][[center]]**This text is bold, center aligned and red.**[[/center]][[/red]]", "<span class=\"color-red\"><span class=\"align-center\"><strong>This text is bold, center aligned and red.</strong></span></span>"),
        ("`[[red]]red in a code span[[/red]]`, [[red]]`a code span in red`[[/red]]", "<code class=\"short\">[[red]]red in a code span[[/red]]</code>, <span class=\"color-red\"><code class=\"short\">a code span in red</code></span>"),
        ("[[math]] `codespan` [[/math]] `[[math]] codespan [[/math]]`", "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mo>`</mo><mi>codespan</mi><mo>`</mo></math> <code class=\"short\">[[math]] codespan [[/math]]</code>"),
        ("`[[math]] codespan` [[/math]] [[math]] `codespan [[/math]]`", "<code class=\"short\">[[math]] codespan</code> [[/math]] <math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mo>`</mo><mi>codespan</mi></math>`"),
        ("[[math]] `codespan [[/math]]` `[[math]] codespan` [[/math]]", "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mo>`</mo><mi>codespan</mi></math>` <code class=\"short\">[[math]] codespan</code> [[/math]]"),
        ("[[math]] a * b * c = abc [[/math]]", "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>a</mi><mo>*</mo><mi>b</mi><mo>*</mo><mi>c</mi><mo>=</mo><mi>abc</mi></math>"),
        ("[[math]] \\\\a + b [[/math]]", "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mo>\\</mo><mi>a</mi><mo>+</mo><mi>b</mi></math>"),
        ("[[highlight = red]] This text is highlighted! [[/highlight]]", "<span class=\"highlight-red\"> This text is highlighted! </span>"),
        ("*inter-math inline element [[math]] F * G [[/math]]", "*inter-math inline element <math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>F</mi><mo>*</mo><mi>G</mi></math>"),
        ("[[highlight]] [[highlight = red]] [[/highlight]] [[highlight = invalid_color]] [[/highlight]]", "[[highlight]] <span class=\"highlight-red\"> </span> [[highlight = invalid_color]] [[/highlight]]"),
        ("[[red]] [[big]] error [[/red]] [[/big]]", "<span class=\"color-red\"> [[big]] error </span> [[/big]]"),
        ("[[div, class = foo]] abc [[/div]]", "<div class=\"foo\"> abc </div>"),
        ("[[div, class = foo, id = bar, class = baz]] abc [[/div]]", "<div class=\"foo baz\" id=\"bar\"> abc </div>"),
        ("[[div, class = foo, onclick = malicious function]] abc [[/div]]", "[[div, class = foo, onclick = malicious function]] abc [[/div]]"),
        ("[[div]][[/div]], [[span]][[/span]], [[anchor]][[/anchor]], [[button]][[/button]], [[script]][[/script]]", "<div></div>, <span></span>, <a></a>, <button></button>, [[script]][[/script]]"),
        ("[[box, no border]] boxed [[/box]]", "<div class=\"box no-border\"> boxed </div>"),
        ("[[br]][[blank]]", "<br/>&nbsp;"),
        ("[[br=2]][[blank=3]]", "<br/><br/>&nbsp;&nbsp;&nbsp;"),
        ("[[br=]][[blank=y]]", "[[br=]][[blank=y]]"),
    ];

    result.iter().map(|(case, answer)| (case.to_string(), answer.to_string())).collect()
}

#[test]
fn inline_render_test() {
    let test_cases = samples();
    let mut failures = vec![];
    let mut doc_data = DocData::default();
    let mut render_option = RenderOption::default();

    for (case, answer) in test_cases.iter() {
        let rendered = InlineNode::from_mdxt(
            &escape_backslashes(&into_v16(case)),
            &mut doc_data,
            &mut render_option
        ).to_html(&[], "");

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

}

#[test]
fn inline_inversion_test() {
    let mut failures = vec![];
    let mut doc_data = DocData::default();
    let mut render_option = RenderOption::default();

    for (case, html) in samples().iter() {

        let inverted = InlineNode::from_mdxt(
            &escape_backslashes(&into_v16(case)),
            &mut doc_data,
            &mut render_option
        ).to_mdxt();

        let inverted_html = InlineNode::from_mdxt(
            &escape_backslashes(&inverted), &mut doc_data, &mut render_option
        ).to_html(&[], "");

        if into_v16(&html) != inverted_html {
            failures.push(format!(
                "inline_test: failed!! given md:  {}\ninverted md:  {}\ngiven html:  {}\ninverted html:  {}",
                case,
                from_v16(&inverted),
                html,
                from_v16(&inverted_html)
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

#[test]
fn predicate_test() {
    let code_span_samples = vec![
        ("`a`", Bool::True(2)),
        ("`*`*`", Bool::True(2)),
        ("`` a ``", Bool::True(6)),
        ("`a``b`", Bool::True(5))
    ];

    for (sample, answer) in code_span_samples.iter() {
        let tested = is_code_span(&into_v16(sample), 0);

        if &tested != answer {
            panic!("{} {:?} {:?}", sample, tested, answer);
        }

    }
}