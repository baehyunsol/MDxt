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

pretty output
- output으로 나오는 html 좀 예쁘게 indent도 하고 그러자
- 이거 만드는 과정에서 html-validator도 나올 듯...

---

list 다음에 줄바꿈 없이 multiline macro가 오면 macro가 list 안으로 들어가버림. 아마 blockquote도 동일할 듯. table도 실험해보셈. 저게 맞는 동작인지 아닌지는 고민을 좀 해보자

---

난독화

class들 난독화하는 기능 추가, 난독화하는 함수 자체도 넣어서 배포!
