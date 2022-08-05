# Math

## MathML Examples

```xml
<math display="block" xmlns="http://www.w3.org/1998/Math/MathML">
    <semantics>
        <mrow>
            <mi>x</mi>
            <mo>=</mo>
            <mfrac>
                <mrow>
                    <mo form="prefix">&minus;</mo>
                    <mi>b</mi>
                    <mo>&pm;</mo>
                        <msqrt>
                            <msup><mi>b</mi><mn>2</mn></msup>
                            <mo>&minus;</mo>
                            <mn>4</mn><mo>&it;</mo><mi>a</mi><mo>&it;</mo><mi>c</mi>
                        </msqrt>
                </mrow>
                <mrow>
                    <mn>2</mn>
                    <mo>&it;</mo>
                    <mi>a</mi>
                </mrow>
            </mfrac>
        </mrow>
        <annotation encoding="application/x-tex"><!-- TeX -->
            x = \frac{-b\pm\sqrt{b^2-4ac}}{2a}
        </annotation>
        <annotation encoding="StarMath 5.0">
            x = {-b plusminus sqrt {b^2 - 4 ac}} over {2 a}
        </annotation>
        <!-- More annotations can be written: application/x-troff-eqn for eqn, application/x-asciimath for AsciiMath... -->
        <!-- Semantic MathML go under <annotation-xml encoding="MathML-Content">. -->
    </semantics>
</math>

<math display="block">
    <mrow>
        <munderover>
            <mo>∑</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mo>+</mo><mn>∞</mn></mrow>
        </munderover>
        <mfrac>
            <mn>1</mn>
            <msup><mi>n</mi><mn>2</mn></msup>
        </mfrac>
    </mrow>
    <mo>=</mo>
    <mfrac>
        <msup><mi>π</mi><mn>2</mn></msup>
        <mn>6</mn>
    </mfrac>
</math>
<math>
    <mmultiscripts>
        <mn>1</mn>
        <mn>2</mn>
        <mn>3</mn>
        <none/>
        <mn>5</mn>
        <mprescripts/>
        <mn>6</mn>
        <none/>
        <mn>8</mn>
        <mn>9</mn>
    </mmultiscripts>
</math>
```

## MathML Syntax

- `<math>`
  - attributes
    - `display="block"`, `display="inline"`
      - css에서 보던 거
  - 웬만한 CSS 다 먹음
- `<mfrac>`
  - fraction
  - 안에 element가 2개 오면 걔네가 각각 분자, 분모임
    - element 개수가 틀리면 render 실패...
  - 무슨 element가 오는지는 신경 안 쓰는 듯
  - attributes
    - `displaystyle="true"`
      - true면 cfrac, false면 frac
        - 근데 test 해보니까 차이가 없는데...??
      - `[[math]]` 구현할 때는 cfrac도 가능하게 하자 그래야 하위호환이 어느정도 되지.
    - `linethickness="200%"`
      - 0% 하면 ~n~C~k~ 표현 가능
- `<munderover>`, `<mover>`, `<munder>`
  - 시그마나 리미트 등등 표기할 때 씀
  - 가운데/아래/위, 가운데/위, 가운데/아래
- `<mroot>`, `<msqrt>`
  - mroot는 n제곱근, msqrt는 그냥 제곱근
  - 그래서 mroot는 인수가 두개임. 두번째 인수가 n
- `<msup>`, `<msub>`, `<msubsup>`
  - 가운데/아래, 가운데/위, 가운데/아래/위
  - 아 이거는 하위 호환 지키기 힘들겠는데...
- `<mmultiscripts>`, `<mprescripts>`
  - `<mmultiscripts>`의 가장 첫 element가 가운데
  - 그 이후로는 아래->위 순서로 가운데에서 멀어지는 방향으로 하나씩 postscript를 채워감
  - 아래나 위를 비우고 싶으면 그 차례에 element 자리에 `<none/>`을 넣으면 됨
  - postscript 그만 쓰고 prescript 쓰고 싶으면 `<mprescripts/>` 넣으면 됨
    - 그럼 또 가운데에서 멀어지는 방향으로 prescript 하나씩 채워감
- `<mi>`
  - identifier
  - 글자 표시
  - 1글자는 italic으로 표시, 아니면 정자체로 표시
  - 1글자는 변수로 보고, 여러 글자는 함수 이름으로 보는 듯
  - 여러 글자짜리 변수 쓰고 싶으면 `<mi>a</mi><mi>b</mi><mi>c</mi>` 이런 식으로 나눠서 써야 함.
- `<mn>`
  - 숫자랑 소수점
  - 오잉 inf도 `<mn>` 안에 넣는데?
- `<mo>`
  - operator
  - 모든 문자는 `<mn>`, `<mi>` 혹은 `<mo>` 안에 있어야 하는 듯, 슥 보고 어디 넣을지 모르겠으면 `<mo>` 안에 넣으셈
  - attributes
    - `form="prefix"`
      - 좌우 margin이 확 줄어듦
  - `<mo>` 안에 있는 친구들은 `::selection`이 안 먹는데..??
    - 이게 구현이 덜 된건지 원래 spec이 이런 건지를 모르겠으니까 일단은 기다려보자.
  - 시그마는 `<mo>` 안에 넣어야 크기가 보기 좋게 커짐
- `<mrow>`
  - mfrac이 원소 2개만 받는다고? 난 분자에 아주 복잡한 식 쓰고 싶은데...
  - 그럼 `<mrow>`로 묶은 다음에 분자에 넣으면 됨!
- `<mtext>`
  - 내가 지금까지 text로 쓰던 친구들
  - 그냥 스페이스바는 안 통하니까 `&nbsp;` 쓰자!

## User Agent StyleSheet for MathML

이거 넣어줘야 한다는데, 이거 없어도 되던데? 걍 브라우저에 기본으로 들어가 있는 건가...?

```css
@namespace url(http://www.w3.org/1998/Math/MathML);

/* Universal rules */
* {
    font-size: math;
    display: block math;
}

/* The <math> element */
math {
  direction: ltr;
  writing-mode:  horizontal-tb;
  text-indent: 0;
  letter-spacing: normal;
  line-height: normal;
  word-spacing: normal;
  font-family: math;
  font-size: inherit;
  font-style: normal;
  font-weight: normal;
  display: inline math;
  math-style: compact;
  math-shift: normal;
  math-level: 0;
}
math[display="block" i] {
  display: block math;
  math-style: normal;
}
math[display="inline" i] {
  display: inline math;
  math-style: compact;
}

/* <mrow>-like elements */
semantics > :not(:first-child) {
  display: none;
}
maction > :not(:first-child) {
  display: none;
}
merror {
 border: 1px solid red;
 background-color: lightYellow;
}
mphantom {
  visibility: hidden;
}

/* Token elements */
mi {
  text-transform: math-auto;
}

/* Tables */
mtable {
  display: inline-table;
  math-style: compact;
}
mtr {
  display: table-row;
}
mtd {
  display: table-cell;
  text-align: center;
  padding: 0.5ex 0.4em;
}

/* Fractions */
mfrac {
  padding-inline-start: 1px;
  padding-inline-end: 1px;
}
mfrac > * {
  math-depth: auto-add;
  math-style: compact;
}
mfrac > :nth-child(2) {
  math-shift: compact;
}

/* Other rules for scriptlevel, displaystyle and math-shift */
mroot > :not(:first-child) {
  math-depth: add(2);
  math-style: compact;
}
mroot, msqrt {
  math-shift: compact;
}
msub > :not(:first-child),
msup > :not(:first-child),
msubsup > :not(:first-child),
mmultiscripts > :not(:first-child),
munder > :not(:first-child),
mover > :not(:first-child),
munderover > :not(:first-child) {
  math-depth: add(1);
  math-style: compact;
}
munder[accentunder="true" i] > :nth-child(2),
mover[accent="true" i] > :nth-child(2),
munderover[accentunder="true" i] > :nth-child(2),
munderover[accent="true" i] > :nth-child(3) {
  font-size: inherit;
}
msub > :nth-child(2),
msubsup > :nth-child(2),
mmultiscripts > :nth-child(even),
mmultiscripts > mprescripts ~ :nth-child(odd),
mover[accent="true" i] > :first-child,
munderover[accent="true" i] > :first-child {
  math-shift: compact;
}
mmultiscripts > mprescripts ~ :nth-child(even) {
  math-shift: inherit;
}
```

## MDxt Math

- `cfrac{}{}`
  - 하위 호환을 위해서 `frac{}{}`과 `cfrac{}{}`을 동일하게 취급
- `sqrt{}`
  - 인수 1개는 `msqrt`, 인수 2개는 `mroot`로 변환
- `sub{}{}`, `sup{}{}`, `subsup{}{}{}`
  - 이건 하위호환 버리는 수밖에...
- `multiscript{}{}{}{}{}`
  - 가운데/pre-sup/post-sup/pre-sub/post-sub

- 문자열 해석
- `[a-zA-Z]`를 제외한 다른 문자가 나오면 지금까지 읽은 문자열 가지고 다음 중 하나에 해당하는지 확인
  - 함수 이름
    - `{}` 해석 후 valid 하면 함수化
  - 특수문자 이름
    - inf 같은 친구들은 해당 문자로 변환
  - 기타
    - 통째로 `<mi>` 안에 넣으셈. 그래야 한 문자짜리만 기울임체로 변하고
- 숫자 해석
  - `[0-9.]`을 제외한 다른 문자가 나오면 지금까지 읽은 숫자를 `<mn>` 안에 집어넣음
- 기타
  - 뭔지 모르겠는 문자들은 전부 `<mo>` 안에 넣으셈