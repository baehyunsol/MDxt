
`[[macro_content]]`

Macro content is case insensitive, and all the whitespaces and underbars are ignored.

> Valid characters inside macros [[br]]
> `a-z`, `A-Z`, `0-9`, `=`, `,`, ` `, `_`, `/`

## Color

- `[[red]] ... [[/red]]`
- `[[green]] ... [[/green]]`
- `[[blue]] ... [[/blue]]`

## Size

- `[[tiny]] ... [[/tiny]]`
- `[[small]] ... [[/small]]`
- `[[medium]] ... [[/medium]]`
  - default
- `[[big]] ... [[/big]]`
- `[[giant]] ... [[/giant]]`

It changes not only font sizes, but also image sizes.

## Highlight

- `[[highlight = red]] ... [[/highlight]]`

## Alignment

- `[[center]] ... [[/center]]`
- `[[left]] ... [[/left]]`
- `[[right]] ... [[/right]]`

## Box

`[[box]] ... [[/box]]`

## Toc

`[[toc]]`

## Characters

- `[[blank]]`
  - `&nbsp;`
- `[[br]]`
  - `<br/>`
- `[[char = 44032]]`
  - `&#44032;`

## Icons

`[[icon = github, size = 32]]`

## Math

`[[math]] cfrac{-b pm sqrt{b sup{2} - 4 a c}}{2 a} [[/math]]`

Math macros are special. Every inline element inside a `[[math]]` block is not rendered. Not even code spans!

## HTML

`[[div, class = foo, id = bar]] baz [[/div]]` is rendered to `<div class="foo" id = "bar"> baz </div>`.

All the whitespaces are ignored inside macros. That means `[[div, class = foo bar]] [[/div]]` is rendered to `<div class="foobar"> </div>`. For multiple classes, the correct syntax would be `[[div, class = foo, class = bar]] [[/div]]`.

It supports 4 types of tags and 2 attributes.

- `[[div]] [[/div]]`
- `[[span]] [[/span]]`
- `[[button]] [[/button]]`
- `[[anchor]] [[/anchor]]`
  - `<a>` tag

Make your own plugins with those macros, CSS and JS.

## Incubator

### Line Height
