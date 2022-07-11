pub fn samples() -> Vec<(&'static str, &'static str)> {
    vec![set1(), set2(), set3()]
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

", "
")
}

fn set3() -> (&'static str, &'static str) {
    ("


> 1
  > 1.1
1.1
1.1
> 2

> 1
  > 1.1
> 2
2
> 3
  > 3.1
    > 3.1.1
  > 3.2
    > 3.2.1
    > 3.2.2
> 4
", "
")
}