```
*abc*: <em>abc</em>

*abc**: <em>abc</em>*

***abc**: *<strong>abc</strong>

****abc***: *<em><strong>abc</strong></em>

**abc***: <strong>abc</strong>*

*abc *: *abc *

*abc**def**ghi*: <em>abc<strong>def</strong>ghi</em>

*abc **def** ghi*: <em>abc <strong>def</strong> ghi</em>

*abc ** def ** ghi*: <em>abc ** def ** ghi</em>

*abc*def*: <em>abc</em>def*

*abc * def*: <em>abc * def</em>

*abc ** def*: <em>abc ** def</em>

**abc*def*ghi**: <strong>abc<em>def</em>ghi</strong>

*abc**def*ghi**: <em>abc**def</em>ghi**

*abc~~abcd~~abc*: <em>abc<del>abcd</del>abc</em>

*abc~~abcd*abc~~: <em>abc~~abcd</em>abc~~

*abc`abcd`abc*: <em>abc<code class="short">abcd</code>abc</em>

*abc`abcd*abc`: *abc<code class="short">abcd*abc</code>

*abc\*: *abc*

`abc\`: <code class="short">abc\</code>

`a``b`: <code class="short">a``b</code>
```

전략
- inline code span은 가장 먼저 처리한다.
- 가장 바깥의 pattern을 확인한 후, 그 pattern의 이전, 내부, 이후를 각각 따로 처리한다.
  - 가장 먼저 시작되는 pattern부터 확인한다는 뜻!