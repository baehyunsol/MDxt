pub mod predicate;

#[cfg(test)]
mod testbench;

use crate::ast::line::Line;
use crate::ast::parse::ParseState;
use crate::utils::{take_and_drop_while, remove_whitespaces, into_v16, lowercase, to_int};
use predicate::is_line_num;

/*
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
*/

// it assumes that the given line is a valid code fence
pub fn read_code_fence_info(line: &Line) -> ParseState {
    let (fence, mut info_string) = take_and_drop_while(&line.content, line.content[0]);
    info_string = remove_whitespaces(&info_string).iter().map(lowercase).collect();

    let mut language = into_v16("");
    let mut line_num = None;

    let arguments = info_string.split(|c| *c == ',' as u16).collect::<Vec<&[u16]>>();

    for argument in arguments.iter() {

        if is_line_num(argument) {

            if argument.len() == 8 {
                line_num = Some(1);
            }

            else {
                line_num = Some(to_int(&argument[9..argument.len() - 1]).unwrap() as usize);
            }

        }

        else {
            language = argument.to_vec();
        }

    }

    ParseState::CodeFence {
        language,
        line_num,
        code_fence_size: fence.len(),
        is_tilde_fence: line.content[0] == '~' as u16
    }
}