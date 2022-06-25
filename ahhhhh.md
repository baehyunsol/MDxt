`[[math]] sum{n=-infty}{infty} [[/math]]` inside a codespan must be ignored. `` inside a math formula must be ignored.

- render_prev ++ `raw` ++ render_next
  - it was the original way, but I abandoned it for a reason that I don't remember...
  - if codespans are rendered first, it would render codespans inside math blocks
  - vice versa...
- goes through multiple passes
  - stuffs inside a codespan will be rendered...

`***bold and italic***`

`~_~~del_and_underline~~_~`


TODO
- pass all the tests in `inline.rs`
- see how gfm works
  - with backslash escapes