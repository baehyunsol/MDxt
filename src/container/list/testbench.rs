use crate::utils::{into_v16, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn list_samples() -> Vec<(String, String)> {
    let result = vec![
        ("
- 1
- 1
- 1
- 1
- 1
", "
<ul>
    <li>1</li>
    <li>1</li>
    <li>1</li>
    <li>1</li>
    <li>1</li>
</ul>
"), ("
- 1
 - 1
  - 1
   - 1
    - 1
", "
<ul>
    <li>1</li>
    <li>1</li>
    <li>1</li>
    <li>1</li>
    <li>1</li>
</ul>
"), ("
    - 1
   - 1
  - 1
 - 1
- 1
", "
<ul>
    <li>1</li>
    <li>1</li>
    <li>1</li>
    <li>1</li>
    <li>1</li>
</ul>
"), ("
- 1
  - 2
    - 3
      - 4
        - 5
   - 3
     - 4
     - 4
", "
<ul>
    <li>1
        <ul>
            <li>2
                <ul>
                    <li>3
                        <ul>
                            <li>4
                                <ul>
                                    <li>5</li>
                                </ul>
                            </li>
                        </ul>
                    </li>
                    <li>3
                        <ul>
                            <li>4</li>
                            <li>4</li>
                        </ul>
                    </li>
                </ul>
            </li>
        </ul>
    </li>
</ul>
"), ("
- 1
  - 2
      - 3
      - 3
      - 3
    - 3
    - 3
      - 4
      - 4
", "
<ul>
    <li>1
        <ul>
            <li>2
                <ul>
                    <li>3</li>
                    <li>3</li>
                    <li>3</li>
                </ul>
                <ul>
                    <li>3</li>
                    <li>3
                        <ul>
                            <li>4</li>
                            <li>4</li>
                        </ul>
                    </li>
                </ul>
            </li>
        </ul>
    </li>
</ul>
"), ("
- 1
  - 2
      - 3
      - 3
      - 3
   - 2
   - 2
     - 3
     - 3
", "
<ul>
    <li>1
        <ul>
            <li>2
                <ul>
                    <li>3</li>
                    <li>3</li>
                    <li>3</li>
                </ul>
            </li>
            <li>2</li>
            <li>2
                <ul>
                    <li>3</li>
                    <li>3</li>
                </ul>
            </li>
        </ul>
    </li>
</ul>
"), ("
        - 1
      - 1
    - 1
  - 1
- 1
", "
<ul>
    <li>1</li>
    <li>1</li>
    <li>1</li>
    <li>1</li>
    <li>1</li>
</ul>
"), ("
        - 1
      - 1
        - 2
        - 2
        - 2
    - 1
  - 1
- 1
", "
<ul>
    <li>1</li>
    <li>1
        <ul>
            <li>2</li>
            <li>2</li>
            <li>2</li>
        </ul>
    </li>
    <li>1</li>
    <li>1</li>
    <li>1</li>
</ul>
"), ("
- 1
1
  1
  - 2
2
  2
    - 3
3
  3

    - 1
  - 1
- 1
 - 
  - 2
", "
<ul>
    <li>1 1 1
        <ul>
            <li>2 2 2
                <ul>
                    <li>3 3 3</li>
                </ul>
            </li>
        </ul>
    </li>
</ul>
<ul>
    <li>1</li>
    <li>1</li>
    <li>1 - 
        <ul>
            <li>2</li>
        </ul>
    </li>
</ul>
"), ("
1. 1
1. 1
1. 1
", "
<ol type=\"1\">
    <li>1</li>
    <li>1</li>
    <li>1</li>
</ol>
"), ("
2. 2
2. 2
2. 2
", "
<ol type=\"1\" start=\"2\">
    <li>2</li>
    <li>2</li>
    <li>2</li>
</ol>
"), ("
99999999999999. 2
99999999999999. 2
99999999999999. 2
", "
<p>99999999999999. 2 99999999999999. 2 99999999999999. 2</p>
"), ("
a. a
a. a
a. a
", "
<ol type=\"a\">
    <li>a</li>
    <li>a</li>
    <li>a</li>
</ol>
"), ("
b. b
b. b
b. b
", "
<p>b. b b. b b. b</p>
"), ("
A. A
A. A
A. A
", "
<ol type=\"A\">
    <li>A</li>
    <li>A</li>
    <li>A</li>
</ol>
"), ("
B. B
B. B
B. B
", "
<p>B. B B. B B. B</p>
"), ("
i. i
i. i
i. i
", "
<ol type=\"i\">
    <li>i</li>
    <li>i</li>
    <li>i</li>
</ol>
"), ("
I. I
I. I
I. I
", "
<ol type=\"I\">
    <li>I</li>
    <li>I</li>
    <li>I</li>
</ol>
"), ("
II. II
II. II
II. II
", "
<p>II. II II. II II. II</p>
"), ("
- nums
  1. 1
  2. 2
  3. 3
- alphabets
  a. a
  a. b
  a. c
- romans
  i. i
  i. ii
  i. iii
- ul
  - 1
  - 2
  - 3
", "<ul>
<li>nums
    <ol type=\"1\">
        <li>1</li>
        <li>2</li>
        <li>3</li>
    </ol>
</li>
<li>alphabets
    <ol type=\"a\">
        <li>a</li>
        <li>b</li>
        <li>c</li>
    </ol>
</li>
<li>romans
    <ol type=\"i\">
        <li>i</li>
        <li>ii</li>
        <li>iii</li>
    </ol>
</li>
<li>ul
    <ul>
        <li>1</li>
        <li>2</li>
        <li>3</li>
    </ul>
</li>
</ul>
"), ("
- 1
---
- 1
# 1
- 1
```
nothing
```
", "
<ul>
    <li>1</li>
</ul>

<hr/>

<ul>
    <li>1</li>
</ul>

<h1 id=\"1\">1</h1>

<ul>
    <li>1</li>
</ul>

<pre><code>
<span class=\"code_fence_row\"><span class=\"code_fence_code\">nothing</span></span>
</code></pre>
"), ("
interrupt
paragraph
- 1
- 1
", "
<p>interrupt paragraph</p>

<ul>
    <li>1</li>
    <li>1</li>
</ul>
"), ("
- 1
  - 1.1
1.1
1.1
- 2

- 1
  - 1.1
- 2
2
- 3
  - 3.1
    - 3.1.1
  - 3.2
    - 3.2.1
    - 3.2.2
- 4
", "
<ul>
    <li>1
        <ul>
            <li>1.1 1.1 1.1</li>
        </ul>
    </li>
    <li>2</li>
</ul>

<ul>
    <li>1
        <ul>
            <li>1.1</li>
        </ul>
    </li>
    <li>2 2</li>
    <li>3
        <ul>
            <li>3.1
                <ul>
                    <li>3.1.1</li>
                </ul>
            </li>
            <li>3.2
                <ul>
                    <li>3.2.1</li>
                    <li>3.2.2</li>
                </ul>
            </li>
        </ul>
    </li>
    <li>4</li>
</ul>
"), ("
- [ ] 1 (valid)
- [ ]
- [ ]3
- [X] 4 (valid)
- [  ] 5
- [] 6
-   [ ] 7 (valid)
- [^] 8 (valid)
", "
<ul>
    <li><div class=\"unchecked_box\"></div>1 (valid)</li>
    <li>[ ]</li>
    <li>[ ]3</li>
    <li><div class=\"checked_box\"><span class=\"checkmark\"></span></div>4 (valid)</li>
    <li>[  ] 5</li>
    <li>[] 6</li>
    <li><div class=\"unchecked_box\"></div>7 (valid)</li>
    <li><div class=\"checked_box\"><span class=\"triangle\"></span></div>8 (valid)</li>
</ul>
")
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn list_test() {
    for (md, html) in list_samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v16(&rendered)) != remove_whitespaces(&into_v16(html)) {
            panic!("{} \n\n {}", md, rendered);
        }

    }

}