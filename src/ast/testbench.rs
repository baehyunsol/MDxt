use super::line::Line;
use crate::utils::{from_v16, into_v16, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn mdxt_samples() -> Vec<(String, String)> {
    let result = vec![("
[[red]]
Red
[[/red]]
", "
<div class=\"color-red\">
<p>Red</p>
</div>
"
), ("
[[red]]

Red

[[/red]]
", "
<div class=\"color-red\">
<p>Red</p>
</div>
"
), ("
[[red]]

Not Red
", "
<p>[[red]]</p>

<p>Not Red</p>
"
), ("
Cannot interrupt a paragraph
[[red]]
Red
[[/red]]
", "
<p>Cannot interrupt a paragraph</p>
<div class=\"color-red\">
    <p>Red</p>
</div>
"
), ("
[[div, class=foo]]

foo class

[[/div]]
", "
<div class=\"foo\"><p>foo class</p></div>
"), ("
[[div, class=foo]]
foo class
[[/div]]
", "
<div class=\"foo\"><p>foo class</p></div>
"), ("
[[div, class=foo]]
foo class

[[/div]]
", "
<div class=\"foo\"><p>foo class</p></div>
"), ("
[[div, class=foo]]

foo class
[[/div]]
", "
<div class=\"foo\"><p>foo class</p></div>
"), (
"
[[br]]

[[blank]]

[[br]]
", "
<p><br/></p><p>&nbsp;</p><p><br/></p>
"
), ("
[[giant]]

[[center]]

Giant and Center

[[/center]]

[[/giant]]
", "
<div class=\"size-giant\">
    <div class=\"align-center\">
        <p>Giant and Center</p>
    </div>
</div>
"), ("
[[giant]]

[[center]]

Macros are not properly closed.

[[/giant]]

[[/center]]
", "
<p>[[giant]]</p>
<p>[[center]]</p>
<p>Macros are not properly closed.</p>
<p>[[/giant]]</p>
<p>[[/center]]</p>
"), ("
[[highlight = red]]

[[span, class = foo, id = bar]]

Red Span

[[/span]]

[[/highlight]]
", "
<div class=\"highlight-red\">
    <span class=\"foo\" id=\"bar\">
        <p>Red Span</p>
    </span>
</div>
"), ("
[[box]]

A paragraph in a box.

[[box]]

A paragraph in a box in a box.

[[/box]]

[[box, no border]]

A paragraph in a borderless box in a box.

[[/box]]

[[/box]]
", "
<div class=\"box\">
    <p>A paragraph in a box.</p>
    <div class=\"box\">
        <p>A paragraph in a box in a box.</p>
    </div>
    <div class=\"box no-border\">
        <p>A paragraph in a borderless box in a box.</p>
    </div>
</div>
"), ("
# Link Escape Test

[escape test &](&)
", "
<h1 id=\"link-escape-test\">Link Escape Test</h1>

<p><a href=\"&amp;\">escape test &amp;</a></p>
"), ("
# Link Esacpe Test2

[&][link1]

[link1]

[link1]: a&b
", "
<h1 id=\"link-esacpe-test2\">Link Esacpe Test2</h1>
<p><a href=\"a&amp;b\">&amp;</a></p>
<p><a href=\"a&amp;b\">link1</a></p>
"), ("
[[box]]

div box

[[/box]]

[[box]] span box [[/box]]
", "
<div class=\"box\">
    <p> div box </p>
</div>

<p><span class=\"box\"> span box </span></p>
"), ("
### 123

[[char = big sigma]]: a finite set of symbols.

- 456
", "
<h3 id=\"123\">123</h3>
<p>&Sigma;: a finite set of symbols.</p>
<ul><li>456</li></ul>
"), (
"
[[box]]
- a
[[/box]]
",
"<div class=\"box\">
    <ul><li>a</li></ul>
</div>"
), ("
Multiline macros start a new paragraph. See below.

a
[[box]]
b
[[/box]]
c
[[box]]
d
[[/box]]
e
", "
<p>Multiline macros start a new paragraph. See below.</p>
<p>a</p>
<div class=\"box\">
    <p>b</p>
</div>
<p>c</p>
<div class=\"box\">
    <p>d</p>
</div>
<p>e</p>
"), ("
> br  
> br\\
> no br
> no br
> br  
> br\\
> no br
> no br

- br  
no br
- br\\
no br
- no br
no br
- no br
no br
- br  
no br
- br\\
no br
- no br
no br
- no br
no br
", "
<blockquote>br<br/> br<br/> no br no br br<br/> br<br/> no br no br </blockquote>
<ul>
    <li>br<br/> no br</li>
    <li>br<br/> no br</li>
    <li>no br no br</li>
    <li>no br no br</li>
    <li>br<br/> no br</li>
    <li>br<br/> no br</li>
    <li>no br no br</li>
    <li>no br no br</li>
</ul>
"),
("
new\r\n
lines\r\n
", "
<p>new</p>
<p>lines</p>
"), ("
# Escape rules

[[math]]text{>}[[/math]]

[[math]]`text{>}`[[/math]]

`[[math]]text{>}[[/math]]`

[[math]]>[[/math]]

[[math]]`>`[[/math]]

`[[math]]>[[/math]]`

[[math]]text{&gt;}[[/math]]

[[math]]`text{&gt;}`[[/math]]

`[[math]]text{&gt;}[[/math]]`

[[math]]&gt;[[/math]]

[[math]]`&gt;`[[/math]]

`[[math]]&gt;[[/math]]`

[[math]]text{&amp;gt;}[[/math]]

[[math]]`text{&amp;gt;}`[[/math]]

`[[math]]text{&amp;gt;}[[/math]]`

[[math]]&amp;gt;[[/math]]

[[math]]`&amp;gt;`[[/math]]

`[[math]]&amp;gt;[[/math]]`

[[math]]5\\>4[[/math]]

[[math]]`5\\>4`[[/math]]

`[[math]]5\\>4[[/math]]`

[[math]]<a>a?</a>[[/math]]

[[math]]`<a>a?</a>`[[/math]]

`[[math]]<a>a?</a>[[/math]]`
", "
<h1 id=\"escape-rules\">Escape rules</h1>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mtext>&gt;</mtext>
    </math>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>`</mo>
        <mtext>&gt;</mtext>
        <mo>`</mo>
    </math>
</p>

<p>
    <code class=\"inline-code-span\">[[math]]text{&gt;}[[/math]]</code>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>&gt;</mo>
    </math>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>`</mo>
        <mo>&gt;</mo>
        <mo>`</mo>
    </math>
</p>

<p>
    <code class=\"inline-code-span\">[[math]]&gt;[[/math]]</code>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mtext>&amp;gt;</mtext>
    </math>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>`</mo>
        <mtext>&amp;gt;</mtext>
        <mo>`</mo>
    </math>
</p>

<p>
    <code class=\"inline-code-span\">[[math]]text{&amp;gt;}[[/math]]</code>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>&amp;</mo>
        <mo>&#62;</mo>
        <mo>;</mo>
    </math>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>`</mo>
        <mo>&amp;</mo>
        <mo>&#62;</mo>
        <mo>;</mo>
        <mo>`</mo>
    </math>
</p>

<p>
    <code class=\"inline-code-span\">[[math]]&amp;gt;[[/math]]</code>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mtext>&amp;amp;gt;</mtext>
    </math>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>`</mo>
        <mtext>&amp;amp;gt;</mtext>
        <mo>`</mo>
    </math>
</p>

<p>
    <code class=\"inline-code-span\">[[math]]text{&amp;amp;gt;}[[/math]]</code>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>&amp;</mo>
        <mi>amp</mi>
        <mo>;</mo>
        <mo>&#62;</mo>
        <mo>;</mo>
    </math>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>`</mo>
        <mo>&amp;</mo>
        <mi>amp</mi>
        <mo>;</mo>
        <mo>&#62;</mo>
        <mo>;</mo>
        <mo>`</mo>
    </math>
</p>

<p>
    <code class=\"inline-code-span\">[[math]]&amp;amp;gt;[[/math]]</code>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mn>5</mn>
        <mo>&gt;</mo>
        <mn>4</mn>
    </math>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>`</mo>
        <mn>5</mn>
        <mo>\\</mo>
        <mo>&gt;</mo>
        <mn>4</mn>
        <mo>`</mo>
    </math>
</p>

<p>
    <code class=\"inline-code-span\">[[math]]5\\>4[[/math]]</code>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>&lt;</mo>
        <mi>a</mi>
        <mo>&gt;</mo>
        <mi>a</mi>
        <mo>?</mo>
        <mo>&lt;</mo>
        <mo>/</mo>
        <mi>a</mi>
        <mo>&gt;</mo>
    </math>
</p>

<p>
    <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
        <mo>`</mo>
        <mo>&lt;</mo>
        <mi>a</mi>
        <mo>&gt;</mo>
        <mi>a</mi>
        <mo>?</mo>
        <mo>&lt;</mo>
        <mo>/</mo>
        <mi>a</mi>
        <mo>&gt;</mo>
        <mo>`</mo>
    </math>
</p>

<p>
    <code class=\"inline-code-span\">[[math]]&lt;a&gt;a?&lt;/a&gt;[[/math]]</code>
</p>
")
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn mdxt_test() {
    for (md, html) in mdxt_samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v16(&rendered)) != remove_whitespaces(&into_v16(html)) {
            panic!("{} \n\n {}", md, rendered);
        }

    }

}

const IS_HEADER: usize = 0;
const IS_EMPTY: usize = 1;
const IS_CODEFENCE_BEGIN: usize = 2;
const IS_CODEFENCE_END: usize = 3;
const IS_TABLE_ROW: usize = 4;
const IS_TABLE_DELIMITER: usize = 5;
const IS_THEMATIC_BREAK: usize = 6;
const IS_BLOCKQUOTE: usize = 7;
const IS_UNORDERED_LIST: usize = 8;
const IS_ORDERED_LIST: usize = 9;
const IS_LINK_OR_FOOTNOTE_REFERENCE_DEFINITION: usize = 10;
const IS_MULTILINE_MACRO: usize = 11;

fn line_samples() -> Vec<(Line, Vec<usize>)> {
    vec![
        (Line::from_raw_string("# Header"), vec![IS_HEADER]),
        (Line::from_raw_string(""), vec![IS_EMPTY]),
        (Line::from_raw_string(" "), vec![IS_EMPTY]),
        (Line::from_raw_string("```"), vec![IS_CODEFENCE_BEGIN, IS_CODEFENCE_END]),
        (Line::from_raw_string("```rust"), vec![IS_CODEFENCE_BEGIN]),
        (Line::from_raw_string("[[box]]"), vec![IS_MULTILINE_MACRO]),
        (Line::from_raw_string("[[box]] box"), vec![]),
        (Line::from_raw_string("[[char = big sigma]]: a finite set of symbols."), vec![]),
        (Line::from_raw_string("---"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string(" ---"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string("  ---"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string("    ---"), vec![]),
        (Line::from_raw_string("***"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string(" ***"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string("  ***"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string("    ***"), vec![]),
        (Line::from_raw_string("___"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string(" ___"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string("  ___"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string("    ___"), vec![]),
        (Line::from_raw_string(" - - -"), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string(" - - - "), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string(" - * - "), vec![IS_UNORDERED_LIST]),
        (Line::from_raw_string(" -           -         - "), vec![IS_THEMATIC_BREAK]),
        (Line::from_raw_string("-"), vec![IS_UNORDERED_LIST]),
        (Line::from_raw_string("- "), vec![IS_UNORDERED_LIST]),
        (Line::from_raw_string("- abc"), vec![IS_UNORDERED_LIST]),
        (Line::from_raw_string("-abc"), vec![]),
    ]
}

#[test]
fn line_predicate_test() {

    let mut failures = vec![];
    let samples = line_samples();

    for (line, trues) in samples.iter() {
        let mut predicates = vec![false; IS_MULTILINE_MACRO + 1];

        for true_ in trues {
            predicates[*true_] = true;
        }

        if line.is_header() != predicates[IS_HEADER] {
            let failure = format!(
                "line: {:?}\npredicate: is_header\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_HEADER],
                line.is_header()
            );

            failures.push(failure);
        }

        if line.is_empty() != predicates[IS_EMPTY] {
            let failure = format!(
                "line: {:?}\npredicate: is_empty\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_EMPTY],
                line.is_empty()
            );

            failures.push(failure);
        }

        if line.is_code_fence_begin() != predicates[IS_CODEFENCE_BEGIN] {
            let failure = format!(
                "line: {:?}\npredicate: is_code_fence_begin\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_CODEFENCE_BEGIN],
                line.is_code_fence_begin()
            );

            failures.push(failure);
        }

        if line.is_code_fence_end() != predicates[IS_CODEFENCE_END] {
            let failure = format!(
                "line: {:?}\npredicate: is_code_fence_end\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_CODEFENCE_END],
                line.is_code_fence_end()
            );

            failures.push(failure);
        }

        if line.is_table_row() != predicates[IS_TABLE_ROW] {
            let failure = format!(
                "line: {:?}\npredicate: is_table_row\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_TABLE_ROW],
                line.is_table_row()
            );

            failures.push(failure);
        }

        if line.is_table_delimiter() != predicates[IS_TABLE_DELIMITER] {
            let failure = format!(
                "line: {:?}\npredicate: is_table_delimiter\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_TABLE_DELIMITER],
                line.is_table_delimiter()
            );

            failures.push(failure);
        }

        if line.is_thematic_break() != predicates[IS_THEMATIC_BREAK] {
            let failure = format!(
                "line: {:?}\npredicate: is_thematic_break\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_THEMATIC_BREAK],
                line.is_thematic_break()
            );

            failures.push(failure);
        }

        if line.is_blockquote() != predicates[IS_BLOCKQUOTE] {
            let failure = format!(
                "line: {:?}\npredicate: is_blockquote\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_BLOCKQUOTE],
                line.is_blockquote()
            );

            failures.push(failure);
        }

        if line.is_unordered_list() != predicates[IS_UNORDERED_LIST] {
            let failure = format!(
                "line: {:?}\npredicate: is_unordered_list\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_UNORDERED_LIST],
                line.is_unordered_list()
            );

            failures.push(failure);
        }

        if line.is_ordered_list() != predicates[IS_ORDERED_LIST] {
            let failure = format!(
                "line: {:?}\npredicate: is_ordered_list\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_ORDERED_LIST],
                line.is_ordered_list()
            );

            failures.push(failure);
        }

        if line.is_link_or_footnote_reference_definition() != predicates[IS_LINK_OR_FOOTNOTE_REFERENCE_DEFINITION] {
            let failure = format!(
                "line: {:?}\npredicate: is_link_or_footnote_reference_definition\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_LINK_OR_FOOTNOTE_REFERENCE_DEFINITION],
                line.is_link_or_footnote_reference_definition()
            );

            failures.push(failure);
        }

        if line.is_multiline_macro() != predicates[IS_MULTILINE_MACRO] {
            let failure = format!(
                "line: {:?}\npredicate: is_multiline_macro\ndesired: {}, result: {}",
                from_v16(&line.to_raw()),
                predicates[IS_MULTILINE_MACRO],
                line.is_multiline_macro()
            );

            failures.push(failure);
        }

    }

    if failures.len() > 0 {
        panic!(
            "{} out of {} line_predicate_test case(s) have failed!\n{}",
            failures.len(),
            (IS_MULTILINE_MACRO + 1) * samples.len(),
            failures.join("\n\n-----------------------------------\n")
        );
    }

}