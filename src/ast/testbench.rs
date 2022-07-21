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
), ("
[[giant]]

[[center]]

Giant and Center

[[/center]]

[[/giant]]
", "
<div class=\"size_giant\">
    <div class=\"align_center\">
        <p>Giant and Center</p>
    </div>
</div>
"), ("
[[giant]]

[[center]]

Macros are not properly closed.

[[/giant]]

[[/center]]
", "
<p>[[giant]]</p>
<p>[[center]]</p>
<p>Macros are not properly closed.</p>
<p>[[/giant]]</p>
<p>[[/center]]</p>
"), ("
[[highlight = red]]

[[span, class = foo, id = bar]]

Red Span

[[/span]]

[[/highlight]]
", "
<div class=\"highlight_red\">
    <span class=\"foo\" id=\"bar\">
        <p>Red Span</p>
    </span>
</div>
"), ("
[[box]]

A paragraph in a box.

[[box]]

A paragraph in a box in a box.

[[/box]]

[[box, no border]]

A paragraph in a borderless box in a box.

[[/box]]

[[/box]]
", "
<div class=\"box\">
    <p>A paragraph in a box.</p>
    <div class=\"box\">
        <p>A paragraph in a box in a box.</p>
    </div>
    <div class=\"box no-border\">
        <p>A paragraph in a borderless box in a box.</p>
    </div>
</div>
")
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