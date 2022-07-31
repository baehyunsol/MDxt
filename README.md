# MDxt

MarkdDown eXTended

MDxt is an extended markdown format. Though not a strict superset of [gfm], MDxt supports most of its features. You can read more about it [here](#gfm-compatibility)

Some elements require proper CSS files to be rendered. You can either write your own, or use CSS files in this repo.

If you wanna know more about MDxt, see these articles.

- [Reference]
- [API Doc]
  - Not yet

## GFM compatibility

[GFM] is a Github-flavored markdown format, which is one of the most widely used markdown extension.

MDxt can read most gfm documents, but the output is different. (ex: unlike gfm, code spans are rendered to `<code class="short">`.) It doesn't support some of gfm's syntaxes. (ex: indented code blocks, setext headings, and a few more)

I won't make any compatibility layers for gfm (ex: GFM <-> MDxt converter). Because some elements are impossible to convert to GFM formats. They require `<script>` tags in their output HTML, which are not supported in GFM and most other markdown extensions.

[GFM]: https://github.github.com/gfm
[Reference]: https://blog.baehyunsol.repl.co/MDxt-Reference.html
[my blog]: https://blog.baehyunsol.repl.co/

## Contributing

### Why `Vec<u16>`

Reading the source code, you'll see tons of `Vec<u16>`. Unlike most other crates, the engine doesn't use `Vec<u8>` for manipulating strings. That's because CJK characters don't fit in `u8` characters. To handle them easily, `Vec<u16>` is the only choice.

