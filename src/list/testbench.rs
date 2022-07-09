use crate::utils::{into_v16, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn list_samples() -> Vec<(String, String)> {
    let result = vec![
        ("
- 1
- 1
- 1
- 1
- 1
", "
"), ("
- 1
 - 1
  - 1
   - 1
    - 1
", "
"), ("
    - 1
   - 1
  - 1
 - 1
- 1
", "
"), ("
- 1
  - 2
    - 3
      - 4
   - 2
     - 3
     - 4
", "
"), ("
- 1
  - 2
      - 3
      - 3
      - 3
    - 3
    - 3
      - 4
      - 4
", "
"), ("
- 1
  - 2
      - 3
      - 3
      - 3
   - 2
   - 2
     - 3
     - 3
", "
"), ("
        - 1
      - 1
    - 1
  - 1
- 1
", "
"), ("
- 1
1
  1
  - 2
2
  2
    - 3
3
  3

    - 1
  - 1
- 1
 - 
  - 2
", "
"), ("
1. 1
1. 1
1. 1
", "
"), ("
2. 2
2. 2
2. 2
", "
"), ("
a. a
a. a
a. a
", "
"), ("
b. b
b. b
b. b
", "
"), ("
A. A
A. A
A. A
", "
"), ("
B. B
B. B
B. B
", "
"), ("
i. i
i. i
i. i
", "
"), ("
I. I
I. I
I. I
", "
"), ("
- nums
  1. 1
  2. 2
  3. 3
- alphabets
  a. a
  a. b
  a. c
- romans
  i. i
  i. ii
  i. iii
- ul
  - 1
  - 2
  - 3
", "
"), ("
- 1
---
- 1
# 1
- 1
```
nothing
```
", "
")
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn list_test() {
    for (md, html) in list_samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v16(&rendered)) != remove_whitespaces(&into_v16(html)) {
            panic!("{} \n\n {}", md, rendered);
        }

    }

}