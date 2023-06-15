---
date: [2023, 6, 15]
hide: true
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

The link syntaxes resemble that of [GFM]'s.

### Images

A valid link after a bang(!) character is rendered to an `img` tag.

`![abc](def)` is rendered to `<img src="def" alt="abc">`.

#### Multimedia types

The engine tries to figure out the type of the image. If the file extension is `mp4` or `webm`, it'd generate a `<video>` tag. If the extension is `mp3`, `ogg`, `m4a`, or `wav`, it'd be `<audio>` tag. Otherwise it's an `<img>` or a youtube video.

If the file destination meets the below conditions, the engine will embed a youtube video.

- It starts with 10 ~ 12 characters of `[0-9A-Za-z_-]`, which is a video id.
- A query for time may follow the video id. The query starts with `?t=`, and ends with an integer.

Only insert the video id, not the full url.

- Valid examples
  - `![an example video](bwxglJEpuDc)`
    - Only an id
    - ![an example video](bwxglJEpuDc)
  - `![another example video](x02eGI-O9WY?t=20)`
    - id + time
    - ![another example video](x02eGI-O9WY?t=20)
- Invalid examples
  - `![another example video](https://www.youtube.com/watch?v=QEnuzwCWpgQ)`
    - full url

### Footnotes

Links whose names begin with `^` are rendered to footnotes. A footnote may have multiple lines, but it can have only 1 paragraph. Don't ref another footnote inside a footnote.

Below is an example.

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

Table cells and table itself can have a macro. A table cell with a macro must start with the macro. For example, `|[[colspan=3]] valid cell|` is a valid table cell with a macro, but `|invalid macro [[colspan=3]]|` is a valid cell without a macro.

Macros applied table-wide come at the first row of a table. The row shall have only one cell. The cell contains only macros and whitespaces. Each table-wide macro must be prefixed by `!!`. See the examples below.

#### Column Alignments

Syntaxes of column alignments resemble that of [GFM]'s.

[GFM]: https://github.github.com/gfm/

#### Multiline Table Head

```
|         [[colspan = 6]] Shopping List         |
| [[colspan = 3]] Food  | [[colspan = 3]] Drink |
|-------|:-----:|-------|:-----:|-------|-------|
| Bread | Cake  | Pie   | Beer  | Water | Coffee|
| None  | Center| None  | Center| None  | None  |
| Foo   | [[colspan = 4]] *Bar*         |
```

|         [[colspan = 6]] Shopping List         |
| [[colspan = 3]] Food  | [[colspan = 3]] Drink |
|-------|:-----:|-------|:-----:|-------|-------|
| Bread | Cake  | Pie   | Beer  | Water | Coffee|
| None  | Center| None  | Center| None  | None  |
| Foo   | [[colspan = 4]] *Bar*         |

#### Colspan

The previous example contains colspan macros.

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

#### Headless

```
| This table head is not shown.      |
|------------------------------------|
|!![[headless]]                      |
| This is a headless table.          |
| This is another row of the table.  |
```

| This table head is not shown.      |
|------------------------------------|
|!![[headless]]                      |
| This is a headless table.          |
| This is another row of the table.  |

If both `headless` and `collapsible` are enabled, `headless` is ignored.

### Lists

#### Task list

```
- [ ] Unchecked
- [X] Checked
- [^] Triangle
```

- [ ] Unchecked
- [X] Checked
- [^] Triangle

#### List Macros

```
- !![[no bullet]]
- no
- bullet
  - 123
  - 456
- 789
  a. !![[start = 20]]
  a. `[[start = t]]` is invalid.
  a. hahaha
```

is rendered to 

```html
<ul class="no-bullet-list">
    <li>no</li>
    <li>bullet
        <ul>
            <li>123</li>
            <li>456</li>
        </ul>
    </li>
    <li>789
        <ol type="a" start="20">
            <li><code class="short">[[start = t]]</code> is invalid.</li>
            <li>hahaha</li>
        </ol>
    </li>
</ul>
```

which looks like

- !![[no bullet]]
- no
- bullet
  - 123
  - 456
- 789
  a. !![[start = 20]]
  a. `[[start = t]]` is invalid.
  a. hahaha

### Fenced Code Blocks

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
    let z = x < 3 && y > 4;
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
    let z = x < 3 && y > 4;
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

### Metadata

The engine can read metadata in your markdown files. Metadata section starts with `---` and ends with `---`. There can only be one or less metadata section in each file. A metadata section must be the very first part of the document, if exists.

The engine uses [yaml-rust](https://github.com/chyh1990/yaml-rust) crate to parse metadata. Metadata should be a valid yaml object. Since yaml is superset of json, you can also use json objects as metadata.

Below is an example of a metadata section.

```
---
date: [2022, 8, 7]
author: Baehyunsol
---

# Header

Paragraph
```

### Unlike GFM...

MDxt doesn't support setext headers and indented code blocks.

## Macros

Macros are inline elements. Which means an opening macro and the closing one has to be in the same paragraph. But there are many cases where you want to apply macros to multiple paragraphs. Read [multiline macro] section for that.

A valid macro consists of `A-Z`, `a-z`, `0-9`, `=`, `,`, `_`, and ` `. If a double square bracket contains invalid characters, that won't be parsed as a macro. Whitespaces and `_`s inside macros are ignored, and all the alphabet characters are lowered. That means `[[box, no_border]]` and `[[box, n o border]]` are exactly the same macro.

### Colors

It has 18 colors: black, dark, gray, lightgray, white, red, green, blue, brown, slateblue, seagreen, aqua, emerald, violet, turquoise, pink, grassgreen, and gold.

| MDxt                                 | html                                              | output                               |
|--------------------------------------|---------------------------------------------------|--------------------------------------|
|\[[black]]black[[/black]]             | <span class="color-black">black</span>            | [[black]]black[[/black]]             |
|\[[dark]]dark[[/dark]]                | <span class="color-dark">dark</span>              | [[dark]]dark[[/dark]]                |
|\[[gray]]gray[[/gray]]                | <span class="color-gray">gray</span>              | [[gray]]gray[[/gray]]                |
|\[[lightgray]]lightgray[[/lightgray]] | <span class="color-lightgray">lightgray</span>    | [[lightgray]]lightgray[[/lightgray]] |
|\[[white]]white[[/white]]             | <span class="color-white">white</span>            | [[white]]white[[/white]]             |
|\[[red]]red[[/red]]                   | <span class="color-red">red</span>                | [[red]]red[[/red]]                   |
|\[[green]]green[[/green]]             | <span class="color-green">green</span>            | [[green]]green[[/green]]             |
|\[[blue]]blue[[/blue]]                | <span class="color-blue">blue</span>              | [[blue]]blue[[/blue]]                |
|\[[brown]]brown[[/brown]]             | <span class="color-brown">brown</span>            | [[brown]]brown[[/brown]]             |
|\[[slateblue]]slateblue[[/slateblue]] | <span class="color-slateblue">slateblue</span>    | [[slateblue]]slateblue[[/slateblue]] |
|\[[seagreen]]seagreen[[/seagreen]]    | <span class="color-seagreen">seagreen</span>      | [[seagreen]]seagreen[[/seagreen]]    |
|\[[aqua]]aqua[[/aqua]]                | <span class="color-aqua">aqua</span>              | [[aqua]]aqua[[/aqua]]                |
|\[[emerald]]emerald[[/emerald]]       | <span class="color-emerald">emerald</span>        | [[emerald]]emerald[[/emerald]]       |
|\[[violet]]violet[[/violet]]          | <span class="color-violet">violet</span>          | [[violet]]violet[[/violet]]          |
|\[[turquoise]]turquoise[[/turquoise]] | <span class="color-turquoise">turquoise</span>    | [[turquoise]]turquoise[[/turquoise]] |
|\[[pink]]pink[[/pink]]                | <span class="color-pink">pink</span>              | [[pink]]pink[[/pink]]                |
|\[[grassgreen]]grassgreen[[/grassgreen]]  | <span class="color-grassgreen">grassgreen</span>       | [[grassgreen]]grassgreen[[/grassgreen]] |
|\[[gold]]gold[[/gold]]                | <span class="color-gold">gold</span>              | [[gold]]gold [[/gold]]               |

### Sizes

It has 5 sizes: tiny, small, medium, big, and giant.

`[[tiny]] tiny [[/tiny]]` is rendered to `<span class="size-tiny"> tiny </span>`. The same rule is applied to the other sizes.

| MDxt                          | html                                       | output                       |
|-------------------------------|--------------------------------------------|------------------------------|
|\[[tiny]]tiny[[/tiny]]         | <span class="size-tiny">tiny</span>        | [[tiny]]tiny[[/tiny]]        |
|\[[small]]small[[/small]]      | <span class="size-small">small</span>      | [[small]]small[[/small]]     |
|\[[medium]]medium[[/medium]]   | <span class="size-medium">medium</span>    | [[medium]]medium[[/medium]]  |
|\[[big]]big[[/big]]            | <span class="size-big">big</span>          | [[big]]big[[/big]]           |
|\[[giant]]giant[[/giant]]      | <span class="size-giant">giant</span>      | [[giant]]giant[[/giant]]     |

### Line Heights

It has 5 heights: tiny, small, medium, big and giant.

```
[[line height = tiny]]

Tiny lines\
Tiny lines\
Tiny lines

[[/line height]]
```

[[line height = tiny]]

Tiny lines\
Tiny lines\
Tiny lines

[[/line height]]

### Alignments

It has 3 alignments: left, right, and center.

`[[center]] center [[/center]]` is rendered to `<span class="align-center"> center </span>`. The same rule is applied to the other alignments.

Inline alignments are rendered to `span` tags and multi-lines are rendered to `div`. In most cases, `span` tags don't work with alignments. That means,

```
[[center]] This text is not centered.[[/center]]

[[center]]

This text is centered.

[[/center]]
```

### Highlights

`[[highlight = red]] This text is highlighted! [[/highlight]]` is rendered to `<span class="highlight-red"> This text is highlighted! </span>`. The same rule is applied to the other colors.

[[highlight = red]] This text is highlighted! [[/highlight]]

To see available colors, read the [Colors](#colors) section.

### Box

`[[box]]A text in a box.[[/box]]` is rendered to `<span class="box">A text in a box.</span>`.

`[[box, no border]]A text in a box.[[/box]]` is rendered to `<span class="box no-border">A text in a box.</span>`.

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

You can set box's width/height using attributes. See examples below.

```
[[box, width = giant, height = giant]]

A Giant Box

[[/box]]

[[box, width = tiny, height = tiny]]

A Tiny Box

[[/box]]
```

[[box, width = giant, height = giant]]

A Giant Box

[[/box]]

[[box, width = tiny, height = tiny]]

A Tiny Box

[[/box]]

There's another attribute: `inline`, which is a bit tricky. An inline `[[box]]` macro is rendered to a `<span>` tag, while a multiline `[[box]]` macro with an `inline` attribute is rendered to a `<div>` tag with an `"inline"` class.

### Tooltips

`[[tooltip]]` macro generates a tooltip. See the example below.

```
[[tooltip=abc]] Hover over me! [[/tooltip]]

[^abc]: This is a tooltip message.
```

[[tooltip=abc]] Hover over me! [[/tooltip]]

[^abc]: This is a tooltip message.

A tooltip message's syntax is the same as footnote cite's. There are some limitations, though. You cannot nest a tooltip inside another tooltip.

### Table of Contents

```
| Table of Contents |
|-------------------|
|!![[collapsible]]  |
| [[toc]]           |
```

See how the above code is rendered, see [here](#tocsample).

### Special Characters

`[[char = 44032]]` is rendered to `&#44032;`, which is [[char = 44032]].

`[[char = copy]]` is rendered to `&copy;`, which is [[char = copy]].

To see the list of available characters, visit [here](MDxt-Character-Reference.html).

`[[br]]` is rendered to `<br/>` and `[[blank]]` is rendered to `&nbsp;` If you want multiple blanks, `[[blank=3]]` and `[[br=4]]` are your options.

### Icons

[[icon = star]][[icon = tree]][[icon = terminal]]

You can embed SVG icons using the `[[icon]]` macro. The full documentation can be found [here](MDxt-Icon-Reference.html).

### Math

`[[math]] sqrt{sup{a}{2} + sup{b}{2}} [[/math]]` is rendered to [[math]] sqrt{sup{a}{2} + sup{b}{2}} [[/math]]. It renders the output in [MathML].

[MathML]: https://developer.mozilla.org/en-US/docs/Web/MathML

To see the list of available math elements, visit [here](MDxt-Math-Reference.html).

### Sidebar

```
[[sidebar]]

[[toc]]

[[/sidebar]]
```

[[sidebar]]

[[toc]]

[[/sidebar]]

`[[sidebar]]` macro generates a sidebar. It only works as a [multiline-macro](#multiline-macro). If multiple sidebars are declared, it only accepts the last one. In order for it to work properly, it requires a css and js. Check the css files in its [repo](https://github.com/baehyunsol/MDxt).

### Multiline Macro

If a paragraph has a macro and no other contents at all, the paragraph is rendered to a multiline macro.

```
[[red]]

These 3 paragraphs are

rendered to

red texts.

[[/red]]
```

As you see above, the first and the last paragraph only consist of a macro. The macro will be rendered to `<div class="color-red">`.

If an opening macro is declared as a multiline-macro, the closing one must also be multiline.

[multiline macro]: #multiline-macro

## Plugins

WIP

## Characters

### Escapes

Backslashes (`\`) inside code spans and fenced code blocks are not escaped. All the other backslash characters are escape characters.

### Tabs

All the tab characters (`\t`) are converted to 4 whitespaces. All the newline characters except `\n` are ignored.