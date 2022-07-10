indent는 3 이하,
`>` 사이에 space 3개 이하로 가능
`>` depth는 무한히 가능!

indent가 3 이하이고, 첫 char가 `>`이면 무조건 blockquote
- 너무 loose해보이지만 gfm이 그렇게 하니까 나도 그렇게 하자!
- 심지어 `>` 다음에 space 없이 다른 문자 와도 blockquote로 침

현재 indent가 4인데 다음 indent가 4 이하면, 계속 4로 유지. 끊고 새로 가지는 않음..!

paragraph/blockquote만 붙이고 나머지는 전부 분리하자! empty 나와도 분리
