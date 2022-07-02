[[giant]]***This document uses HMD's syntax. The syntax is slightly different to [gfm](https://github.github.com/gfm). If you're reading this document on github, please visit [my blog](https://somewhere) to see the properly rendered version.***[[/giant]]

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

AST 안에 render_option이라는 field를 만들자. 그리고 parse든 render든 인수로 `&AST`를 받게 하자. 왜냐면 parse나 render가 인수로 받아야할 게 계속 늘어날텐데, 그렇다고 함수의 parameter를 계속 늘리면 너무 코드가 더러워지잖아? 그냥 AST에 싹 다 집어넣고 `&AST`만 넘기자..!
- footnote랑 toc 정보도 parse 함수한테 넘겨야하는데, 그러려면 걍 AST를 통째로 넘기는게 낫지...ㅎㅎ
- 근데 inline node가 AST 안에 들어있는데 `&mut InlineNode`한테 `AST`를 넘기는게 되나?

파일들 `mod.rs`로 바꿔서 폴더 안에 넣자..!

만약에 누가 header에 `[[toc]]`를 넣으면 어떻게 되는 거임..??
- 걍 header 안에 macro는 못 넣게 하자!