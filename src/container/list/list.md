- `-`랑 `*`랑 구분 X
- list element 바로 아래에 paragraph가 오면 element에 편입. (empty line 오면 편입 X)
  - 단, 편입 시에는 줄바꿈이나 문단 바꿈 없음. 즉, 한 list 안에 여러 `<p>` 못 씀
  - list 안에 blockquote 넣는 거나 fenced code 넣는 거나 전부 불가능!
- ordered list는 `1. ` ~ `999999999. `로 시작함.
  - `a. `, `A. `, `i. `, `I. ` 등도 가능!
  - 쟤네가 `start`랑 `type`을 결정!
  - 다른 type을 섞어도 상관없음. 첫번째 element의 type이 전체의 type을 결정
- 빈 element 허용 X
- ordered랑 unordered랑 혼용 가능
  - list의 종류는 첫번째 bullet/marker가 결정
- ordered list는 element가 2개 이상 있어야 유효
  - sublist는 element 하나만 있어도 됨.
  - element가 하나고 sublist가 있으면 그것도 ㄱㅊ

이전 indent보다 2칸 이상 더 indent하면 새로운 level
이전 indent보다 2칸 이상 덜 indent하면 이전 level로 돌아감.
  - 현재 indent보다 더 작은 indent가 나올 때까지 traverse하면서 전부 닫음.
  - traverse가 끝났을 때, 그 indent와 현재 indent가 1칸 이하로 차이나면 거기에 붙이고, 아니면 걔의 sublist로 붙이고

```
- !![[no bullet]]
- no
- bullet
  - 123
  - 456
- 789
  a. !![[start = 24]]
  a. `[[start = x]]` is invalid.
  a. hahaha
```