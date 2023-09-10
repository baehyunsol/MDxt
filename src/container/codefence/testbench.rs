use super::read_code_fence_info;
use crate::ast::line::Line;
use crate::ast::parse::ParseState;
use crate::utils::{into_v32, from_v32, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn fence_samples() -> Vec<(
    String,            // case
    bool,              // is_valid
    bool,              // is_code_fence_begin
    bool,              // is_code_fence_end
    String,            // language
    Option<usize>,     // line_num
    Vec<usize>,        // highlights
    usize,             // fence_size
    bool,              // copy_button
    Option<String>,    // id
    Vec<String>,       // classes
    bool               // is_tilde_fence
)> {
    let samples = vec![
        ("```", true, true, true, "", None, vec![], 3, false, None, vec![], false),
        ("````", true, true, true, "", None, vec![], 4, false, None, vec![], false),
        ("~~~", true, true, true, "", None, vec![], 3, false, None, vec![], true),
        ("~~~~", true, true, true, "", None, vec![], 4, false, None, vec![], true),
        ("``", false, false, false, "", None, vec![], 0, false, None, vec![], false),
        ("``` `", false, false, false, "", None, vec![], 0, false, None, vec![], false),
        ("```rust `", false, false, false, "", None, vec![], 0, false, None, vec![], false),
        ("`~~", false, false, false, "", None, vec![], 0, false, None, vec![], false),
        ("~~`", false, false, false, "", None, vec![], 0, false, None, vec![], false),
        ("```Rust", true, true, false, "rust", None, vec![], 3, false, None, vec![], false),
        ("``` rust", true, true, false, "rust", None, vec![], 3, false, None, vec![], false),
        ("``` line_num", true, true, false, "", Some(1), vec![], 3, true, None, vec![], false),
        ("```line_num(5)", true, true, false, "", Some(5), vec![], 3, true, None, vec![], false),
        ("```rust, line_num(5)", true, true, false, "rust", Some(5), vec![], 3, true, None, vec![], false),
        ("```line_num(5), rust", true, true, false, "rust", Some(5), vec![], 3, true, None, vec![], false),
        ("~~~Rust", true, true, false, "rust", None, vec![], 3, false, None, vec![], true),
        ("~~~ rust", true, true, false, "rust", None, vec![], 3, false, None, vec![], true),
        ("~~~ line_num", true, true, false, "", Some(1), vec![], 3, true, None, vec![], true),
        ("~~~line_num(5)", true, true, false, "", Some(5), vec![], 3, true, None, vec![], true),
        ("~~~rust, line_num(5)", true, true, false, "rust", Some(5), vec![], 3, true, None, vec![], true),
        ("~~~line_num(5), rust", true, true, false, "rust", Some(5), vec![], 3, true, None, vec![], true),
        ("```highlight(4), line_num", true, true, false, "", Some(1), vec![4], 3, true, None, vec![], false),
        ("```highlight(4, 5), line_num", true, true, false, "", Some(1), vec![4, 5], 3, true, None, vec![], false),
        ("```line_num, highlight(4)", true, true, false, "", Some(1), vec![4], 3, true, None, vec![], false),
        ("```line_num, highlight(4, 5)", true, true, false, "", Some(1), vec![4, 5], 3, true, None, vec![], false),
        ("```highlight(4", true, true, false, "highlight(4", None, vec![], 3, false, None, vec![], false),
        ("```!!", false, false, false, "", None, vec![], 0, false, None, vec![], false),
        ("```sublime-syntax", true, true, false, "sublime-syntax", None, vec![], 3, false, None, vec![], false),
        ("```.bash_login", true, true, false, ".bash_login", None, vec![], 3, false, None, vec![], false),
        ("```copy_button", true, true, false, "", None, vec![], 3, true, None, vec![], false),
        ("```copy_button(true)", true, true, false, "", None, vec![], 3, true, None, vec![], false),
        ("```copy_button(false)", true, true, false, "", None, vec![], 3, false, None, vec![], false),
        ("```copy_button(okay)", true, true, false, "copy_button(okay)", None, vec![], 3, false, None, vec![], false),
        ("```id(abcd)", true, true, false, "", None, vec![], 3, false, Some("abcd"), vec![], false),
        ("```class(abc), class(def)", true, true, false, "", None, vec![], 3, false, None, vec!["abc", "def"], false),
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
            id,
            classes,
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
            id.map(|i| i.to_string()),
            classes.into_iter().map(|c| c.to_string()).collect(),
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
        id_answer,
        classes_answer,
        is_tilde_fence_answer
    ) in test_cases.iter() {
        let v32 = into_v32(&case);
        let line = Line::from_raw(&v32);

        if *is_valid != (line.is_code_fence_begin() || line.is_code_fence_end()) {
            failures.push(format!(
                "case: {case}\nis_valid: answer: {is_valid}, actual: {}",
                line.is_code_fence_begin() || line.is_code_fence_end()
            ));
            continue;
        }

        if !is_valid {
            continue;
        }

        if *is_code_fence_begin != line.is_code_fence_begin() || *is_code_fence_end != line.is_code_fence_end() {
            failures.push(format!(
                "case: {case}
is_code_fence_begin: answer: {is_code_fence_begin}, actual: {}
is_code_fence_end: answer: {is_code_fence_end}, actual: {}",
                line.is_code_fence_begin(),
                line.is_code_fence_end()
            ));
            continue;
        }

        let (
            language_actual,
            line_num_actual,
            highlights_actual,
            fence_size_actual,
            copy_button_actual,
            id_actual,
            classes_actual,
            is_tilde_fence_actual
        ) = match read_code_fence_info(&line, 0) {
            ParseState::CodeFence { language, line_num, highlights, code_fence_size, copy_button, is_tilde_fence, id, classes, index: _index } => (
                from_v32(&language),
                line_num,
                highlights,
                code_fence_size,
                copy_button,
                id.as_ref().map(|i| from_v32(i)),
                classes.iter().map(|c| from_v32(c)).collect::<Vec<String>>(),
                is_tilde_fence,
            ),
            _ => unreachable!(),
        };

        if language_answer != &language_actual {
            failures.push(format!(
                "case: {case}\nlanguage: answer: {language_answer:?}, actual: {language_actual:?}"
            ));
            continue;
        }

        if line_num_answer != &line_num_actual {
            failures.push(format!(
                "case: {case}\nline_num: answer: {line_num_answer:?}, actual: {line_num_actual:?}"
            ));
            continue;
        }

        if highlights_answer != &highlights_actual {
            failures.push(format!(
                "case: {case}\nhighlights: answer: {highlights_answer:?}, actual: {highlights_actual:?}",
            ));
            continue;
        }

        if fence_size_answer != &fence_size_actual {
            failures.push(format!(
                "case: {case}\nfence_size: answer: {fence_size_answer:?}, actual: {fence_size_actual:?}"
            ));
            continue;
        }

        if copy_button_answer != &copy_button_actual {
            failures.push(format!(
                "case: {case}\ncopy_button: answer: {copy_button_answer:?}, actual: {copy_button_actual:?}"
            ));
            continue;
        }

        if id_answer != &id_actual {
            failures.push(format!(
                "case: {case}\nid: answer: {id_answer:?}, actual: {id_actual:?}"
            ));
            continue;
        }

        if classes_answer != &classes_actual {
            failures.push(format!(
                "case: {case}\nclasses: answer: {classes_answer:?}, actual: {classes_actual:?}"
            ));
            continue;
        }

        if is_tilde_fence_answer != &is_tilde_fence_actual {
            failures.push(format!(
                "case: {case}\nis_tilde_fence: answer: {is_tilde_fence_answer:?}, actual: {is_tilde_fence_actual:?}",
            ));
            continue;
        }

    }

    if !failures.is_empty() {
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
가나다🍜👁🦈🥣🍚🗼🎂💍📷🍝🦑👍🎥👵😀🧒🏽🤷🏽👨🏿‍🎓🇰🇷🫵🏽🫵🏾🫵🏿❤️🧡💛💚💙💜🖤🤍🤎
```
", "
<pre class=\"fenced-code-block\">
    <code>
        <span class=\"code-fence-row\">
            <span class=\"code-fence-code\">
                <span class=\"color-white\">가나다🍜👁🦈🥣🍚🗼🎂💍📷🍝🦑👍🎥👵😀🧒🏽🤷🏽👨🏿‍🎓🇰🇷🫵🏽🫵🏾🫵🏿❤️🧡💛💚💙💜🖤🤍🤎</span>
            </span>
        </span>
    </code>
</pre>
"), ("
```rust, line_num, highlight(2, 3)
fn main() {
    println!(\"Hello World!\\n\");
}
```
", "
<pre class=\"fenced-code-block line-num-width-0\"><code>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-index\">1</span>
        <span class=\"code-fence-code\"><span class=\"color-violet\">fn</span><span class=\"color-aqua\">main</span><span class=\"color-white\">(){</span></span>
    </span>
    <span class=\"highlight code-fence-row\">
        <span class=\"code-fence-index\">2</span>
        <span class=\"code-fence-code\">    <span class=\"color-white\">println!(&quot;</span><span class=\"color-green\">Hello World!</span><span class=\"color-emerald\">\\n</span><span class=\"color-white\">&quot;);</span></span>
    </span>
    <span class=\"highlight code-fence-row\">
        <span class=\"code-fence-index\">3</span>
        <span class=\"code-fence-code\"><span class=\"color-white\">}</span></span>
    </span>
</code>
    <button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(0)\">Copy</button>
</pre>
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
<pre class=\"fenced-code-block line-num-width-0\"><code>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-index\">5</span>
        <span class=\"code-fence-code\"><span class=\"color-violet\">fn</span><span class=\"color-aqua\">main</span><span class=\"color-white\">(){</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-index\">6</span>
        <span class=\"code-fence-code\">    <span class=\"color-white\">println!(&quot;</span><span class=\"color-green\">Hello World!</span><span class=\"color-emerald\">\\n</span><span class=\"color-white\">&quot;);</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-index\">7</span>
        <span class=\"code-fence-code\"><span class=\"color-white\">}</span></span>
    </span>
</code>
    <button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(0)\">Copy</button>
</pre>
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
<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\"><span class=\"color-white\">&lt;</span><span class=\"color-red\">p</span><span class=\"color-white\">&gt;&lt;</span><span class=\"color-red\">div</span><span class=\"color-gold\">class</span><span class=\"color-white\">=&quot;</span><span class=\"color-green\">box</span><span class=\"color-white\">&quot;&gt; box &lt;/</span><span class=\"color-red\">div</span><span class=\"color-white\">&gt;&lt;/</span><span class=\"color-red\">p</span><span class=\"color-white\">&gt;</span></span></span>
</code></pre>
"), ("
```line_num
<p> <div class=\"box\"> box </div> </p>
```
", "
<pre class=\"fenced-code-block line-num-width-0\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-index\">1</span><span class=\"code-fence-code\">&lt;p&gt; &lt;div class=&quot;box&quot;&gt; box &lt;/div&gt; &lt;/p&gt;</span></span>
</code>
    <button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(0)\">Copy</button>
</pre>

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
<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\">&lt;p&gt; &lt;div class=&quot;box&quot;&gt; box &lt;/div&gt; &lt;/p&gt;</span></span>
</code></pre>
"), ("
```
```
", "
<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\"></span></span>
</code></pre>
"), ("
# Backslash test

`\\\\`

```
\\\\
```
", "
<h1 id=\"backslash-test\">Backslash test</h1>

<p><code class=\"inline-code-span\">\\\\</code></p>

<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">\\\\</span>
    </span>
</code></pre>
"), ("
`````
````
``````
````
``````
", "
<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\">````</span></span>
</code></pre>

<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\"></span></span>
</code></pre>
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

<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\">&lt;&gt;</span></span>
</code></pre>

<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\"><span class=\"color-white\">&lt;</span><span class=\"color-red\">div</span><span class=\"color-white\">&gt; a &lt;/</span><span class=\"color-red\">div</span><span class=\"color-white\">&gt;</span></span></span>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\"><span class=\"color-white\">&lt;</span></span></span>
</code></pre>

<p><code class=\"inline-code-span\">&lt;&gt;</code></p>
"
), ("
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
<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-gray\">/*</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-gray\">    multiline</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-gray\">    comment</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-gray\">*/</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-gray\">// single line comment</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-violet\">fn</span><span class=\"color-aqua\">main</span><span class=\"color-white\">(){</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">    <span class=\"color-violet\">let mut</span><span class=\"color-white\"> x =</span><span class=\"color-gold\">3</span><span class=\"color-white\">;</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">    <span class=\"color-violet\">let mut</span><span class=\"color-white\"> y =</span><span class=\"color-violet\">if</span><span class=\"color-white\"> x ==</span><span class=\"color-gold\">3</span><span class=\"color-white\">{</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">        <span class=\"color-gold\">4</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">    <span class=\"color-white\">}</span><span class=\"color-violet\">else</span><span class=\"color-white\">{</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">        <span class=\"color-gold\">5</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">    <span class=\"color-white\">};</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">    <span class=\"color-white\">println!(&quot;</span><span class=\"color-green\">Hello World!</span><span class=\"color-emerald\">\\n</span><span class=\"color-white\">&quot;);</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">    <span class=\"color-white\">println!(&quot;</span><span class=\"color-gold\">{:?}</span><span class=\"color-white\">&quot;,</span><span class=\"color-gold\">3</span><span class=\"color-white\">+</span><span class=\"color-gold\">4</span><span class=\"color-white\">);</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-white\">}</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-violet\">pub struct</span><span class=\"color-white\">Point {</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">    <span class=\"color-red\">x</span><span class=\"color-white\">:</span><span class=\"color-violet\">f32</span><span class=\"color-white\">,</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">    <span class=\"color-red\">y</span><span class=\"color-white\">:</span><span class=\"color-violet\">f32</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-white\">}</span></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"></span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\"><span class=\"color-violet\">pub const</span><span class=\"color-gold\">CONST</span><span class=\"color-white\">:</span><span class=\"color-violet\">u32</span><span class=\"color-white\">=</span><span class=\"color-gold\">1</span><span class=\"color-white\">;</span></span>
    </span>
</code></pre>

<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">```rust</span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">fn main() {</span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">    println!(&quot;Hello World!&quot;);</span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">}</span>
    </span>
    <span class=\"code-fence-row\">
        <span class=\"code-fence-code\">```</span>
    </span>
</code></pre>
"), ("
# Copy Button test

```copy_button
Nothing1
```

```copy_button
Nothing2
```
", "
<h1 id=\"copy-button-test\">Copy Button test</h1>

<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\">Nothing1</span></span>
</code>
    <button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(0)\">Copy</button>
</pre>

<pre class=\"fenced-code-block\"><code>
    <span class=\"code-fence-row\"><span class=\"code-fence-code\">Nothing2</span></span>
</code>
    <button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(1)\">Copy</button>
</pre>

<script>
const fenced_code_block_contents = [\"Nothing1\", \"Nothing2\"];

function copy_code_to_clipboard(index) {
    navigator.clipboard.writeText(fenced_code_block_contents[index]);
}
</script>
"), ("
```copy_button
\"
```
", "
<pre class=\"fenced-code-block\">
    <code>
        <span class=\"code-fence-row\"><span class=\"code-fence-code\">&quot;</span></span>
    </code>
    <button class=\"copy-fenced-code\" onclick=\"copy_code_to_clipboard(0)\">Copy</button>
</pre>

<script>
const fenced_code_block_contents = [\"\\\"\"];

function copy_code_to_clipboard(index) {
    navigator.clipboard.writeText(fenced_code_block_contents[index]);
}
</script> 
"), ("
```
<
\\
\\<
```

```c
<
\\
\\<
```
",/* test case A */ "
<pre class=\"fenced-code-block\">
    <code>
        <span class=\"code-fence-row\">
            <span class=\"code-fence-code\">&lt;</span>
        </span>\n
        <span class=\"code-fence-row\">
            <span class=\"code-fence-code\">\\</span>
        </span>\n
        <span class=\"code-fence-row\">
            <span class=\"code-fence-code\">\\&lt;</span>
        </span>\n
    </code>
</pre>

<pre class=\"fenced-code-block\">
    <code>
        <span class=\"code-fence-row\">
            <span class=\"code-fence-code\">
                <span class=\"color-white\">&lt;</span>
            </span>
        </span>\n
        <span class=\"code-fence-row\">
            <span class=\"code-fence-code\">
                <span class=\"color-white\">\\</span>
            </span>
        </span>\n
        <span class=\"code-fence-row\">
            <span class=\"code-fence-code\">
                <span class=\"color-white\">\\&lt;</span>
            </span>
        </span>\n
    </code>
</pre>
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

        if remove_whitespaces(&into_v32(&rendered)) != remove_whitespaces(&into_v32(html)) {
            panic!("{md} \n\n {rendered} \n\n {rendered:?}");
        }

    }

}