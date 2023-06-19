use crate::render_to_html_with_default_options;
use crate::utils::{into_v32, remove_whitespaces};

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
[^d]: Another one, but shadowed.
[^d]: Another one, that's not shadowed.
[^e]: It's never used.

[^ff^]: It's not a valid label for a footnote.
".to_string(), "
<p>
    a
        <span class=\"footnote-ref\" id=\"footnote-ref-0\">
            <a href=\"#footnote-cite-0\">
                [0]
            </a>
        </span>
</p>
<p>
    b
        <span class=\"footnote-ref\" id=\"footnote-ref-1\">
            <a href=\"#footnote-cite-0\">
                [1]
            </a>
        </span>
        <span class=\"footnote-ref\" id=\"footnote-ref-2\">
            <a href=\"#footnote-cite-1\">
                [2]
            </a>
        </span>
    [^no]: A footnote cannot interrupt a paragraph!
</p>
<p>
    c
        <span class=\"footnote-ref\" id=\"footnote-ref-3\">
            <a href=\"#footnote-cite-2\">
                [3]
            </a>
        </span>
        <span class=\"footnote-ref\" id=\"footnote-ref-4\">
            <a href=\"#footnote-cite-0\">
                [4]
            </a>
        </span>
</p>
<p>
    d
        <span class=\"footnote-ref\" id=\"footnote-ref-5\">
            <a href=\"#footnote-cite-1\">
                [5]
            </a>
        </span>
        <span class=\"footnote-ref\" id=\"footnote-ref-6\">
            <a href=\"#footnote-cite-3\">
                [6]
            </a>
        </span>
</p>

<p>
    <a href=\"\"><sup>ff</sup></a>
</p>

<hr class=\"footnote-hr\"/>

<div class=\"mdxt-footnote-cites\">
    <p>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-0\"></a>
                <a href=\"#footnote-ref-0\"> [0] </a>
                <a href=\"#footnote-ref-1\"> [1] </a>
                <a href=\"#footnote-ref-4\"> [4] </a>
                This is a <strong>Sample</strong> <em>Footnote</em>.
        </div>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-1\"></a>
                <a href=\"#footnote-ref-2\"> [2] </a>
                <a href=\"#footnote-ref-5\"> [5] </a>
                This is another <sup>sample</sup>!!
        </div>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-2\"></a>
                <a href=\"#footnote-ref-3\"> [3] </a>
                This is another footnote.<br/>
        </div>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-3\"></a>
                <a href=\"#footnote-ref-6\"> [6] </a>
                Another one, that&apos;s not shadowed.
        </div>
    </p>
</div>
".to_string()), ("
![^a]

[^a]: not an image.
".to_string(), "
<p>
    !<span class=\"footnote-ref\" id=\"footnote-ref-0\"><a href=\"#footnote-cite-0\">[0]</a></span>
</p>

<hr class=\"footnote-hr\"/>

<div class=\"mdxt-footnote-cites\">
    <p>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-0\"></a><a href=\"#footnote-ref-0\"> [0]</a> not an image.
        </div>
    </p>
</div>
".to_string()), (
"
a[^a] b[^b]

[^a]: a

[^a]: aa

[^b]: b

[^b]: bb
".to_string(), "
<p>
    a<span class=\"footnote-ref\" id=\"footnote-ref-0\"><a href=\"#footnote-cite-0\">[0]</a></span>
    b<span class=\"footnote-ref\" id=\"footnote-ref-1\"><a href=\"#footnote-cite-1\">[1]</a></span>
</p>

<hr class=\"footnote-hr\"/>

<div class=\"mdxt-footnote-cites\">
    <p>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-0\"></a><a href=\"#footnote-ref-0\"> [0]</a> aa
        </div>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-1\"></a><a href=\"#footnote-ref-1\"> [1]</a> bb
        </div>
    </p>
</div>
".to_string()
), ("
a[^A] b[^B] c[^C]

[^A]: Hi there!

[^B]: Hello there!

[^C]: Goodbye!

aa[^A] bb[^B]

[^A]: another A.

[^B]: another B.
".to_string(), "
<p>
    a<span class=\"footnote-ref\" id=\"footnote-ref-0\"><a href=\"#footnote-cite-0\">[0]</a></span>
    b<span class=\"footnote-ref\" id=\"footnote-ref-1\"><a href=\"#footnote-cite-1\">[1]</a></span>
    c<span class=\"footnote-ref\" id=\"footnote-ref-2\"><a href=\"#footnote-cite-2\">[2]</a></span>
</p>
<p>
    aa<span class=\"footnote-ref\" id=\"footnote-ref-3\"><a href=\"#footnote-cite-0\">[3]</a></span>
    bb<span class=\"footnote-ref\" id=\"footnote-ref-4\"><a href=\"#footnote-cite-1\">[4]</a></span>
</p>

<hr class=\"footnote-hr\"/>

<div class=\"mdxt-footnote-cites\">
    <p>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-0\"></a><a href=\"#footnote-ref-0\"> [0]</a> <a href=\"#footnote-ref-3\"> [3]</a> another A.
        </div>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-1\"></a><a href=\"#footnote-ref-1\"> [1]</a> <a href=\"#footnote-ref-4\"> [4]</a> another B.
        </div>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-2\"></a><a href=\"#footnote-ref-2\"> [2]</a> Goodbye!
        </div>
    </p>
</div>
".to_string())
    ]
}

#[test]
fn footnote_test() {
    for (md, html) in samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v32(&rendered)) != remove_whitespaces(&into_v32(html)) {
            panic!("{md} \n\n {rendered}");
        }

    }

}