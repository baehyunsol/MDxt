## TODO

Underscore Emphasis

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

collapsible

... 어떻게 구현?

---

htmx처럼 내용 바꿔치기 구현

e.g) 표 2개를 만들어 두고, (날짜순 정렬, 금액순 정렬) 화면에는 하나만 표시. 버튼을 누르거나 체크박스를 선택하면 표의 내용이 바꿔치기가 됨

구현 자체는 쉬울 거 같은데 mdxt 문법 깔끔하게 만드는게 힘들 듯??

---

SVG: (모든 아이콘에 대해서) 우측 상단에 작게 '+' 모양 추가하기 -> 새로운 아이콘 나옴!
