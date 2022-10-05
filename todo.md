## TODO

Plugin system

macro 처리하는 함수가, 일단 지 거 해보고 안되는 거 있으면 사용자가 넘겨준 함수한테 매크로 처리 시키는 거지! `arguments: Vec<Vec<Vec<u16>>>, content: Vec<u16> -> Option<Vec<u16>>`의 함수를 사용자가 만들면 그게 곧 plugin 아님?

`Plugin`이라는 struct를 만들고 render_option에 넣어서 engine한테 넘기자. `.json`이나 `.yaml`, `.toml`등으로 `Plugin` 정의하고 그거 serde하는 함수도 만들자!

예시: `[[foo]] bar [[/foo]]` -> `[[div, class=foo]] bar [[/div]]`
- `div`랑 `class` 조합하는 거는 설정파일로 정의하기 쉬울 거 같은데 다른 것들은?
  - `id`가 조금씩 변하게 하려면?
  - 안의 내용을 다룰 수 있게 하려면?

아니면 아예 plugin 다루는 스크립트 언어를 추가할까?? Rust에 embed할 수 있는 script가 뭐 있지...

---

tooltip

얘도 `[[span]]`으로 어찌저찌 하거나 걍 plugin으로 편입시켜버리면 될 듯!

---

list 다음에 줄바꿈 없이 multiline macro가 오면 macro가 list 안으로 들어가버림. 아마 blockquote도 동일할 듯. table도 실험해보셈. 저게 맞는 동작인지 아닌지는 고민을 좀 해보자

---

TOC

```markdown
- !![[no bullet]]
- 1. h1
  - 1.1. h2
  - 1.2. h2
    - 1.2.1. h3
  - 1.3. h2
- 2. h1
  - 2.1. h2
```

저런 식으로 하고 index에다가만 링크를 걸어야 header에 inline decoration 넣기 수월!

각 header마다 `[[span]]`으로 감싸면 좀 더 낫지 않을까?
1. 만약에 header 안에다가 `[[span]]` 없이 `[[/span]]`만 넣는 싸이코가 있으면?
  - `[]`는 전부 escape해서 ㄱㅊ
1. 

---

html_to_mdxt도 만들 수 있지 않음? `<b>x</b>`는 `**x**`로 바꾸는 방식으로!

`<em>a</em><em>b</em>`을 `*a**b*`로 바꿔버리면 어떻게 해?? 저러면 아예 다른 syntax인데? 똑같은 decoration끼리 붙어있으면 합칠까? 그건 구현에서 edge case가 무지 많을텐데...

---

from gitlab

1. diagram
  - fenced code block 안에 diagram을 그려줌. language 설정을 mermaid, plantuml, kroki 등으로 해주면 됨.
  - 내가 직접 diagram language 구현하는 건 빡센데 쟤넨 전부 js로 돼 있어서 내 엔진에 넣기 애매함
    - 직접 만들자!
  - 굳이 diagram이 아니더라도 fenced code block에 특별한 language 주면 특별한 동작하도록 하는 건 좋은 아이디어인 듯!
    - 얘는 math도 fenced code block에 `math`라는 language를 줘서 구현함
  - language를 json으로 넣어서 table로 만드는 것도 있네..!!
1. emoji
  - 얘는 `:monkey:` 이런 식으로 emoji를 넣음
1. front matter
  - metadata block을 front matter라고 하네.
  - metadata를 어느 언어(json, yaml, toml등)로 쓰냐에 따라서 fence의 모양이 다름
1. task list
  - `[~]`도 있음.
  - 내 추가 아이디어: `[!]`도 넣을까?
1. multiline blockquote
  - fenced code block이랑 비슷한 blockquote
  - fence로 `>>>`를 씀.
  - 이러면 blockquote 안에 표, 리스트 등등을 넣을 수 있음!
  - list 안에다가 표/fencedcode 넣고 싶은데 이건 어떻게 할까??
1. underscore로 emphasis하는 것도 넣을까..??

---

`*a* **b**`하고 `*a***b**`가 다르게 render되는데 이건 intended로 치는게 낫겠지?

---

```
a
---
b
---
c
```

쟤네를 horizontal line으로 할지 paragraph로 할 지는 어떻게 결정함..?? GFM 따라할까? 근데 GFM은 쟤네를 header로 쓰는데?

---

math에 있는 기호들 웬만해선 char에도 넣자!

곱셈, rightarrow

음표도 추가 ㄱㄱ