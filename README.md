# MDxt

MarkdDown eXTended

MDxt is an extended markdown format. Though not a strict superset of [gfm], MDxt supports most of its features. You can read more about it [here](#gfm-compatibility)

Many features of MDxt require proper style sheets and script files. I highly recommend you use MDxt with a dedicated [web framework](https://github.com/baehyunsol/engine).

If you wanna know more about MDxt, see these articles.

- [Reference]
- [API Doc]

## GFM compatibility

[GFM] is a Github-flavored markdown format, which is one of the most widely used markdown extension.

MDxt can read most gfm documents, but the output is different. (ex: unlike gfm, code spans are rendered to `<code class="short">`.) It doesn't support some of gfm's syntaxes. (ex: indented code blocks, setext headings, and a few more)

I won't make any compatibility layers for gfm (ex: GFM <-> MDxt converter). Because some elements are impossible to convert to GFM formats. They require `<script>` tags in their output HTML, which are not supported in GFM and most other markdown extensions.

[GFM]: https://github.github.com/gfm
[Reference]: https://baehyunsol.github.io/MDxt-Reference.html
[API Doc]: https://docs.rs/mdxt/latest/mdxt/index.html
[my blog]: https://baehyunsol.github.io/

## Frontend Framework

If you want a fully-featured frontend framework with MDxt, try [this project](https://github.com/baehyunsol/engine).

TODO: remove `./styles`

## Contributing

### Why `Vec<u32>`

It's the only way to index strings (`String` doesn't support indexing) and use emojis (most emojis are greater than `u16::MAX`)
