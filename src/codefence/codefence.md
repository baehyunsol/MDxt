[ref](https://www.getzola.org/documentation/content/syntax-highlighting/#styling-codeblocks)

with `line_num` on, 

`<pre><code><table></table></code></pre>`

no `<thead>`, two `<td>` per `<tr>`. first for the line_num, and the other for the content

`highlight` is easily implemented with coloring `<tr>`s.

````
```rust, line_num, highlight(2, 3)
fn main() {
    println!("Hello World!");
}
```

```rust, line_num(5)
fn main() {
    println!("Hello World!");
}
```

``` rust
fn main() {
    println!("Hello World!");
}
```

```line_num
fn main() {
    println!("Hello World!");
}
```
````