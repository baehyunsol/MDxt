## lines

- paragraph, empty, code_fence, table, list, blockquote
- header, macro, thematic_break
- link_ref, footnote_ref

## nodes

- paragraph
  - if paragraph
    - add curr_line to curr_node
  - if empty
    - add curr_node to curr_nodes
    - goto empty
  - if header, list, blockquote, codefence
    - add curr_node to curr_nodes
    - goto another state
  - if table
    - if next is delimiter
      - add curr_node to curr_nodes
      - goto table
    - else
      - add curr_line to curr_node
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

## Table

table_header 오고, delimiter 와야 table로 취급

possible_table_header 만나면, 그 다음에 delimiter 오는지 미리 검사해버리자! 아니면 그냥 table_header도 paragraph 취급하셈

leading and trailing pipe는 항상 필수!
- `|`를 pipe라고 부르는구만