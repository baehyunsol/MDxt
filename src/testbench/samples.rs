pub fn samples() -> Vec<(&'static str, &'static str)> {
    vec![set1(), set2()]
}

fn set1() -> (&'static str, &'static str) {
    ("
# Test document

This is a sample document for testing purpose. The rendered result of this document is written manually, and tested by various web-browsers.

[[toc]]

## HMD

In most cases, a valid [Github Flavored Markdown](https://github.github.com/gfm/) document is a valid HMD document. HMD adds extra syntaxes, including [macros](#macros), sub~scripts~, super^scripts^, ~_underlines_~, ~~deletion lines~~ and a few more.

If you want to know more about it, checkout its [repo](https://github.com/baehyunsol/HMD).

### Macros

[[box]]

[[red]] This text is red. [[/red]]

[[big]] This text is big. [[/big]]

[[center]] This text is center-aligned. [[/center]]

[[cetner]] This macro is broken. Broken macros are treated like normal paragraphs. [[/cetner]]

`[[blank]]` for `&nbsp;` and `[[br]]` for `<br/>`.

[[/box]]

### Unicode

The HMD engine uses UTF-8 encoding, which means it supports 한글, ひらがな, and 漢字. If your keyboard doesn't support those characters, use `[[char]]` macro. [[char = 44032]]

### Tables

HMD's table syntax resembles that of [Github Flavored Markdown](https://github.github.com/gfm/). It has a few more extra syntaxes.

```
|Left aligned Column |Centered Column |Right aligned Column |
|:-------------------|:--------------:|--------------------:|
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
```

|Left aligned Column |Centered Column |Right aligned Column |
|:-------------------|:--------------:|--------------------:|
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |

Don't forget to write `|` at the end and the start of each row.
", "
\n<h1 id=\"Testdocument\">Test document</h1>\n\n<p>This is a sample document for testing purpose. The rendered result of this document is written manually, and tested by various web-browsers.</p>\n\n<div class=\"box\">\n<p><div class=\"align_center\"><span class=\"size_big\">Table of Contents</span></div></p>\n<div class=\"align_left\">\n<ul><li><a href=\"#Testdocument\">Test document</a><ul><li><a href=\"#HMD\">HMD</a><ul><li><a href=\"#Macros\">Macros</a></li><li><a href=\"#Unicode\">Unicode</a></li><li><a href=\"#Tables\">Tables</a></li></ul></li></ul></li></ul>\n</div>\n</div>\n\n\n\n<h2 id=\"HMD\">HMD</h2>\n\n<p>In most cases, a valid <a href=\"https://github.github.com/gfm/\">Github Flavored Markdown</a> document is a valid HMD document. HMD adds extra syntaxes, including <a href=\"#macros\">macros</a>, sub<sub>scripts</sub>, super<sup>scripts</sup>, <u>underlines</u>, <del>deletion lines</del> and a few more.</p>\n\n<p>If you want to know more about it, checkout its <a href=\"https://github.com/baehyunsol/HMD\">repo</a>.</p>\n\n<h3 id=\"Macros\">Macros</h3>\n\n<div class=\"box\">\n\n<p><span class=\"color_red\"> This text is red. </span></p>\n\n<p><span class=\"size_big\"> This text is big. </span></p>\n\n<p><div class=\"align_center\"> This text is center-aligned. </div></p>\n\n<p>[[cetner]] This macro is broken. Broken macros are treated like normal paragraphs. [[/cetner]]</p>\n\n<p><code class=\"short\">[[blank]]</code> for <code class=\"short\">&amp;nbsp;</code> and <code class=\"short\">[[br]]</code> for <code class=\"short\">&lt;br/></code>.</p>\n\n</div>\n\n<h3 id=\"Unicode\">Unicode</h3>\n\n<p>The HMD engine uses UTF-8 encoding, which means it supports 한글, ひらがな, and 漢字. If your keyboard doesn&apos;t support those characters, use <code class=\"short\">[[char]]</code> macro. &#44032;</p>\n\n<h3 id=\"Tables\">Tables</h3>\n\n<p>HMD&apos;s table syntax resembles that of <a href=\"https://github.github.com/gfm/\">Github Flavored Markdown</a>. It has a few more extra syntaxes.</p>\n\n<pre><code>|Left aligned Column |Centered Column |Right aligned Column |\n|:-------------------|:--------------:|--------------------:|\n|        Left        |     Center     |        Right        |\n|        Left        |     Center     |        Right        |\n|        Left        |     Center     |        Right        |\n|        Left        |     Center     |        Right        |\n|        Left        |     Center     |        Right        |</code></pre>\n\n<table><thead><th>Left aligned Column </th><th>Centered Column </th><th>Right aligned Column </th></thead><tbody><tr><td class=\"align_left\">        Left        </td><td class=\"align_center\">     Center     </td><td class=\"align_right\">        Right        </td></tr><tr><td class=\"align_left\">        Left        </td><td class=\"align_center\">     Center     </td><td class=\"align_right\">        Right        </td></tr><tr><td class=\"align_left\">        Left        </td><td class=\"align_center\">     Center     </td><td class=\"align_right\">        Right        </td></tr><tr><td class=\"align_left\">        Left        </td><td class=\"align_center\">     Center     </td><td class=\"align_right\">        Right        </td></tr><tr><td class=\"align_left\">        Left        </td><td class=\"align_center\">     Center     </td><td class=\"align_right\">        Right        </td></tr></tbody></table>\n\n<p>Don&apos;t forget to write <code class=\"short\">|</code> at the end and the start of each row.</p>\n\n")
}

fn set2() -> (&'static str, &'static str) {
    ("
# Valid header
Valid header
# Valid header

# Valid header

Valid header

# Valid header

# [[red]] Red header [[/red]]

# `Code header`", "
<h1>Valid header</h1><p>Valid header</p><h1>Valid header</h1><h1>Valid header</h1><p>Valid header</p><h1>Valid header</h1><h1><div class=\"color_red\">Red header</div></h1><h1><code class=\"short\">Code header</code></h1>
")

}

fn set3() -> (&'static str, &'static str) {
    ("[invalid_ref], [valid_ref], [[invalid]](https://github.com), [valid](https://github.com), [*valid*](https://github.com)

---

이거 걍 다 분리해서 testbench에 넣으셈... 그게 훨씬 나을 듯... ㅠㅠ

![invalid_ref], ![valid_ref], ![[invalid]](https://github.com), ![valid](https://github.com), ![*valid*](https://github.com)

---

[*valid ref*][valid_ref], [invalid ref][invalid_ref], ![*valid ref*][valid_ref], ![invalid ref][invalid_ref]

---

nested links are not allowed

[link [valid_ref]][valid_ref], [link [another link](https://github.com)][valid_ref], [link [another link](https://github.com)](https://github.com)

---

collapsed links... are these used? nobody would even know it...

[valid_ref][], [invalid_ref][], ![valid_ref][], ![invalid_ref][]

---

Consecutive link ref defs without blank lines are ok, but they cannot interrupt a paragraph.
Link ref defs are not rendered whether or not they're used.

[valid_ref]: https://github.com
[unused_ref]: https://github.com
", "
")
}