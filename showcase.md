# MDex Showcase

## Tables

Tables with multiline-headers, colspans and column alingments!

```
|           |   [[colspan=2]] Compiled   |   Interpreted   |
|           |     Rust     |     Zig     |     Python      |
|-----------|--------------|-------------|-----------------|
| Repo      | [Rust]       | [Zig]       | [Python]        |
| Stars     | 68.5k        | 14.9k       | 46.1k           |
```

|           |   [[colspan=2]] Compiled   |   Interpreted   |
|           |     Rust     |     Zig     |     Python      |
|-----------|--------------|-------------|-----------------|
| Repo      | [Rust]       | [Zig]       | [Python]        |
| Stars     | 68.5k        | 14.9k       | 46.1k           |

[Zig]: https://github.com/ziglang/zig
[Rust]: https://github.com/rust-lang/Rust
[Python]: https://github.com/python/cpython

## Footnotes

```
This is a footnote.[^A]

[^A]: Hi, there!
```

This is a footnote.[^A]

[^A]: Hi, there!

## Fenced Code blocks

````
```rust, line_num, highlight(6, 17, 22)
/*
    multiline
    comment
*/
// single line comment
fn main() {
    let mut x = 3;
    let mut y = if x == 3 {
        4
    } else {
        5
    };
    println!("Hello World!\n");
    println!("{:?}", 3 + 4);
}

pub struct Point {
    x: f32,
    y: f32
}

pub const CONST: u32 = 1;
```
````

```rust, line_num, highlight(6, 17, 22)
/*
    multiline
    comment
*/
// single line comment
fn main() {
    let mut x = 3;
    let mut y = if x == 3 {
        4
    } else {
        5
    };
    println!("Hello World!\n");
    println!("{:?}", 3 + 4);
}

pub struct Point {
    x: f32,
    y: f32
}

pub const CONST: u32 = 1;
```

## Inline elements

`CO~2~` is rendered to CO~2~. [[br]]
`E = mc^2^` is rendered to E = mc^2^. [[br]]
`~~del~~` is rendered to ~~del~~. [[br]]
`~_underline_~` is rendered to ~_underline_~.

## Macros

`[[big]][[red]]Big red text.[[/red]][[/big]]` is rendered to [[big]][[red]]Big red text.[[/red]][[/big]].

`[[math]]cfrac{-b pm sqrt{b sup{2} - 4 a c}}{2a}[[/math]]` is rendered to [[math]]cfrac{-b pm sqrt{b sup{2} - 4 a c}}{2a}[[/math]].