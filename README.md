***On github, some elements are not rendered properly. To read a properly rendered version, visit [my blog].***

# MDxt

[[Giant]] MarkdDown eXTended [[/Giant]]

MDxt is an extended markdown format. Though not a strict superset of [gfm], MDxt supports most of its features. You can read more about it [here](#gfm-compatibility)

Some elements require proper CSS files to be rendered. You can either write your own, or use CSS files in this repo.

If you wanna know more about MDxt, see these articles.

- [Showcase]
- [Reference]
- [API Doc]

## GFM compatibility

[GFM] is a Github-flavored markdown format, which is one of the most widely used markdown extension.

MDxt can read most gfm documents, but the output is different. (ex: unlike gfm, fenced code blocks are rendered to tables.) It doesn't support some of gfm's syntaxes. (ex: indented code blocks, setext headings, and a few more)

I won't make any compatibility layers for gfm (ex: GFM <-> MDxt converter). Because some elements are impossible to convert to GFM formats. They require `<script>` tags in their output HTML, which are not supported in GFM and most other markdown extensions.

[GFM]: https:github.github.com/gfm
[Showcase]: showcase.md
[Reference]: https://molla
[my blog]: https://molla
[API Doc]: https://molla

## Contributing

### Why `Vec<u16>`

Reading the source code, you'll see tons of `Vec<u16>`. Unlike most other crates, the engine doesn't use `Vec<u8>` for manipulating strings. That's because CJK characters don't fit in `u8` characters. To handle them easily, `Vec<u16>` is the only choice.

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

Plugin system

macro 처리하는 함수가, 일단 지 거 해보고 안되는 거 있으면 사용자가 넘겨준 함수한테 매크로 처리 시키는 거지! `arguments: Vec<Vec<Vec<u16>>>, content: Vec<u16> -> Option<Vec<u16>>`의 함수를 사용자가 만들면 그게 곧 plugin 아님?

---

tooltip

---

mdx engine이 mathjax 결과물까지 다 처리한 다음에 최종 결과물에는 svg만 넣으면 안되나?

---

prefix

html로 변환할 때 class랑 id 앞에다가 일괄적으로 prefix 붙이기

---

multiline macro

```rust
enum MultilineMacro {
    Color {
        color: Vec<u16>,
        content: Vec<Line>
    }
}
```

이런 식 ㅇㄸ? 그리고 저거 parse 할 때는 쟤한테 `doc_data`랑 `render_option` 다 줘버리고 AST를 뽑아낸 다음에 걔를 바깥 AST에 합치는 거임...!! 너무 거한가?

---

toc

`doc_data.has_toc`를 만들고, 저게 참일 때만 `AST.toc: Vec<Node>`를 생성하는 거임! `.to_html()`하면서 저 toc를 그대로 rendering해서 던져주는 거지!
- 그럼 toc가 여러개 있으면 to_html도 여러번 함? 그건 너무 비효율적이지 않나... 일단은 cache를 할까?

---

showcase를 정식 API에 편입시키자. showcase를 html로 렌더링한 다음에 공식 css 파일까지 붙여서 반환

---

fenced code block의 hover effect on/off 가능하게 하자!