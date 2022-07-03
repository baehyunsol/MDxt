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
")
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
")

}

fn set3() -> (&'static str, &'static str) {
    ("
[[math]] `a codespan in a math` [[/math]] `[[math]] a math in a codespan [[/math]]`

`[[math]] a codespan before a math`[[/math]] [[math]] `a codespan after a math [[/math]]`

[[math]] `a codespan after a math [[/math]]` `[[math]] a codespan before a math`[[/math]]

[[math]] a * b * c = abc [[/math]]

*inter-math inline element [[math]] F * G = int{-infty}{infty} F(theta)G(k - theta) d theta [[/math]]

|          a          |          b          |          c          |
|---------------------|---------------------|---------------------|
| math inside a table | [[math]] |a| [[/math]] | the pipe shouldn't break a cell |
|         `|`         | a pipe in a *cell   | inter-cell highlights* |
", "
")
}