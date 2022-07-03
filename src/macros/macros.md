
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

Math macros are special. Every inline element inside a `[[math]]` block is not rendered. Not even codespans!
