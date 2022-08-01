use crate::render_to_html_with_default_options;
use crate::utils::{into_v16, remove_whitespaces};

fn samples() -> Vec<(String, String)> {
    vec![
        ("
a[^a]

b[^a][^b]
[^no]: A footnote cannot interrupt a paragraph!

c[^c][^a]

d[^b][^d]

[^ff^]

[^a]: This is a **Sample** *Footnote*.
[^b]: This is another ^sample^!!

[^c]: This is another footnote.[[br]]
[^d]: Another one, but shadowed
[^d]: Another one, that's not shadowed.
[^e]: It's never used.

[^ff^]: It's not a valid label for a footnote.
        ".to_string(), "
<p>
    a
        <sup id=\"footnote-ref-0\">
            <a href=\"#footnote-cite-0\">
                [0]
            </a>
        </sup>
</p>
<p>
    b
        <sup id=\"footnote-ref-1\">
            <a href=\"#footnote-cite-0\">
                [1]
            </a>
        </sup>
        <sup id=\"footnote-ref-2\">
            <a href=\"#footnote-cite-1\">
                [2]
            </a>
        </sup>
    [^no]: A footnote cannot interrupt a paragraph!
</p>
<p>
    c
        <sup id=\"footnote-ref-3\">
            <a href=\"#footnote-cite-2\">
                [3]
            </a>
        </sup>
        <sup id=\"footnote-ref-4\">
            <a href=\"#footnote-cite-0\">
                [4]
            </a>
        </sup>
</p>
<p>
    d
        <sup id=\"footnote-ref-5\">
            <a href=\"#footnote-cite-1\">
                [5]
            </a>
        </sup>
        <sup id=\"footnote-ref-6\">
            <a href=\"#footnote-cite-3\">
                [6]
            </a>
        </sup>
</p>

<p>
    <a href=\"\"><sup>ff</sup></a>
</p>

<hr class=\"footnote-hr\"/>
<p>
    <a id=\"footnote-cite-0\"></a>
        0.
            <a href=\"#footnote-ref-0\"> [0] </a>
            <a href=\"#footnote-ref-1\"> [1] </a>
            <a href=\"#footnote-ref-4\"> [4] </a>
            This is a <strong>Sample</strong> <em>Footnote</em>.<br/>
    <a id=\"footnote-cite-1\"></a>
        1.
            <a href=\"#footnote-ref-2\"> [2] </a>
            <a href=\"#footnote-ref-5\"> [5] </a>
            This is another <sup>sample</sup>!!<br/>
    <a id=\"footnote-cite-2\"></a>
        2.
            <a href=\"#footnote-ref-3\"> [3] </a>
            This is another footnote.<br/><br/>
    <a id=\"footnote-cite-3\"></a>
        3.
            <a href=\"#footnote-ref-6\"> [6] </a>
            Another one, that&apos;s not shadowed.<br/>
</p>
".to_string()),
    ]
}

#[test]
fn footnote_test() {
    for (md, html) in samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v16(&rendered)) != remove_whitespaces(&into_v16(html)) {
            panic!("{} \n\n {}", md, rendered);
        }

    }

}