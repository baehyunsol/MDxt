## TODO

Plugin system

```rust
fn custom_macro(arguments: &Vec<Vec<Vec<u16>>>, content: &Vec<u16>) -> Option<Vec<u16>>;
```

저거 호출을 어느 시점에서 해? custom macro로 table 만들 수도 있나?? 그럼 구현이 좀 빡센데

- 매크로 예시: 아래의 매크로들만 매끄럽게 구현이 가능하면 충분할 듯
  - `[[foo]]bar[[/foo]]` -> `[[div, class = foo, id = foo#]]bar[[/div]]`
    - 37번째 등장한 `foo`의 id는 `foo37`임.
- 구현
  - macro 만나면 일단은 builtin인지 확인해보고, 안되면 저 함수에 넣어서 some인지 보고, some이면 그 문자열 그대로 대체
- 외부에서 어떻게 넣지..??
  - json, toml, ...
    - 아주 간단한 mapping만 가능...ㅜ
  - embedded scripting language
    - 음...
  - Rust
    - 아니 이러면 plugin 만들 때마다 새로 컴파일해?

---

````md

```rust, id(7)
fn main() {
  println!("Hello World!");
}
```

````

저렇게 하면 저 id로 codefence in table도 구현할 수 있고, 저 codefence의 특정 줄을 highlight하는 버튼을 만들 수도 있음!

---

list 다음에 줄바꿈 없이 multiline macro가 오면 macro가 list 안으로 들어가버림. 아마 blockquote도 동일할 듯. table도 실험해보셈. 저게 맞는 동작인지 아닌지는 고민을 좀 해보자

---

underscore로 emphasis하는 문법도 넣을까? 그냥 test case 무지무지 많이 만든 다음에 gfm이랑 동일하게 동작하도록 구현하면 될 듯?

---

indented code blocks -> why not?

https://github.github.com/gfm/#indented-code-blocks

---

list/table/blockquote 안에 list/table/blockquote/fenced_code_block 넣기! ...how?

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

table에서 `[[background=red]]` 같은 것도 되게 할까?

`[[column background = red]]`나 `[[row background = red]]` 같은 것도 되게 만든 다음에 `cell > row > column` 우선순위로 적용되게 해도 좋을 것 같고.

굳이 색깔만 설정하는게 아니고 다른 것도 되게하면 좋을 거 같고...!!

column이나 row는 어디서 선언해야해?

---

table에 rowspan도 추가하자!

```
| [[rowspan=2]]DB  | [[colspan=2]]Creation | [[colspan=2]]Operation |
|                  | Debug    | Release    | Debug    | Release     |
|------------------|----------|------------|----------|-------------|
```

colspan은 cell을 안 만들어도 되잖아? rowspan은 cell을 만들어야함. 그대신 해당 cell은 무시됨. (안의 내용도 무시됨)

md로 쓰는 건 간편하다고 쳐도 구현하는게 개빡셀 듯... 일단 보류...

---

ordered, unordered list도 collapsible하게 하자!

```
- !![[collapsible, default=hidden]]1
  - 2
  - 3
  - 4
- 5
- 6
```

저러면 2, 3, 4가 default로 안 보임 -> 1~6이 다 안 보여야 하는 거 아님?

---

asciigraph inside fenced code blocks

diagrams inside fenced code blocks

````

```math
sqrt{3 + 3 + 3} = 3
```

````

저거도 `[[math]]`처럼 작동하게 할까?

---

collapsible

... 어떻게 구현?

---

multiline macro에 id 부여한다고 까불던 거 없애기! -> dependency에서 rand를 통째로 없애기!

---

extra syntax set 관련된 것도 언급 ㄱㄱ

sidebar default visibility -> 까먹고 아직 구현 안함! ㅋㅋㅋ