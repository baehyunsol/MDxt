# MDxt Reference

MDxt is an extended version of Markdown.

| Table of Contents |
|-------------------|
|!![[collapsible]]   |
| [[toc]]           |

## Inline Elements

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

### Footnotes

## Containers

### Headers

***Inline macros don't work in headers!***

### Tables

### Lists

#### Task list

### Fenced Code Blocks

### Blockquotes

> This is a blockquote.
>> This is another blockquote.

### Unlike GFM...

MDxt doesn't support setext headers and indented code blocks.

## Macros

Macros are inline elements. Which means an opening macro and the closing one has to be in the same paragraph. But there are many cases where you want to apply macros to multiple paragraphs. Read [multiline macro] section for that.

### Colors

It has 14 colors.

`[[red]] abc [[/red]]` is rendered to `<span class="color_red"> abc </span>`. The same rule is applied to the other colors.

### Sizes

### Alignments

### Highlights

### Box

### Table of Contents

### Special Characters

### Icons

WIP

### Math

### Multiline Macro

[multiline macro]: #multiline-macro

## Plugins

WIP
