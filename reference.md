---
date: [2022, 7, 26]
tags: [mdxt, reference, documentation]
---

# MDxt Reference

MDxt is an extended version of Markdown.

[[anchor, id = toc sample]][[/anchor]]

| Table of Contents |
|-------------------|
|!![[collapsible]]  |
| [[toc]]           |

## Inline Elements

### `Code spans`

`` `abc` `` is rendered to `<code class="short">abc</code>`. If the first and the last character of the code span is a whitespace, both are ignored. If a code span opens with n `` ` ``s, it has to be closed with the same number n of `` ` ``s.

### *Italic*

`*abc*` is rendered to `<em>abc</em>`. The inner text may not start/end with whitespace(s).

### **Bold**

`**abc**` is rendered to `<strong>abc</strong>`. The inner text may not start/end with whitespace(s).

### ~_Underline_~

`~_abc_~` is rendered to `<u>abc</u>`. The inner text may not start/end with whitespace(s).

### ~~Deletion~~

`~~abc~~` is rendered to `<del>abc</del>`. The inner text may not start/end with whitespace(s).

### ~Subscript~

`~abc~` is rendered to `<sub>abc</sub>`. The inner text may not start/end with whitespace(s).

### ^Superscript^

`^abc^` is rendered to `<sup>abc</sup>`. The inner text may not start/end with whitespace(s).

### Links

### Images

A valid link after a bang(!) character is rendered to an `img` tag.

`![abc](def)` is rendered to `<img src="def" alt="abc">`.

### Footnotes

```
This is a footnote.[^A]

This is another footnote.[^B]

[^A]: Hi, there!
[^B]: Hello!
```

This is a footnote.[^A]

This is another footnote.[^B]

[^A]: Hi, there!
[^B]: Hello!

## Containers

### Headers

`### Headers` is rendered to `<h3>Headers</h3>`.

### Tables

#### Column Alignments

#### Multiline Table Head

#### Colspan

#### Collapsible Tables

```
| Click Me! (Default shown)              |
|----------------------------------------|
|!![[collapsible, default=shown]]        |
| Hi, there!                             |

| Click Me! (Default hidden)            |
|---------------------------------------|
|!![[collapsible, default=hidden]]      |
| Hi, there!                            |
```
| Click Me! (Default shown)              |
|----------------------------------------|
|!![[collapsible, default=shown]]        |
| Hi, there!                             |

| Click Me! (Default hidden)            |
|---------------------------------------|
|!![[collapsible, default=hidden]]      |
| Hi, there!                            |

### Lists

#### Task list

```
- [ ] Unchecked
- [X] Checked
- [^] Not yet
```

- [ ] Unchecked
- [X] Checked
- [^] Not yet

### Fenced Code Blocks

Fenced code blocks are rendered to `<table>` tags in html. Be aware if you're writing custom css files for mdxt.

````
```rust, line_num, highlight(6, 17, 22)
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
    println!("Hello World!\n");
    println!("{:?}", 3 + 4);
}

pub struct Point {
    x: f32,
    y: f32
}

pub const CONST: u32 = 1;
```
````

```rust, line_num, highlight(6, 17, 22)
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
    println!("Hello World!\n");
    println!("{:?}", 3 + 4);
}

pub struct Point {
    x: f32,
    y: f32
}

pub const CONST: u32 = 1;
```

### Blockquotes

> This is a blockquote.
>> This is another blockquote.

### Unlike GFM...

MDxt doesn't support setext headers and indented code blocks.

## Macros

Macros are inline elements. Which means an opening macro and the closing one has to be in the same paragraph. But there are many cases where you want to apply macros to multiple paragraphs. Read [multiline macro] section for that.

A valid macro consists of `A-Z`, `a-z`, `0-9`, `=`, `,`, `_`, and ` `. If a double square bracket contains invalid characters, that won't be parsed as a macro. Whitespaces and `_`s inside macros are ignored, and all the alphabet characters are lowered. That means `[[box, no_border]]` and `[[box, n o border]]` are exactly the same macro.

### Colors

It has 14 colors: black, dark, gray, lightgray, white, red, green, blue, aqua, emerald, violet, pink, grassgreen, and gold.

| MDxt                                 | html                                              | output                              |
|--------------------------------------|---------------------------------------------------|-------------------------------------|
|\[[black]]black[[/black]]          | <span class="color_black">black</span>          | [[black]]black[[/black]]         |
|\[[dark]]dark[[/dark]]              | <span class="color_dark">dark</span>             | [[dark]]dark[[/dark]]             |
|\[[gray]]gray[[/gray]]              | <span class="color_gray">gray</span>             | [[gray]]gray[[/gray]]             |
|\[[lightgray]]lightgray[[/lightgray]]    | <span class="color_lightgray">lightgray</span>        | [[lightgray]]lightgray[[/lightgray]]   |
|\[[white]]white[[/white]]            | <span class="color_white">white</span>            | [[white]]white[[/white]]           |
|\[[red]]red[[/red]]                | <span class="color_red">red</span>              | [[red]]red[[/red]]               |
|\[[green]]green[[/green]]            | <span class="color_green">green</span>            | [[green]]green[[/green]]           |
|\[[blue]]blue[[/blue]]              | <span class="color_blue">blue</span>             | [[blue]]blue[[/blue]]             |
|\[[aqua]]aqua[[/aqua]]              | <span class="color_aqua">aqua</span>             | [[aqua]]aqua[[/aqua]]             |
|\[[emerald]]emerald[[/emerald]]        | <span class="color_emerald">emerald</span>          | [[emerald]]emerald[[/emerald]]       |
|\[[violet]]violet[[/violet]]          | <span class="color_violet">violet</span>           | [[violet]]violet[[/violet]]         |
|\[[pink]]pink[[/pink]]              | <span class="color_pink">pink</span>             | [[pink]]pink[[/pink]]             |
|\[[grassgreen]]grassgreen[[/grassgreen]]  | <span class="color_grassgreen">grassgreen</span>       | [[grassgreen]]grassgreen[[/grassgreen]] |
|\[[gold]]gold[[/gold]]              | <span class="color_gold">gold</span>             | [[gold]]gold [[/gold]]              |

### Sizes

It has 5 sizes: tiny, small, medium, big, and giant.

`[[tiny]] tiny [[/tiny]]` is rendered to `<span class="size_tiny"> tiny </span>`. The same rule is applied to the other sizes.

| MDxt                            | html                                             | output                       |
|---------------------------------|--------------------------------------------------|------------------------------|
|\[[tiny]]tiny[[/tiny]]           | <span class="size_tiny">tiny</span>              | [[tiny]]tiny[[/tiny]]        |
|\[[small]]small[[/small]]        | <span class="size_small">small</span>            | [[small]]small[[/small]]     |
|\[[medium]]medium[[/medium]]     | <span class="size_medium">medium</span>          | [[medium]]medium[[/medium]]  |
|\[[big]]big[[/big]]           | <span class="size_big">big</span>              | [[big]]big[[/big]]    |
|\[[giant]]giant[[/giant]]           | <span class="size_giant">giant</span>              | [[giant]]giant[[/giant]]    |

### Alignments

It has 3 alignments: left, right, and center.

`[[center]] center [[/center]]` is rendered to `<span class="align_center"> center </span>`. The same rule is applied to the other alignments.

Inline alignments are rendered to `span` tags and multi-lines are rendered to `div`. In most cases, `span` tags don't work with alignments.

### Highlights

`[[highlight = red]] This text is highlighted! [[/highlight]]` is rendered to `<span class="highlight_red"> This text is highlighted! </span>`. The same rule is applied to the other colors.

To see available colors, read [Colors](#colors) section.

### Box

`[[box]]A text in a box.[[/box]]` is rendered to `<div class="box">A text in a box.</div>`.

`[[box, no border]]A text in a box.[[/box]]` is rendered to `<div class="box no-border">A text in a box.</div>`.

```
[[box]]

A paragraph in a box.

[[box]]

A paragraph in a box in a box.

[[/box]]

[[box, no border]]

A paragraph in a borderless box in a box.

[[/box]]

[[/box]]
```

[[box]]

A paragraph in a box.

[[box]]

A paragraph in a box in a box.

[[/box]]

[[box, no border]]

A paragraph in a borderless box in a box.

[[/box]]

[[/box]]

### Table of Contents

```
| Table of Contents |
|-------------------|
|!![[collapsible]]  |
| [[toc]]           |
```

See how that's rendered [here](#tocsample)

### Special Characters

`[[char = 44032]]` is rendered to `&#44032;`, which is [[char = 44032]].

`[[char = copy]]` is rendered to `&copy;`, which is [[char = copy]].

To see the complete list of available characters, visit [here](MDxt-Character-Reference.html).

### Icons

WIP

### Math

`[[math]] sqrt{a sup{2} + b sup{2}} [[/math]]` is rendered to [[math]] sqrt {a sup{2} + b sup{2}} [[/math]]. It uses the [mathjax] library to render math formula.

[mathjax]: https://www.mathjax.org/

To see available math elements, visit [here](MDxt-Math-Reference.html).

### Multiline Macro

If a paragraph has a macro and no other contents at all, the paragraph is rendered to a multiline macro.

```
[[red]]

These 3 paragraphs are

rendered to

red texts.

[[/red]]
```

As you see above, the first and the last paragraph only consist of a macro. The macro will be rendered to `<div class="color_red">`.

[multiline macro]: #multiline-macro

## Plugins

WIP

## Characters

### Escapes

Backslashes (`\`) inside code spans and fenced code blocks are not escaped. All the other backslash characters are escape characters.

### Tabs

All the tab characters (`\t`) are converted to 4 whitespaces. All the newline characters except `\n` are ignored.