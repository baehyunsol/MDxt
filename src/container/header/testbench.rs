use crate::utils::{into_v16, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn header_samples() -> Vec<(String, String)> {
    let result = vec![
        ("
# Header 1
## Header 2
### 

#### Header 4
##### Header 5
###### Header 6
####### Header 7

### *Header*", "
<h1 id=\"header-1\">Header 1</h1>
<h2 id=\"header-2\">Header 2</h2>
<h3></h3>

<h4 id=\"header-4\">Header 4</h4>
<h5 id=\"header-5\">Header 5</h5>
<h6 id=\"header-6\">Header 6</h6>
<p>####### Header 7</p>
<h3 id=\"header\"><em>Header</em></h3>
"
), ("
#Header 1
##Header 2
###

####Header 4
#####Header 5
######Header 6
#######Header 7", "
<p>#Header 1 ##Header 2</p><h3></h3><p>####Header 4 #####Header 5 ######Header 6 #######Header 7</p>"), ("
br  
br\\
no br
no br 
br  
p

end
", "
<p>br<br/>br<br/>no br no br br<br/>p</p><p>end</p>
"), ("
# Valid header
Valid header
# Valid header

# Valid header

Valid header

# Valid header

# [[Red]] Red [[/Red]]

# **Bold** and ~~Del~~..!!

# `Code header`

", "
<h1 id=\"valid-header\">Valid header</h1>
<p>Valid header</p>
<h1 id=\"valid-header\">Valid header</h1>

<h1 id=\"valid-header\">Valid header</h1>

<p>Valid header</p>

<h1 id=\"valid-header\">Valid header</h1>

<h1 id=\"red-red-red\"><span class=\"color-red\"> Red </span></h1>
<h1 id=\"bold-and-del\"><strong>Bold</strong> and <del>Del</del>..!!</h1>
<h1 id=\"code-header\"><code class=\"short\">Code header</code></h1>
"), ("
# Header 1

[header1](#Header-1)
", "
<h1 id=\"header-1\">Header 1</h1>
<p><a href=\"#header-1\">header1</a></p>
"), ("
# TOC test

[[toc]]

## Header 1-1

## Header 1-2

##### Header 1-2-1

## Header 1-3

## *Italic* and [[red]] red [[/red]]

## ] <- valid link?

## Footnotes [^A]

[^A]: This is a footnote.

## Recursive [[toc]]

Another [[toc]]
", "

<h1 id=\"toc-test\">TOC test</h1>

<p>
<ol type=\"1\">
    <li><a href=\"#toc-test\">TOC test</a>
        <ol type=\"1\">
            <li><a href=\"#header-1-1\">Header 1-1</a></li>
            <li><a href=\"#header-1-2\">Header 1-2</a>
                <ol type=\"1\">
                    <li><a href=\"#header-1-2-1\">Header 1-2-1</a></li>
                </ol>
            </li>
            <li><a href=\"#header-1-3\">Header 1-3</a></li>
            <li><a href=\"#italic-and-red-red-red\"><em>Italic</em> and &#91;&#91;red&#93;&#93; red &#91;&#91;/red&#93;&#93;</a></li>
            <li><a href=\"#-lt--valid-link\">&#93; &lt;- valid link?</a></li>
            <li><a href=\"#footnotes-a\">Footnotes &#91;^A&#93;</a></li>
            <li><a href=\"#recursive-toc\">Recursive &#91;&#91;toc&#93;&#93;</a></li>
        </ol>
    </li>
</ol>
</p>

<h2 id=\"header-1-1\">Header 1-1</h2>
<h2 id=\"header-1-2\">Header 1-2</h2>
<h5 id=\"header-1-2-1\">Header 1-2-1</h5>
<h2 id=\"header-1-3\">Header 1-3</h2>
<h2 id=\"italic-and-red-red-red\"><em>Italic</em> and <span class=\"color-red\"> red </span></h2>
<h2 id=\"-lt--valid-link\">] &lt;- valid link?</h2>
<h2 id=\"footnotes-a\">Footnotes <sup id=\"footnote-ref-0\"><a href=\"#footnote-cite-0\">[0]</a></sup></h2>
<h2 id=\"recursive-toc\">Recursive
    <ol type=\"1\">
        <li><a href=\"#toc-test\">TOC test</a>
            <ol type=\"1\">
                <li><a href=\"#header-1-1\">Header 1-1</a></li>
                <li><a href=\"#header-1-2\">Header 1-2</a>
                    <ol type=\"1\">
                        <li><a href=\"#header-1-2-1\">Header 1-2-1</a></li>
                    </ol>
                </li>
                <li><a href=\"#header-1-3\">Header 1-3</a></li>
                <li><a href=\"#italic-and-red-red-red\"><em>Italic</em> and &#91;&#91;red&#93;&#93; red &#91;&#91;/red&#93;&#93;</a></li>
                <li><a href=\"#-lt--valid-link\">&#93; &lt;- valid link?</a></li>
                <li><a href=\"#footnotes-a\">Footnotes &#91;^A&#93;</a></li>
                <li><a href=\"#recursive-toc\">Recursive &#91;&#91;toc&#93;&#93;</a></li>
            </ol>
        </li>
    </ol>
</h2>

<p>
Another <ol type=\"1\">
    <li><a href=\"#toc-test\">TOC test</a>
        <ol type=\"1\">
            <li><a href=\"#header-1-1\">Header 1-1</a></li>
            <li><a href=\"#header-1-2\">Header 1-2</a>
                <ol type=\"1\">
                    <li><a href=\"#header-1-2-1\">Header 1-2-1</a></li>
                </ol>
            </li>
            <li><a href=\"#header-1-3\">Header 1-3</a></li>
            <li><a href=\"#italic-and-red-red-red\"><em>Italic</em> and &#91;&#91;red&#93;&#93; red &#91;&#91;/red&#93;&#93;</a></li>
            <li><a href=\"#-lt--valid-link\">&#93; &lt;- valid link?</a></li>
            <li><a href=\"#footnotes-a\">Footnotes &#91;^A&#93;</a></li>
            <li><a href=\"#recursive-toc\">Recursive &#91;&#91;toc&#93;&#93;</a></li>
        </ol>
    </li>
</ol>
</p>

<hr class=\"footnote-hr\"/>

<div class=\"mdxt-footnote-cites\">
<p><a id=\"footnote-cite-0\"></a>0. <a href=\"#footnote-ref-0\"> [0]</a> This is a footnote.<br/></p>
</div>
")
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn header_test() {
    for (md, html) in header_samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v16(&rendered)) != remove_whitespaces(&into_v16(html)) {
            panic!("{} \n\n {}", md, rendered);
        }

    }

}