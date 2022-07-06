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
        <sup id=\"footnote_ref0\">
            <a href=\"#footnote_cite0\">
                [0]
            </a>
        </sup>
</p>
<p>
    b
        <sup id=\"footnote_ref1\">
            <a href=\"#footnote_cite0\">
                [1]
            </a>
        </sup>
        <sup id=\"footnote_ref2\">
            <a href=\"#footnote_cite1\">
                [2]
            </a>
        </sup>
    [^no]: A footnote cannot interrupt a paragraph!
</p>
<p>
    c
        <sup id=\"footnote_ref3\">
            <a href=\"#footnote_cite2\">
                [3]
            </a>
        </sup>
        <sup id=\"footnote_ref4\">
            <a href=\"#footnote_cite0\">
                [4]
            </a>
        </sup>
</p>
<p>
    d
        <sup id=\"footnote_ref5\">
            <a href=\"#footnote_cite1\">
                [5]
            </a>
        </sup>
        <sup id=\"footnote_ref6\">
            <a href=\"#footnote_cite3\">
                [6]
            </a>
        </sup>
</p>

<p>
    <a href=\"\"><sup>ff</sup></a>
</p>

<hr/>

<a id=\"footnote_cite0\"></a>
    0.
        <a href=\"#footnote_ref0\"> [0] </a>
        <a href=\"#footnote_ref1\"> [1] </a>
        <a href=\"#footnote_ref4\"> [4] </a>
        This is a <strong>Sample</strong> <em>Footnote</em>.<br/>
<a id=\"footnote_cite1\"></a>
    1.
        <a href=\"#footnote_ref2\"> [2] </a>
        <a href=\"#footnote_ref5\"> [5] </a>
        This is another <sup>sample</sup>!!<br/>
<a id=\"footnote_cite2\"></a>
    2.
        <a href=\"#footnote_ref3\"> [3] </a>
        This is another footnote.<br/><br/>
<a id=\"footnote_cite3\"></a>
    3.
        <a href=\"#footnote_ref6\"> [6] </a>
        Another one, that&apos;s not shadowed.<br/>
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