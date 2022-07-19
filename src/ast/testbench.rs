use crate::utils::{into_v16, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn mdxt_samples() -> Vec<(String, String)> {
    let result = vec![
        ("
[[red]]
Red
[[/red]]
", "
<div class=\"color_red\">
<p>Red</p>
</div>
"
), ("
[[red]]

Red

[[/red]]
", "
<div class=\"color_red\">
<p>Red</p>
</div>
"
), ("
[[red]]

Not Red
", "
<p>[[red]]</p>

<p>Not Red</p>
"
), ("
Interrupt Paragraph
[[red]]
Red
[[/red]]
", "
<p>
Interrupt Paragraph <span class=\"color_red\"> Red </span>
</p>
"
), ("
[[div, class=foo]]

foo class

[[/div]]
", "
<div class=\"foo\"><p>foo class</p></div>
"), ("
[[div, class=foo]]
foo class
[[/div]]
", "
<div class=\"foo\"><p>foo class</p></div>
"), ("
[[div, class=foo]]
foo class

[[/div]]
", "
<div class=\"foo\"><p>foo class</p></div>
"), ("
[[div, class=foo]]

foo class
[[/div]]
", "
<div class=\"foo\"><p>foo class</p></div>
"), (
"
[[br]]

[[blank]]

[[br]]
", "
<p><br/></p><p>&nbsp;</p><p><br/></p>
"
)
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn mdxt_test() {
    for (md, html) in mdxt_samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v16(&rendered)) != remove_whitespaces(&into_v16(html)) {
            panic!("{} \n\n {}", md, rendered);
        }

    }

}