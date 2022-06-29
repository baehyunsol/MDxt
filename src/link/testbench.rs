use super::normalize_link;
use crate::inline::InlineNode;
use crate::utils::into_v16;
use crate::escape::{escape_backslashes, render_backslash_escapes};
use crate::render::render_option::RenderOption;
use std::collections::HashMap;

fn samples() -> Vec<(String, String)> {  // (test_case, answer)
    let result = vec![
        ("[github](https://github.com)", "<a href=\"https://github.com\">github</a>"),
    ];

    result.iter().map(|(case, answer)| (case.to_string(), answer.to_string())).collect()
}

#[test]
fn link_render_test() {

    let test_cases = samples();
    let mut failures = vec![];

    for (case, answer) in test_cases.iter() {
        let rendered = render_backslash_escapes(
            &InlineNode::from_md(&escape_backslashes(&into_v16(case)),
            &HashMap::new(),
            &mut RenderOption::default()).to_html()
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