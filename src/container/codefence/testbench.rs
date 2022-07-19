use super::read_code_fence_info;
use crate::ast::line::Line;
use crate::ast::parse::ParseState;
use crate::utils::{into_v16, from_v16, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn fence_samples() -> Vec<(
    String,         // case
    bool,           // is_valid
    bool,           // is_code_fence_begin
    bool,           // is_code_fence_end
    String,         // language
    Option<usize>,  // line_num
    Vec<usize>,     // highlights
    usize,          // fence_size
    bool,           // copy_button
    bool            // is_tilde_fence
)> {
    let samples = vec![
        ("```", true, true, true, "", None, vec![], 3, false, false),
        ("````", true, true, true, "", None, vec![], 4, false, false),
        ("~~~", true, true, true, "", None, vec![], 3, false, true),
        ("~~~~", true, true, true, "", None, vec![], 4, false, true),
        ("``", false, false, false, "", None, vec![], 0, false, false),
        ("``` `", false, false, false, "", None, vec![], 0, false, false),
        ("```rust `", false, false, false, "", None, vec![], 0, false, false),
        ("`~~", false, false, false, "", None, vec![], 0, false, false),
        ("~~`", false, false, false, "", None, vec![], 0, false, false),
        ("```Rust", true, true, false, "rust", None, vec![], 3, false, false),
        ("``` rust", true, true, false, "rust", None, vec![], 3, false, false),
        ("``` line_num", true, true, false, "", Some(1), vec![], 3, true, false),
        ("```line_num(5)", true, true, false, "", Some(5), vec![], 3, true, false),
        ("```rust, line_num(5)", true, true, false, "rust", Some(5), vec![], 3, true, false),
        ("```line_num(5), rust", true, true, false, "rust", Some(5), vec![], 3, true, false),
        ("~~~Rust", true, true, false, "rust", None, vec![], 3, false, true),
        ("~~~ rust", true, true, false, "rust", None, vec![], 3, false, true),
        ("~~~ line_num", true, true, false, "", Some(1), vec![], 3, true, true),
        ("~~~line_num(5)", true, true, false, "", Some(5), vec![], 3, true, true),
        ("~~~rust, line_num(5)", true, true, false, "rust", Some(5), vec![], 3, true, true),
        ("~~~line_num(5), rust", true, true, false, "rust", Some(5), vec![], 3, true, true),
        ("```highlight(4), line_num", true, true, false, "", Some(1), vec![4], 3, true, false),
        ("```highlight(4, 5), line_num", true, true, false, "", Some(1), vec![4, 5], 3, true, false),
        ("```line_num, highlight(4)", true, true, false, "", Some(1), vec![4], 3, true, false),
        ("```line_num, highlight(4, 5)", true, true, false, "", Some(1), vec![4, 5], 3, true, false),
        ("```highlight(4", true, true, false, "highlight(4", None, vec![], 3, false, false),
        ("```!!", false, false, false, "", None, vec![], 0, false, false),
        ("```sublime-syntax", true, true, false, "sublime-syntax", None, vec![], 3, false, false),
        ("```.bash_login", true, true, false, ".bash_login", None, vec![], 3, false, false),
        ("```copy_button", true, true, false, "", None, vec![], 3, true, false),
        ("```copy_button(true)", true, true, false, "", None, vec![], 3, true, false),
        ("```copy_button(false)", true, true, false, "", None, vec![], 3, false, false),
        ("```copy_button(okay)", true, true, false, "copy_button(okay)", None, vec![], 3, false, false),
    ];

    samples.into_iter().map(
        |(
            case,
            is_valid,
            is_code_fence_begin,
            is_code_fence_end,
            language,
            line_num,
            highlights,
            fence_size,
            copy_button,
            is_tilde_fence
        )| (
            case.to_string(),
            is_valid,
            is_code_fence_begin,
            is_code_fence_end,
            language.to_string(),
            line_num,
            highlights,
            fence_size,
            copy_button,
            is_tilde_fence
        )
    ).collect()
}

#[test]
fn fence_test() {
    let mut failures = vec![];
    let test_cases = fence_samples();

    for (
        case,
        is_valid,
        is_code_fence_begin,
        is_code_fence_end,
        language_answer,
        line_num_answer,
        highlights_answer,
        fence_size_answer,
        copy_button_answer,
        is_tilde_fence_answer
    ) in test_cases.iter() {
        let v16 = into_v16(&case);
        let line = Line::from_raw(&v16);

        if *is_valid != (line.is_code_fence_begin() || line.is_code_fence_end()) {
            failures.push(format!(
                "case: {}\nis_valid: answer: {}, actual: {}",
                case, is_valid, line.is_code_fence_begin() || line.is_code_fence_end()
            ));
            continue;
        }

        if !is_valid {
            continue;
        }

        if *is_code_fence_begin != line.is_code_fence_begin() || *is_code_fence_end != line.is_code_fence_end() {
            failures.push(format!(
                "case: {}\nis_code_fence_begin: answer: {}, actual: {}\nis_code_fence_end: answer: {}, actual: {}",
                case,
                is_code_fence_begin, line.is_code_fence_begin(),
                is_code_fence_end, line.is_code_fence_end()
            ));
            continue;
        }

        let (
            language_actual,
            line_num_actual,
            highlights_actual,
            fence_size_actual,
            copy_button_actual,
            is_tilde_fence_actual
        ) = match read_code_fence_info(&line, 0) {
            ParseState::CodeFence { language, line_num, highlights, code_fence_size, copy_button, is_tilde_fence, index: _index } => (
                from_v16(&language), line_num, highlights, code_fence_size, copy_button, is_tilde_fence
            ),
            _ => panic!("This doesn't make sense at all."),
        };

        if language_answer != &language_actual {
            failures.push(format!(
                "case: {}\nlanguage: answer: {:?}, actual: {:?}",
                case, language_answer, language_actual
            ));
            continue;
        }

        if line_num_answer != &line_num_actual {
            failures.push(format!(
                "case: {}\nline_num: answer: {:?}, actual: {:?}",
                case, line_num_answer, line_num_actual
            ));
            continue;
        }

        if highlights_answer != &highlights_actual {
            failures.push(format!(
                "case: {}\nhighlights: answer: {:?}, actual: {:?}",
                case, highlights_answer, highlights_actual
            ));
            continue;
        }

        if fence_size_answer != &fence_size_actual {
            failures.push(format!(
                "case: {}\nfence_size: answer: {:?}, actual: {:?}",
                case, fence_size_answer, fence_size_actual
            ));
            continue;
        }

        if copy_button_answer != &copy_button_actual {
            failures.push(format!(
                "case: {}\ncopy_button: answer: {:?}, actual: {:?}",
                case, copy_button_answer, copy_button_actual
            ));
            continue;
        }

        if is_tilde_fence_answer != &is_tilde_fence_actual {
            failures.push(format!(
                "case: {}\nis_tilde_fence: answer: {:?}, actual: {:?}",
                case, is_tilde_fence_answer, is_tilde_fence_actual
            ));
            continue;
        }

    }

    if failures.len() > 0 {
        panic!(
            "Codefence fence test: {} case(s) out of {} cases have failed!\n\n{}",
            failures.len(),
            test_cases.len(),
            failures.join("\n\n")
        );
    }

}

fn code_fence_samples() -> Vec<(String, String)> {
    let result = vec![
        ("
```rust
/*
    multiline
    comment
*/
// single line comment
fn main() {
    let mut x = 3;
    let mut y = if x == 3 {
        4
    } else {
        5
    };
    println!(\"Hello World!\\n\");
    println!(\"{:?}\", 3 + 4);
}

pub struct Point {
    x: f32,
    y: f32
}

pub const CONST: u32 = 1;
```

````
```rust
fn main() {
    println!(\"Hello World!\");
}
```
````
", "
<pre><code><table><tbody>
    <tr>
        <td><span class=\"color_gray\">/*</span></td>
    </tr>
    <tr>
        <td><span class=\"color_gray\">    multiline</span></td>
    </tr>
    <tr>
        <td><span class=\"color_gray\">    comment</span></td>
    </tr>
    <tr>
        <td><span class=\"color_gray\">*/</span></td>
    </tr>
    <tr>
        <td><span class=\"color_gray\">//</span><span class=\"color_gray\"> single line comment</span></td>
    </tr>
    <tr>
        <td><span class=\"color_violet\">fn</span><span class=\"color_white\"> </span><span class=\"color_aqua\">main</span><span class=\"color_white\">(</span><span class=\"color_white\">)</span><span class=\"color_white\"> </span><span class=\"color_white\">{</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">    </span><span class=\"color_violet\">let</span><span class=\"color_white\"> </span><span class=\"color_violet\">mut</span><span class=\"color_white\"> x </span><span class=\"color_white\">=</span><span class=\"color_white\"> </span><span class=\"color_gold\">3</span><span class=\"color_white\">;</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">    </span><span class=\"color_violet\">let</span><span class=\"color_white\"> </span><span class=\"color_violet\">mut</span><span class=\"color_white\"> y </span><span class=\"color_white\">=</span><span class=\"color_white\"> </span><span class=\"color_violet\">if</span><span class=\"color_white\"> x </span><span class=\"color_white\">=</span><span class=\"color_white\">=</span><span class=\"color_white\"> </span><span class=\"color_gold\">3</span><span class=\"color_white\"> </span><span class=\"color_white\">{</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">        </span><span class=\"color_gold\">4</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">    </span><span class=\"color_white\">}</span><span class=\"color_white\"> </span><span class=\"color_violet\">else</span><span class=\"color_white\"> </span><span class=\"color_white\">{</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">        </span><span class=\"color_gold\">5</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">    </span><span class=\"color_white\">}</span><span class=\"color_white\">;</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">    </span><span class=\"color_white\">println!</span><span class=\"color_white\">(</span><span class=\"color_white\">&quot;</span><span class=\"color_green\">Hello World!</span><span class=\"color_emerald\">\\n</span><span class=\"color_white\">&quot;</span><span class=\"color_white\">)</span><span class=\"color_white\">;</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">    </span><span class=\"color_white\">println!</span><span class=\"color_white\">(</span><span class=\"color_white\">&quot;</span><span class=\"color_gold\">{:?}</span><span class=\"color_white\">&quot;</span><span class=\"color_white\">,</span><span class=\"color_white\"> </span><span class=\"color_gold\">3</span><span class=\"color_white\"> </span><span class=\"color_white\">+</span><span class=\"color_white\"> </span><span class=\"color_gold\">4</span><span class=\"color_white\">)</span><span class=\"color_white\">;</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">}</span></td>
    </tr>
    <tr>
        <td></td>
    </tr>
    <tr>
        <td><span class=\"color_violet\">pub</span><span class=\"color_white\"> </span><span class=\"color_violet\">struct</span><span class=\"color_white\"> </span><span class=\"color_white\">Point</span><span class=\"color_white\"> </span><span class=\"color_white\">{</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">    </span><span class=\"color_red\">x</span><span class=\"color_white\">:</span><span class=\"color_white\"> </span><span class=\"color_violet\">f32</span><span class=\"color_white\">,</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">    </span><span class=\"color_red\">y</span><span class=\"color_white\">:</span><span class=\"color_white\"> </span><span class=\"color_violet\">f32</span></td>
    </tr>
    <tr>
        <td><span class=\"color_white\">}</span></td>
    </tr>
    <tr>
        <td></td>
    </tr>
    <tr>
        <td><span class=\"color_violet\">pub</span><span class=\"color_white\"> </span><span class=\"color_violet\">const</span><span class=\"color_white\"> </span><span class=\"color_gold\">CONST</span><span class=\"color_white\">:</span><span class=\"color_white\"> </span><span class=\"color_violet\">u32</span><span class=\"color_white\"> </span><span class=\"color_white\">=</span><span class=\"color_white\"> </span><span class=\"color_gold\">1</span><span class=\"color_white\">;</span></td>
    </tr>
</tbody></table></code></pre>

<pre><code><table><tbody>
    <tr>
        <td>```rust</td>
    </tr>
    <tr>
        <td>fn main() {</td>
    </tr>
    <tr>
        <td>    println!(&quot;Hello World!&quot;);</td>
    </tr>
    <tr>
        <td>}</td>
    </tr>
    <tr>
        <td>```</td>
    </tr>
</tbody></table></code></pre>
"), ("
```rust, line_num, highlight(2, 3)
fn main() {
    println!(\"Hello World!\\n\");
}
```
", "
<pre><code><table><tbody>
    <tr>
        <td class=\"index\">1</td>
        <td><span class=\"color_violet\">fn</span><span class=\"color_white\"> </span><span class=\"color_aqua\">main</span><span class=\"color_white\">(</span><span class=\"color_white\">)</span><span class=\"color_white\"> </span><span class=\"color_white\">{</span></td>
    </tr>
    <tr class=\"highlight\">
        <td class=\"index\">2</td>
        <td><span class=\"color_white\">    </span><span class=\"color_white\">println!</span><span class=\"color_white\">(</span><span class=\"color_white\">&quot;</span><span class=\"color_green\">Hello World!</span><span class=\"color_emerald\">\\n</span><span class=\"color_white\">&quot;</span><span class=\"color_white\">)</span><span class=\"color_white\">;</span></td>
    </tr>
    <tr class=\"highlight\">
        <td class=\"index\">3</td>
        <td><span class=\"color_white\">}</span></td>
    </tr>
    <tr class=\"copy-button-row\"><td><button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(0)\">Copy</button></td></tr>
</tbody></table></code></pre>
<script>
const fenced_code_block_contents = [\"fn main() {\\n    println!(\\\"Hello World!\\\\n\\\");\\n}\"];

function copy_code_to_clipboard(index) {
    navigator.clipboard.writeText(fenced_code_block_contents[index]);
}
</script>
"), ("
```rust, line_num(5)
fn main() {
    println!(\"Hello World!\\n\");
}
```
", "
<pre><code><table><tbody>
    <tr>
        <td class=\"index\">5</td>
        <td><span class=\"color_violet\">fn</span><span class=\"color_white\"> </span><span class=\"color_aqua\">main</span><span class=\"color_white\">(</span><span class=\"color_white\">)</span><span class=\"color_white\"> </span><span class=\"color_white\">{</span></td>
    </tr>
    <tr>
        <td class=\"index\">6</td>
        <td><span class=\"color_white\">    </span><span class=\"color_white\">println!</span><span class=\"color_white\">(</span><span class=\"color_white\">&quot;</span><span class=\"color_green\">Hello World!</span><span class=\"color_emerald\">\\n</span><span class=\"color_white\">&quot;</span><span class=\"color_white\">)</span><span class=\"color_white\">;</span></td>
    </tr>
    <tr>
        <td class=\"index\">7</td>
        <td><span class=\"color_white\">}</span></td>
    </tr>
    <tr class=\"copy-button-row\"><td><button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(0)\">Copy</button></td></tr>
</tbody></table></code></pre>
<script>
const fenced_code_block_contents = [\"fn main() {\\n    println!(\\\"Hello World!\\\\n\\\");\\n}\"];

function copy_code_to_clipboard(index) {
    navigator.clipboard.writeText(fenced_code_block_contents[index]);
}
</script>
"), ("
``` html
<p> <div class=\"box\"> box </div> </p>
```
", "
<pre><code><table><tbody><tr><td><span class=\"color_white\">&lt;</span><span class=\"color_red\">p</span><span class=\"color_white\">></span><span class=\"color_white\"> </span><span class=\"color_white\">&lt;</span><span class=\"color_red\">div</span><span class=\"color_white\"> </span><span class=\"color_gold\">class</span><span class=\"color_white\">=</span><span class=\"color_white\">&quot;</span><span class=\"color_green\">box</span><span class=\"color_white\">&quot;</span><span class=\"color_white\">></span><span class=\"color_white\"> box </span><span class=\"color_white\">&lt;/</span><span class=\"color_red\">div</span><span class=\"color_white\">></span><span class=\"color_white\"> </span><span class=\"color_white\">&lt;/</span><span class=\"color_red\">p</span><span class=\"color_white\">></span></td></tr></tbody></table></code></pre>
"), ("
```line_num
<p> <div class=\"box\"> box </div> </p>
```
", "
<pre><code><table><tbody>
    <tr><td class=\"index\">1</td><td>&lt;p> &lt;div class=&quot;box&quot;> box &lt;/div> &lt;/p></td></tr>
    <tr class=\"copy-button-row\"><td><button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(0)\">Copy</button></td></tr>
</tbody></table></code></pre>

<script>
const fenced_code_block_contents = [\"<p> <div class=\\\"box\\\"> box </div> </p>\"];

function copy_code_to_clipboard(index) {
    navigator.clipboard.writeText(fenced_code_block_contents[index]);
}
</script>
"), ("
```invalid_language_name
<p> <div class=\"box\"> box </div> </p>
```
", "
<pre><code><table><tbody>
    <tr><td>&lt;p> &lt;div class=&quot;box&quot;> box &lt;/div> &lt;/p></td></tr>
</tbody></table></code></pre>
"), ("
```
```
", "
<pre><code><table><tbody><tr><td></td></tr></tbody></table></code></pre>
"), ("
# Backslash test

`\\\\`

```
\\\\
```
", "
<h1 id=\"backslash-test\">Backslash test</h1>

<p><code class=\"short\">\\\\</code></p>

<pre><code><table><tbody>
    <tr>
        <td>\\\\</td>
    </tr>
</tbody></table></code></pre>
"), ("
`````
````
``````
````
``````
", "
<pre><code><table><tbody>
    <tr><td>````</td></tr>
</tbody></table></code></pre>

<pre><code><table><tbody>
    <tr><td></td></tr>
</tbody></table></code></pre>
"), (
"
# Bracket tests

```
<>
```

```html
<div> a </div>
<
```

`<>`
", "
<h1 id=\"bracket-tests\">Bracket tests</h1>

<pre><code><table><tbody>
    <tr><td>&lt;></td></tr>
</tbody></table></code></pre>

<pre><code><table><tbody>
    <tr><td><span class=\"color_white\">&lt;</span><span class=\"color_red\">div</span><span class=\"color_white\">></span><span class=\"color_white\"> a </span><span class=\"color_white\">&lt;/</span><span class=\"color_red\">div</span><span class=\"color_white\">></span></td></tr>
    <tr><td><span class=\"color_white\">&lt;</span></td></tr>
</tbody></table></code></pre>

<p><code class=\"short\">&lt;></code></p>
"
), ("
# Copy Button test

```copy_button
Nothing1
```

```copy_button
Nothing2
```
", "
<h1 id=\"copy-button-test\">Copy Button test</h1>

<pre><code><table><tbody>
    <tr><td>Nothing1</td></tr>
    <tr class=\"copy-button-row\"><td><button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(0)\">Copy</button></td></tr>
</tbody></table></code></pre>

<pre><code><table><tbody>
    <tr><td>Nothing2</td></tr>
    <tr class=\"copy-button-row\"><td><button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(1)\">Copy</button></td></tr>
</tbody></table></code></pre>

<script>
const fenced_code_block_contents = [\"Nothing1\", \"Nothing2\"];

function copy_code_to_clipboard(index) {
    navigator.clipboard.writeText(fenced_code_block_contents[index]);
}
</script>
")
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn code_fence_test() {
    for (md, html) in code_fence_samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v16(&rendered)) != remove_whitespaces(&into_v16(html)) {
            panic!("{} \n\n {} \n\n {:?}", md, rendered, rendered);
        }

    }

}