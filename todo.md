## TODO

normalize_어쩌구저쩌구, link_handler 등에 특수문자를 넣어서 저 함수를 여러번 거치는 친구들이 있는지 검사

ex) normalize 함수를 거치면 결과물에 0x8000이 포함되게 하고, normalize 함수에 들어온 input에 0x8000이 포함돼 있으면 에러 던지기

---

Plugin system

macro 처리하는 함수가, 일단 지 거 해보고 안되는 거 있으면 사용자가 넘겨준 함수한테 매크로 처리 시키는 거지! `arguments: Vec<Vec<Vec<u16>>>, content: Vec<u16> -> Option<Vec<u16>>`의 함수를 사용자가 만들면 그게 곧 plugin 아님?

---

tooltip

얘도 `[[span]]`으로 어찌저찌 하거나 걍 plugin으로 편입시켜버리면 될 듯!

---

mdx engine이 mathjax 결과물까지 다 처리한 다음에 최종 결과물에는 svg만 넣으면 안되나?

---

prefix

html로 변환할 때 class랑 id 앞에다가 일괄적으로 prefix 붙이기

---

pretty output
- output으로 나오는 html 좀 예쁘게 indent도 하고 그러자
- 이거 만드는 과정에서 html-validator도 나올 듯...

---

link에 그냥 `index.md`라고 적으면 `https://blog.com/index.md`처럼 앞에다가 prefix 주소 붙여주기! prefix도 정할 수 있게 하자!
