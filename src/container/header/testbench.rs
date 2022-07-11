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

# [[red]] Macros not rendered [[/red]]

# **Bold** and ~~Del~~..!!

# `Code header`

", "
<h1 id=\"valid-header\">Valid header</h1>
<p>Valid header</p>
<h1 id=\"valid-header\">Valid header</h1>

<h1 id=\"valid-header\">Valid header</h1>

<p>Valid header</p>

<h1 id=\"valid-header\">Valid header</h1>

<h1 id=\"red-macros-not-rendered-red\">[[red]] Macros not rendered [[/red]]</h1>
<h1 id=\"bold-and-del\"><strong>Bold</strong> and <del>Del</del>..!!</h1>
<h1 id=\"code-header\"><code class=\"short\">Code header</code></h1>
"), ("
# Header 1

[header1](#Header-1)
", "
<h1 id=\"header-1\">Header 1</h1>
<p><a href=\"#header-1\">header1</a></p>
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