use crate::utils::{into_v16, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn ast_samples() -> Vec<(String, String)> {
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
<h1>Header 1</h1>
<h2>Header 2</h2>
<h3></h3>

<h4>Header 4</h4>
<h5>Header 5</h5>
<h6>Header 6</h6>
<p>####### Header 7</p>
<h3><em>Header</em></h3>
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
")
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn ast_test() {
    for (md, html) in ast_samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v16(&rendered)) != remove_whitespaces(&into_v16(html)) {
            panic!("{} \n\n {}", md, rendered);
        }

    }

}