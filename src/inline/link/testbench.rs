use super::normalize_link_label;
use crate::utils::{into_v32, from_v32};

fn samples() -> Vec<(String, String)> {  // (test_case, answer)

    // some cases are not the same as the [gfm's spec](https://github.github.com/gfm)
    let result = vec![
        ("[]()", "<a href=\"\"></a>"),
        ("[ ]()", "<a href=\"\"> </a>"),
        ("[]( )", "<a href=\"\"></a>"),
        ("[ ]( )", "<a href=\"\"> </a>"),
        ("[] ()", "[] ()"),
        ("[ ] ()", "[ ] ()"),
        ("[] ( )", "[] ( )"),
        ("[ ] ( )", "[ ] ( )"),

        ("[github](https://github.com)", "<a href=\"https://github.com\">github</a>"),
        ("[*github*](https://github.com)", "<a href=\"https://github.com\"><em>github</em></a>"),
        ("*[github](https://github.com)*", "<em><a href=\"https://github.com\">github</a></em>"),
        ("*[github*](https://github.com)", "<em>[github</em>](<a href=\"https://github.com\">https://github.com</a>)"),
        ("[github](*https://github.com*)", "<a href=\"\">github</a>"),
        ("[link] [no link] `[link]` [`link`][link]", "<a href=\"https://example\">link</a> [no link] <code class=\"inline-code-span\">[link]</code> <a href=\"https://example\"><code class=\"inline-code-span\">link</code></a>"),
        ("*[github](https://github.com*)", "<em>[github](<a href=\"https://github.com\">https://github.com</a></em>)"),  // Add this to the document

        ("[invalid url](*bold*~subscript~)", "<a href=\"\">invalid url</a>"),

        ("[[macro]](https://github.com)", "[[macro]](<a href=\"https://github.com\">https://github.com</a>)"),
        ("[not [macro]](https://github.com)", "<a href=\"https://github.com\">not [macro]</a>"),
        ("[not [macro], but *bold*](https://github.com)", "<a href=\"https://github.com\">not [macro], but <em>bold</em></a>"),
        ("[no nested [link]](https://github.com)", "[no nested <a href=\"https://example\">link</a>](<a href=\"https://github.com\">https://github.com</a>)"),
        ("[no nested [link]][link]", "[no nested <a href=\"https://example\">link</a>]<a href=\"https://example\">link</a>"),

        ("[link]", "<a href=\"https://example\">link</a>"),
        ("[link][]", "<a href=\"https://example\">link</a>"),
        ("[link]()", "<a href=\"\">link</a>"),
        ("[valid_link][link]", "<a href=\"https://example\">valid_link</a>"),
        ("[link][invalid_link]", "[link][invalid_link]"),
        ("[link][[macro]]", "<a href=\"https://example\">link</a>[[macro]]"),
        ("[[macro]][link]", "[[macro]]<a href=\"https://example\">link</a>"),
        ("[link](https://github.com)", "<a href=\"https://github.com\">link</a>"),
        ("[invalid_link]", "[invalid_link]"),

        ("[[red]][link]", "[[red]]<a href=\"https://example\">link</a>"),
        ("[[red]][link](https://github.com)", "[[red]]<a href=\"https://github.com\">link</a>"),
        ("[[red]][link][[/red]]", "<span class=\"color-red\"><a href=\"https://example\">link</a></span>"),
        ("[[red]][link](https://github.com)[[/red]]", "<span class=\"color-red\"><a href=\"https://github.com\">link</a></span>"),

        ("[link][link2][link][link2]", "<a href=\"https://example2\">link</a><a href=\"https://example2\">link</a>"),

        ("![]()", "<img src=\"\" alt=\"\"/>"),
        ("![ ]()", "<img src=\"\" alt=\" \"/>"),
        ("![]( )", "<img src=\"\" alt=\"\"/>"),
        ("![ ]( )", "<img src=\"\" alt=\" \"/>"),
        ("![] ()", "![] ()"),
        ("![ ] ()", "![ ] ()"),
        ("![] ( )", "![] ( )"),
        ("![ ] ( )", "![ ] ( )"),
        ("! []()", "! <a href=\"\"></a>"),

        ("![github](https://github.com)", "<img src=\"https://github.com\" alt=\"github\"/>"),
        ("![*github*](https://github.com)", "<img src=\"https://github.com\" alt=\"*github*\"/>"),
        ("*![github](https://github.com)*", "<em><img src=\"https://github.com\" alt=\"github\"/></em>"),
        ("*![github*](https://github.com)", "<em>![github</em>](<a href=\"https://github.com\">https://github.com</a>)"),
        ("![github](*https://github.com*)", "<img src=\"\" alt=\"github\"/>"),
        ("![link] ![no link] `![link]` ![`link`][link]", "<img src=\"https://example\" alt=\"link\"/> ![no link] <code class=\"inline-code-span\">![link]</code> <img src=\"https://example\" alt=\"`link`\"/>"),
        ("*![github](https://github.com*)", "<em>![github](<a href=\"https://github.com\">https://github.com</a></em>)"),  // Add this to the document

        ("![invalid url](*bold*~subscript~)", "<img src=\"\" alt=\"invalid url\"/>"),

        ("![[macro]](https://github.com)", "![[macro]](<a href=\"https://github.com\">https://github.com</a>)"),
        ("![not [macro]](https://github.com)", "<img src=\"https://github.com\" alt=\"not [macro]\"/>"),
        ("![not [macro], but *bold*](https://github.com)", "<img src=\"https://github.com\" alt=\"not [macro], but *bold*\"/>"),
        ("![no nested [link]](https://github.com)", "![no nested <a href=\"https://example\">link</a>](<a href=\"https://github.com\">https://github.com</a>)"),
        ("![no nested [link]][link]", "![no nested <a href=\"https://example\">link</a>]<a href=\"https://example\">link</a>"),

        ("![link]", "<img src=\"https://example\" alt=\"link\"/>"),
        ("![link][]", "<img src=\"https://example\" alt=\"link\"/>"),
        ("![link]()", "<img src=\"\" alt=\"link\"/>"),
        ("![valid_link][link]", "<img src=\"https://example\" alt=\"valid_link\"/>"),
        ("![link][invalid_link]", "![link][invalid_link]"),
        ("![link][[macro]]", "<img src=\"https://example\" alt=\"link\"/>[[macro]]"),
        ("[[macro]]![link]", "[[macro]]<img src=\"https://example\" alt=\"link\"/>"),
        ("![link](https://github.com)", "<img src=\"https://github.com\" alt=\"link\"/>"),
        ("![invalid_link]", "![invalid_link]"),

        ("[[red]]![link]", "[[red]]<img src=\"https://example\" alt=\"link\"/>"),
        ("[[red]]![link](https://github.com)", "[[red]]<img src=\"https://github.com\" alt=\"link\"/>"),
        ("[[red]]![link][[/red]]", "<span class=\"color-red\"><img src=\"https://example\" alt=\"link\"/></span>"),
        ("[[red]]![link](https://github.com)[[/red]]", "<span class=\"color-red\"><img src=\"https://github.com\" alt=\"link\"/></span>"),

        ("[link]![link2][link][link2]", "<a href=\"https://example\">link</a><img src=\"https://example\" alt=\"link2\"/><a href=\"https://example2\">link2</a>"),
        ("[[[char=9650]]](#top)[[[char=9660]]](#bottom)", "<a href=\"#top\">&#9650;</a><a href=\"#bottom\">&#9660;</a>"),

        ("[Wikipedia: Polymorphism](https://en.wikipedia.org/wiki/Polymorphism_(computer_science))", "<a href=\"https://en.wikipedia.org/wiki/Polymorphism_(computer_science)\">Wikipedia: Polymorphism</a>"),

        ("[[br]][link][[br]]", "<br/><a href=\"https://example\">link</a><br/>")
    ];

    let links = "\n\n[link]: https://example\n\n[link2]: https://example2\n";

    result.iter().map(
        |(case, answer)| (
            format!("{case}{links}"),
            format!("<p>{answer}</p>"),
        )
    ).collect()
}

#[test]
fn link_render_test() {
    let test_cases = samples();
    let mut failures = vec![];

    for (case, answer) in test_cases.iter() {
        let rendered = into_v32(&crate::render_to_html_with_default_options(case));

        if rendered != into_v32(answer) {
            failures.push(format!(
                "link_test: failed!! given md:  {case}\ndesired html:  {answer}\nactual result:  {}",
                from_v32(&rendered)
            ));
        }

    }

    if !failures.is_empty() {
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
        (into_v32("FOO"), into_v32("foo")),
        (into_v32("  F  OO "), into_v32("f oo"))
    ];

    for (case, answer) in cases_and_answers.into_iter() {
        assert_eq!(normalize_link_label(&case), answer);
    }

}