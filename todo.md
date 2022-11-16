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

`[[details, summary = click me to open!]]` 이렇게 하고 싶긴 한데...

- 대소문자 구분 X, 띄어쓰기 무시
  - 저 제한 그냥 없애버릴까?
  - 대소문자 달라도 같은 identifier이지만 대소문자 정보를 날리진 말자!
- 문장부호 아무것도 못 씀
  - 이거는 좀 다른 얘기임. macro 읽는 함수들을 싹다 뜯어 고쳐야하는데?

footnote 전부 tooltip으로 띄우고 싶음!

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

저런 식으로 하고 index에다가만 링크를 걸어야 header에 inline decoration 넣기 수월함.

여러 방식의 link/index 넣을 생각하지 말고 저 방식으로 통일하자! inline::macros::toc.rs만 고치면 됨.

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

math에 있는 기호들 웬만해선 char에도 넣자!

곱셈, rightarrow

음표도 추가 ㄱㄱ

---

underscore로 emphasis하는 문법도 넣을까? 그냥 testcode 무지무지 많이 만든 다음에 gfm이랑 동일하게 동작하도록 구현하면 될 듯?

---

blockquote에서 `\`로 줄바꿈하는 거 왜 안됨..??

`\`로 줄바꿈하는 거랑 `  `로 줄바꿈 하는 거 대대적으로 손보자! 지금 너무 지저분하게 구현돼있음...

---

아래의 md를 gfm이랑 mdxt에서 둘다 해보셈. 첫번째랑 두번째 code fence가 다른데 rendering된 결과는 같음...(gfm 기준) mdxt도 같을 듯?

일단 gfm spec 뒤져보자

````

abc

```c
int a = 3;
```

abc

```c
int a = 3;

```

abc

```c
int a = 3;


```

````

---

`[[define, id = table1]]`이랑 `[[reference, id = table1]]`를 만들까? `[[define]]` 안에서 table을 만들고 `[[reference]]`로 그 table을 재활용하는 거임. 이러면 table in table 등등도 전부 구현 가능!

parameter도 넣을 수 있게 할까? macro 안에 대소문자 구분이 없고 띄어쓰기가 다 무시되는데 어떻게 할 거임??

문제가 많을 거 같은데...

1. circular reference
  - circular reference를 찾아내서 막기
    - 어떻게 막을 건데? 컴파일 에러를 낼 순 없잖아.
  - reference depth를 일정 수준 이하로 제한하기.
    - circular든 아니든 reference depth가 최대 5까지만 가능하도록 막기
1. footnote 순서 주기
  - definition 안의 footnote는 별개로 처리할까?
  - definition은 아예 별개로 render를 한 다음에 html 수준에서 concat하는 거지.
  - 그럼 definition 안에서 정의된 footnote들은 cite랑 ref랑 둘 다 그 안에서만 나오는 거임!
  - 이런 식이면 table in table하기 되게 애매하지 않냐
1. header 있으면 toc에는 걔네가 어떻게 들어감? 여러번 들어가? 애초에 들어갈 순서 정하는 것도 되게 애매할 듯...
  - footnote랑 똑같은 방식으로 할까? definition 안에서 전부 먹어버리고 html 결과물만 내뿜는 거지!
1. 앞으로 새로운 거 추가할 때마다 문제도 추가될 듯!

장점도 너무 크고 단점도 너무 커서 애매하네...

사실 definition 안에서 header 쓸 일은 많이 없을 거 같고 정 안되겠으면 header 자체를 막아버릴 수도 있음. 근데 definition 안에서 footnote 쓸 일은 생각보다 있을 거 같은데??

`doc_data`라는 친구가 header도 전부 세고, footnote도 전부 세거든? 근데 모든 parser unit이 `&mut doc_data`을 갖고 있음. 그럼 `definition` 안에 있는 header랑 footnote도 얘가 세면 되는 거 아님?

어쨌든 현재 후보 2개:

1. 각각 별도로 render한 다음에 html 수준에서 합치기
  - 이러면 recursion depth를 제한할 수 있나?
  - toc랑 footnote는?
  - def 밖에서 만든 link를 def 안에서 ref하면?
  - def 안에서 다른 def를 ref하면?
    - 이건 recursion이랑도 이어지는 문제인 듯!
1. 적절히 잘 묶은 다음에 한번에 render
  - mdxt 수준에서 concat해버리는 건 말이 안됨. 그럼 table in table이 구현이 안되거든
  - `def`라는 inline_node를 만들어야지.
  - `def` 안의 toc랑 footnote도 따로 세고
  - rendering할 때 `def`를 html로 바꿈. 그때 recursion depth도 셈.

아니면 좀 덜 flexible하게 할까? table 하나만 하거나 code fence 하나만 하는 거임!

table 안에다가 `!![[ref, id = table1]]` 하거나, fence에다가 ```` ```rust, line_num, ref(code1) ```` 하거나 이런 식으로 하고 나중에 `[[reference, id = code1]]` 이런 식으로 불러오는 거지! 이거는 구현이 그나마 나을 듯?
- 그래도 footnote는 신경 써야함.

---

```markdown
[[box]]
교수님의 질문: 내 폰이랑 공유기랑 통신하고 니네 폰이랑 공유기랑도 통신하지? 그럼 니네 폰에서 공유기로 가는 packet을 내 폰에서도 볼 수 있을까?
- yes.
- 근데 header 확인해보고 나랑 관련없는 packet은 걍 버림. 그래서 내 폰에서 공유기로 가는 정보를 다른 폰에서 못 보는 거임.
  - 엥? 근데 이럼 보안 구멍 아님? 내 폰에서 쓰는 데이터를 친구 폰에서 볼 수 있는 거잖아?
  - ㅇㅇ 그래서 application layer에서 암호화를 해버림. 그럼 중간에 가로채도 알 방법이 없음.
    - 왜 application layer냐? 민감한 정보는 다 저기 있을 거 아녀? 다른 layer에는 공유기 ip 주소같은 안 민감한 정보만 있을 거잖아.
[[/box]]
```

버그 찾았음!

---

```markdown
[[box]]
a
[[/box]]
b
[[box]]
c
[[/box]]
```

이거 지금처럼 되는게 맞아?

---

table에서 `[[background=red]]` 같은 것도 되게 할까?

`[[column background = red]]`나 `[[row background = red]]` 같은 것도 되게 만든 다음에 `cell > row > column` 우선순위로 적용되게 해도 좋을 것 같고.

굳이 색깔만 설정하는게 아니고 다른 것도 되게하면 좋을 거 같고...!!

column이나 row는 어디서 선언해야해?

---

`[[math]]`안에서 `text{=>}` 하니까 죽음...

`inline::math::escape_special_characters`가 범인임. escape 되고 풀고 하는 거 아주 복잡하게 돼 있잖아? 그거 좀 깔끔하게 정리하자!

---

github이나 youtube 같은 거 macro로 지원할까? [linus.dev](https://linus.dev)에 있는 거 같은 github 카드!
- `<iframe width="420" height="315"src="https://www.youtube.com/embed/W_xYzhjIEV8"></iframe>`
- 간단하구먼.
- github는 3rd party js lib 써야함... https://github.com/lepture/github-cards 같은 거??? 별루...

emoji도 지원했으면 좋겠음...
- https://www.w3schools.com/charsets/ref_emoji.asp
- https://www.alt-codes.net/flags
- char랑 겹치는 건 빼자!
