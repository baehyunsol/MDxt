## lines

- paragraph, empty, code_fence, table, list, blockquote
- header, macro, thematic_break
- link_ref, footnote_ref

## nodes

- paragraph
  - if paragraph
    - add curr_line to curr_nodes
- code_fence
  - if code_fence_end
    - add curr_node to curr_nodes
    - goto empty
  - else
    - add curr_line to curr_node


- paragraph, code_fence, table, list, blockquote
- header, macro, empty

## gfm

### Thematic breaks vs Settexts

`-`, `*`, `_` vs `-`, `=`

### Paragraph Interruption

- Header, List, Blockquote, Table, CodeFence
  - P 바로 다음 줄 ㄱㄴ
- link_ref, footnote_ref
  - P 바로 다음 줄 X
- thematic break, settext
  - 아오

## Macro

단독 macro는 invalid하면 paragraph랑 동일하게 취급