use crate::utils::{into_v32, remove_whitespaces};
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
<h1 id=\"code-header\"><code class=\"inline-code-span\">Code header</code></h1>
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

##### Header 1-2-1-1-1

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
    <div class=\"toc\">
        <ul class=\"no-bullet-list\">
            <li><a href=\"#toc-test\">1.</a> TOC test
                <ul class=\"no-bullet-list\">
                    <li><a href=\"#header-1-1\">1.1.</a> Header 1-1</li>
                    <li><a href=\"#header-1-2\">1.2.</a> Header 1-2
                        <ul class=\"no-bullet-list\">
                            <li><a href=\"#header-1-2-1-1-1\">1.2.1.1.1.</a> Header 1-2-1-1-1</li>
                        </ul>
                    </li>
                    <li><a href=\"#header-1-3\">1.3.</a> Header 1-3</li>
                    <li><a href=\"#italic-and-red-red-red\">1.4.</a> <em>Italic</em> and <span class=\"color-red\"> red </span></li>
                    <li><a href=\"#---valid-link\">1.5.</a> ] &lt;- valid link?</li>
                    <li><a href=\"#footnotes-a\">1.6.</a> Footnotes <span class=\"footnote-ref\" id=\"footnote-ref-1\"><a href=\"#footnote-cite-0\">[1]</a></span></li>
                    <li><a href=\"#recursive-toc\">1.7.</a> Recursive &#91;[toc]]</li>
                </ul>
            </li>
        </ul>
    </div>
</p>

<h2 id=\"header-1-1\">Header 1-1</h2>

<h2 id=\"header-1-2\">Header 1-2</h2>

<h5 id=\"header-1-2-1-1-1\">Header 1-2-1-1-1</h5>

<h2 id=\"header-1-3\">Header 1-3</h2>

<h2 id=\"italic-and-red-red-red\"><em>Italic</em> and <span class=\"color-red\"> red </span></h2>

<h2 id=\"---valid-link\">] &lt;- valid link?</h2>

<h2 id=\"footnotes-a\">Footnotes <span class=\"footnote-ref\" id=\"footnote-ref-0\"><a href=\"#footnote-cite-0\">[0]</a></span></h2>

<h2 id=\"recursive-toc\">Recursive
    <div class=\"toc\">
        <ul class=\"no-bullet-list\">
            <li><a href=\"#toc-test\">1.</a> TOC test
                <ul class=\"no-bullet-list\">
                    <li><a href=\"#header-1-1\">1.1.</a> Header 1-1</li>
                    <li><a href=\"#header-1-2\">1.2.</a> Header 1-2
                        <ul class=\"no-bullet-list\">
                            <li><a href=\"#header-1-2-1-1-1\">1.2.1.1.1.</a> Header 1-2-1-1-1</li>
                        </ul>
                    </li>
                    <li><a href=\"#header-1-3\">1.3.</a> Header 1-3</li>
                    <li><a href=\"#italic-and-red-red-red\">1.4.</a> <em>Italic</em> and <span class=\"color-red\"> red </span></li>
                    <li><a href=\"#---valid-link\">1.5.</a> ] &lt;- valid link?</li>
                    <li><a href=\"#footnotes-a\">1.6.</a> Footnotes <span class=\"footnote-ref\" id=\"footnote-ref-1\"><a href=\"#footnote-cite-0\">[1]</a></span></li>
                    <li><a href=\"#recursive-toc\">1.7.</a> Recursive &#91;[toc]]</li>
                </ul>
            </li>
        </ul>
    </div>
</h2>

<p>Another
    <div class=\"toc\">
        <ul class=\"no-bullet-list\">
            <li><a href=\"#toc-test\">1.</a> TOC test
                <ul class=\"no-bullet-list\">
                    <li><a href=\"#header-1-1\">1.1.</a> Header 1-1</li>
                    <li><a href=\"#header-1-2\">1.2.</a> Header 1-2
                        <ul class=\"no-bullet-list\">
                            <li><a href=\"#header-1-2-1-1-1\">1.2.1.1.1.</a> Header 1-2-1-1-1</li>
                        </ul>
                    </li>
                    <li><a href=\"#header-1-3\">1.3.</a> Header 1-3</li>
                    <li><a href=\"#italic-and-red-red-red\">1.4.</a> <em>Italic</em> and <span class=\"color-red\"> red </span></li>
                    <li><a href=\"#---valid-link\">1.5.</a> ] &lt;- valid link?</li>
                    <li><a href=\"#footnotes-a\">1.6.</a> Footnotes <span class=\"footnote-ref\" id=\"footnote-ref-1\"><a href=\"#footnote-cite-0\">[1]</a></span></li>
                    <li><a href=\"#recursive-toc\">1.7.</a> Recursive &#91;[toc]]</li>
                </ul>
            </li>
        </ul>
    </div>
</p>

<hr class=\"footnote-hr\"/>

<div class=\"mdxt-footnote-cites\">
    <p>
        <div class=\"footnote-cite\">
            <a id=\"footnote-cite-0\"></a><a href=\"#footnote-ref-0\"> [0]</a> <a href=\"#footnote-ref-1\"> [1]</a> This is a footnote.
        </div>
    </p>
</div>
"), (
"
# Header 1

# Header 2

## Header 2.1

## Header 2.2

## Header 2.3

### Header 2.3.1

## Header 2.4

# Header 3

[[toc]]
", "
<h1 id=\"header-1\">Header 1</h1>

<h1 id=\"header-2\">Header 2</h1>

<h2 id=\"header-21\">Header 2.1</h2>

<h2 id=\"header-22\">Header 2.2</h2>

<h2 id=\"header-23\">Header 2.3</h2>

<h3 id=\"header-231\">Header 2.3.1</h3>

<h2 id=\"header-24\">Header 2.4</h2>

<h1 id=\"header-3\">Header 3</h1>

<p>
    <div class=\"toc\">
        <ul class=\"no-bullet-list\">
            <li><a href=\"#header-1\">1.</a> Header 1</li>
            <li><a href=\"#header-2\">2.</a> Header 2
                <ul class=\"no-bullet-list\">
                    <li><a href=\"#header-21\">2.1.</a> Header 2.1</li>
                    <li><a href=\"#header-22\">2.2.</a> Header 2.2</li>
                    <li><a href=\"#header-23\">2.3.</a> Header 2.3
                        <ul class=\"no-bullet-list\">
                            <li><a href=\"#header-231\">2.3.1.</a> Header 2.3.1</li>
                        </ul>
                    </li>
                    <li><a href=\"#header-24\">2.4.</a> Header 2.4</li>
                </ul>
            </li>
            <li><a href=\"#header-3\">3.</a> Header 3</li>
        </ul>
    </div>
</p>
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

        if remove_whitespaces(&into_v32(&rendered)) != remove_whitespaces(&into_v32(html)) {
            panic!("{md} \n\n {rendered}");
        }

    }

}