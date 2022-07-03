```markdown
This is a text.[^f1]

[^f1]: This is a footnote. **Inline** ~elements~ in a footnote are rendered.

```

This is a text.[^a]

[^a]: This is a footnote. **Inline** ~~elements~~ in a footnote are rendered.

Footnote label (`f1` in the example) is not rendered. That's just for writer's convenience. A label begins with `^` and may not contain any special characters. A label is case-insensitive, and all the whitespaces are ignored.