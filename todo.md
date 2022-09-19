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

pretty output
- output으로 나오는 html 좀 예쁘게 indent도 하고 그러자
- 이거 만드는 과정에서 html-validator도 나올 듯...

---

list 다음에 줄바꿈 없이 multiline macro가 오면 macro가 list 안으로 들어가버림. 아마 blockquote도 동일할 듯. table도 실험해보셈. 저게 맞는 동작인지 아닌지는 고민을 좀 해보자

---

난독화

class들 난독화하는 기능 추가, 난독화하는 함수 자체도 넣어서 배포!

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

`[[math]]`에 `calc{1 + sqrt{2}}` 이런 식으로 할 수 있을까??

계산 가능한 친구들
- `sqrt{}`, `root{}`
- `bincoeff{}{}`
- `cfrac{}{}`, `frac{}{}`
- `sum{}{}`
  - 아 이거 좀 어렵다. 어디까지가 시그마에 걸려있는 expr인지 파악하기가 무지무지하게 어렵네

덧셈뺄셈 범위 잡는 것만 해도 개빡셀 듯

---

fenced code block 실행가능하게 하고 싶음... 쥬피터 노트북처럼.
- 만약에 MDxt를 다른 인터넷 커뮤니티에서도 쓰이게 하고 싶으면 여기가 보안 구멍이 될 확률이 매우 높음.
- Python이나 lua 정도는 그럭저럭 넣을 수 있을텐데 다른 언어들은 빡센 거 많을텐데? 당장 Rust만 해도 구현 개빡셈.
- 실행 오래 걸리면 어떻게 할 거임? 끝날 때까지 기다릴 거임?
  - halting problem!

---

footnote에 `<sup>` 붙이지 말고 `<span class="footnote-ref">`를 붙이자! 이게 더 유연하지

---

html_to_mdxt도 만들 수 있지 않음? `<b>x</b>`는 `**x**`로 바꾸는 방식으로!

`<em>a</em><em>b</em>`을 `*a**b*`로 바꿔버리면 어떻게 해?? 저러면 아예 다른 syntax인데?