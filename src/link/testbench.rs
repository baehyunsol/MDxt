use super::normalize_link;
use crate::inline::InlineNode;
use crate::utils::into_v16;
use crate::escape::{escape_backslashes, render_backslash_escapes};
use crate::render::render_option::RenderOption;
use std::collections::HashMap;

fn samples() -> Vec<(String, String)> {  // (test_case, answer)

    // some cases are not the same as the [gfm's spec](https://github.github.com/gfm)
    let result = vec![
        ("[github](https://github.com)", "<a href=\"https://github.com\">github</a>"),
        ("[*github*](https://github.com)", "<a href=\"https://github.com\"><em>github</em></a>"),
        ("*[github](https://github.com)*", "<em><a href=\"https://github.com\">github</a></em>"),
        ("*[github*](https://github.com)", "<em>[github</em>](https://github.com)"),
        ("[github](*https://github.com*)", "<a href=\"\">github</a>"),
        ("[link] [no link] `[link]` [`link`][link]", "<a href=\"https://example\">link</a> [no link] <code class=\"short\">[link]</code> <a href=\"https://example\"><code class=\"short\">link</code></a>"),
        ("*[github](https://github.com*)", "<em>[github](https://github.com</em>)"),  // Add this to the document

        ("[invalid url](*bold*~subscript~)", "<a href=\"\">invalid url</a>"),

        ("[[macro]](https://github.com)", "[[macro]](https://github.com)"),
        ("[not [macro]](https://github.com)", "<a href=\"https://github.com\">not [macro]</a>"),
        ("[not [macro], but *bold*](https://github.com)", "<a href=\"https://github.com\">not [macro], but <em>bold</em></a>"),
        ("[no nested [link]](https://github.com)", "[no nested <a href=\"https://example\">link</a>](https://github.com)"),
        ("[no nested [link]][link]", "[no nested <a href=\"https://example\">link</a>]<a href=\"https://example\">link</a>"),

        ("[link]", "<a href=\"https://example\">link</a>"),
        ("[link][]", "<a href=\"https://example\">link</a>"),
        ("[link]()", "<a href=\"\">link</a>"),
        ("[valid_link][link]", "<a href=\"https://example\">valid_link</a>"),
        ("[link][invalid_link]", "[link][invalid_link]"),
        ("[link][[macro]]", "[link][[macro]]"),
        ("[link](https://github.com)", "<a href=\"https://github.com\">link</a>"),
        ("[invalid_link]", "[invalid_link]"),

        ("![github](https://github.com)", "<img src=\"https://github.com\" alt=\"github\"/>"),
        ("![*github*](https://github.com)", "<img src=\"https://github.com\" alt=\"*github*\"/>"),
        ("*![github](https://github.com)*", "<em><img src=\"https://github.com\" alt=\"github\"/></em>"),
        ("*![github*](https://github.com)", "<em>![github</em>](https://github.com)"),
        ("![github](*https://github.com*)", "<img src=\"\" alt=\"github\"/>"),
        ("![link] ![no link] `![link]` ![`link`][link]", "<img src=\"https://example\" alt=\"link\"/> ![no link] <code class=\"short\">![link]</code> <img src=\"https://example\" alt=\"`link`\"/>"),
        ("*![github](https://github.com*)", "<em>![github](https://github.com</em>)"),  // Add this to the document

        ("![invalid url](*bold*~subscript~)", "<img src=\"\" alt=\"invalid url\"/>"),

        ("![[macro]](https://github.com)", "![[macro]](https://github.com)"),
        ("![not [macro]](https://github.com)", "<img src=\"https://github.com\" alt=\"not [macro]\"/>"),
        ("![not [macro], but *bold*](https://github.com)", "<img src=\"https://github.com\" alt=\"not [macro], but *bold*\"/>"),
        ("![no nested [link]](https://github.com)", "![no nested <a href=\"https://example\">link</a>](https://github.com)"),
        ("![no nested [link]][link]", "![no nested <a href=\"https://example\">link</a>]<a href=\"https://example\">link</a>"),

        ("![link]", "<img src=\"https://example\" alt=\"link\"/>"),
        ("![link][]", "<img src=\"https://example\" alt=\"link\"/>"),
        ("![link]()", "<img src=\"\" alt=\"link\"/>"),
        ("![valid_link][link]", "<img src=\"https://example\" alt=\"valid_link\"/>"),
        ("![link][invalid_link]", "![link][invalid_link]"),
        ("![link][[macro]]", "![link][[macro]]"),
        ("![link](https://github.com)", "<img src=\"https://github.com\" alt=\"link\"/>"),
        ("![invalid_link]", "![invalid_link]"),
    ];

    result.iter().map(|(case, answer)| (case.to_string(), answer.to_string())).collect()
}

#[test]
fn link_render_test() {

    let test_cases = samples();
    let mut failures = vec![];
    let mut link_references = HashMap::new();
    let mut footnote_references = HashMap::new();
    let mut render_option = RenderOption::default();

    link_references.insert(
        into_v16("link"), into_v16("https://example")
    );

    for (case, answer) in test_cases.iter() {
        let rendered = render_backslash_escapes(
            &InlineNode::from_md(&escape_backslashes(&into_v16(case)),
            &link_references,
            &footnote_references,
            &mut render_option).to_html()
        );

        if rendered != into_v16(answer) {
            failures.push(format!(
                "link_test: failed!! given md:  {}\ndesired html:  {}\nactual result:  {}",
                case,
                answer,
                String::from_utf16(&rendered).unwrap()
            ));
        }

    }

    if failures.len() > 0 {
        panic!(
            "Link render test: {} case(s) out of {} cases have failed!\n\n{}",
            failures.len(),
            test_cases.len(),
            failures.join("\n\n")
        );
    }

}

#[test]
fn normalize_link_test() {
    let cases_and_answers = vec![
        (into_v16("FOO"), into_v16("foo")),
        (into_v16("  F  OO "), into_v16("f oo"))
    ];

    for (case, answer) in cases_and_answers.into_iter() {
        assert_eq!(normalize_link(&case), answer);
    }

}