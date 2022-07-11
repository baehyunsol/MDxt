# MDex

MDex is an extended markdown format. Though not a strict superset of [gfm], MDex supports most of its features. You can read more about it [here](#gfm-compatibility)

Some elements require proper CSS files to be rendered. You can either write your own, or use CSS files in this repo.

If you wanna know more about MDex, see these articles.

- [showcase]
- [reference]

## GFM compatibility

[GFM] is a Github-flavored markdown format, which is one of the most widely used markdown extension.

MDx can read most gfm documents, but the output is different. (ex: unlike gfm, fenced code blocks are rendered to tables.) It doesn't support some of gfm's syntaxes. (ex: indented code blocks, setext headings, and a few more)

I'm planning to add more compatibility layers for gfm, but they're all WIP.

- [ ] GFM <-> MDex converter.
- [ ] Render MDex to GFM's output format.

[GFM]: https:github.github.com/gfm
[showcase]: showcase.md

---

# OLD

## Inline elements

It supports most inline decorations that [gfm](https://github.github.com/gfm) supports, and has a few more.

### Underlines

`~_Underlines_~` is rendered to ~_Underlines_~. The first and the last character may not be a whitespace.

## Macros

HMD doesn't let you use raw html tags. Instead, it has plenty of macros.

Some macros have opening and closing elements, just like HTML tags! Those macros are translated directly to HTML tags. For them to be translated correctly, they should not mess up with `<p>` tags. So an opening macro and closing one should be in the same paragraph, or take up a single paragraph. See examples below.

```
[[red]]

This text is red!

[[/red]]

[[red]] This text is red! [[/red]]
```

> Properly closed.

```
[[red]] This text is not red!

Because the closing macro is in another paragraph. [[/red]]
```

> Not proper.

[[box]]

All the double square brackets are rendered to macros. If you want a square bracket inside a link text, use escape characters. `[\\[link text\\]]` like this.

[[/box]]

### Colors

The actual colors may differ depending on css files you use.

| Name |                      Showcase                      |
|:----:|:--------------------------------------------------:|
| Red  | [[red]] This text is red! [[/red]]                 |
| Blue | [[blue]] This text is blue! [[/blue]]              |

## Backslash Escapes

Unlike gfm, backslash escapes work everywhere, even inside code blocks!

## TODO

normalize_어쩌구저쩌구, link_handler 등에 특수문자를 넣어서 저 함수를 여러번 거치는 친구들이 있는지 검사

ex) normalize 함수를 거치면 결과물에 0x8000이 포함되게 하고, normalize 함수에 들어온 input에 0x8000이 포함돼 있으면 에러 던지기

---

tooltip, collapsible, button

CRLF
- chr(11), chr(12), chr(13)

---

mdx engine이 mathjax 결과물까지 다 처리한 다음에 최종 결과물에는 svg만 넣으면 안되나?

---

Header anchor

알파벳 대소문자, 숫자, 한글, `-`, `_`은 살리고 ` `는 `-`로 바꿈
