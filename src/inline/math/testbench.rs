use super::{Math, md_to_math};
use crate::utils::{from_v16, into_v16, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn md_samples1() -> Vec<(Vec<u16>, Vec<u16>)> {  // Vec<(test_case, answer)>
    let result = vec![
("sum{n=1}{+inf} frac{1}{sup{n}{2}} = frac{sup{pi}{2}}{6}", "
<math>
    <munderover displaystyle=\"true\">
        <mo>∑</mo>
        <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
        <mrow><mo>+</mo><mn>&#8734;</mn></mrow>
    </munderover>
    <mfrac displaystyle=\"false\">
        <mn>1</mn>
        <msup>
            <mi>n</mi>
            <mn>2</mn>
        </msup>
    </mfrac>
    <mo>=</mo>
    <mfrac displaystyle=\"false\">
        <msup>
            <mi>&#960;</mi>
            <mn>2</mn>
        </msup>
        <mn>6</mn>
    </mfrac>
</math>
"), ("frac{-b pm sqrt{sup{b}{2} - 4 a c}}{2 a}", "
<math>
    <mfrac displaystyle=\"false\">
        <mrow>
            <mo>-</mo>
            <mi>b</mi>
            <mo>&#177;</mo>
            <msqrt>
                <msup>
                    <mi>b</mi>
                    <mn>2</mn>
                </msup>
                <mo>-</mo>
                <mn>4</mn>
                <mi>a</mi>
                <mi>c</mi>
            </msqrt>
        </mrow>
        <mrow>
            <mn>2</mn>
            <mi>a</mi>
        </mrow>
    </mfrac>
</math>
"), ("lim{n rightarrow +inf} sup{(1 + frac{1}{n})}{n} = e simeq 2.718", "
<math>
    <munder>
        <mi>lim</mi>
        <mrow>
            <mi>n</mi>
            <mo>&#8594;</mo>
            <mo>+</mo>
            <mn>&#8734;</mn>
        </mrow>
    </munder>
    <msup>
        <mrow>
            <mo>(</mo>
            <mn>1</mn>
            <mo>+</mo>
            <mfrac displaystyle=\"false\">
                <mn>1</mn>
                <mi>n</mi>
            </mfrac>
            <mo>)</mo>
        </mrow>
        <mi>n</mi>
    </msup>
    <mo>=</mo>
    <mi>e</mi>
    <mo>&#8771;</mo>
    <mn>2.718</mn>
</math>
"), ("(bincoeff{5}{2}) = multiscript{C}{}{}{5}{2} = frac{5!}{2!3!} = 10", "
<math>
    <mo>(</mo>
    <mfrac displaystyle=\"false\" linethickness=\"0\">
        <mn>5</mn>
        <mn>2</mn>
    </mfrac>
    <mo>)</mo>
    <mo>=</mo>
    <mmultiscripts>
        <mi>C</mi>
        <mn>2</mn>
        <none/>
        <mprescripts/>
        <mn>5</mn>
        <none/>
    </mmultiscripts>
    <mo>=</mo>
    <mfrac displaystyle=\"false\">
        <mrow>
            <mn>5</mn>
            <mo>!</mo>
        </mrow>
        <mrow>
            <mn>2</mn>
            <mo>!</mo>
            <mn>3</mn>
            <mo>!</mo>
        </mrow>
    </mfrac>
    <mo>=</mo>
    <mn>10</mn>
</math>
"), ("a circ b = |a| space |b| space cos theta", "
<math>
    <mi>a</mi>
    <mo>&#8728;</mo>
    <mi>b</mi>
    <mo>=</mo>
    <mo>|</mo>
    <mi>a</mi>
    <mo>|</mo>
    <mspace width=\"0.333em\"/>
    <mo>|</mo>
    <mi>b</mi>
    <mo>|</mo>
    <mspace width=\"0.333em\"/>
    <mi>cos</mi>
    <mi>&#952;</mi>
</math>
"), ("alpha beta gamma Alpha Beta Gamma", "
<math>
    <mi>&#945;</mi>
    <mi>&#946;</mi>
    <mi>&#947;</mi>
    <mi>&#913;</mi>
    <mi>&#914;</mi>
    <mi>&#915;</mi>
</math>
"), ("broken sqrt{1", "
<math>
    <mi>broken</mi>
    <mi>sqrt</mi>
    <mo>{</mo>
    <mn>1</mn>
</math>
"), ("text{delta} delta", "
<math>
    <mtext>delta</mtext>
    <mi>&#948;</mi>
</math>
"), ("sub{Phi}{E} = oint{S}{} E circ d A", "
<math>
    <msub>
        <mi>&#934;</mi>
        <mi>E</mi>
    </msub>
    <mo>=</mo>
    <munder displaystyle=\"true\">
        <mo>∮</mo>
        <mi>S</mi>
    </munder>
    <mi>E</mi>
    <mo>&#8728;</mo>
    <mi>d</mi>
    <mi>A</mi>
</math>
"), ("1+1=2", "
<math>
    <mn>1</mn>
    <mo>+</mo>
    <mn>1</mn>
    <mo>=</mo>
    <mn>2</mn>
</math>
"), ("a space b sspace c ssspace d", "
<math>
    <mi>a</mi>
    <mspace width=\"0.333em\"/>
    <mi>b</mi>
    <mspace width=\"0.667em\"/>
    <mi>c</mi>
    <mspace width=\"1em\"/>
    <mi>d</mi>
</math>
"), ("root{4}{sup{|a|}{4} + sup{|b|}{4} + sup{|c|}{4} + sup{|d|}{4}} leq sup{(a+b+c+d)}{4} < inf", "
<math>
    <mroot>
        <mrow>
            <msup>
                <mrow>
                    <mo>|</mo>
                    <mi>a</mi>
                    <mo>|</mo>
                </mrow>
                <mn>4</mn>
            </msup>
            <mo>+</mo>
            <msup>
                <mrow>
                    <mo>|</mo>
                    <mi>b</mi>
                    <mo>|</mo>
                </mrow>
                <mn>4</mn>
            </msup>
            <mo>+</mo>
            <msup>
                <mrow>
                    <mo>|</mo>
                    <mi>c</mi>
                    <mo>|</mo>
                </mrow>
                <mn>4</mn>
            </msup>
            <mo>+</mo>
            <msup>
                <mrow>
                    <mo>|</mo>
                    <mi>d</mi>
                    <mo>|</mo>
                </mrow>
                <mn>4</mn>
            </msup>
        </mrow>
        <mn>4</mn>
    </mroot>
    <mo>&#8804;</mo>
    <msup>
        <mrow>
            <mo>(</mo>
            <mi>a</mi>
            <mo>+</mo>
            <mi>b</mi>
            <mo>+</mo>
            <mi>c</mi>
            <mo>+</mo>
            <mi>d</mi>
            <mo>)</mo>
        </mrow>
        <mn>4</mn>
    </msup>
    <mo><</mo>
    <mn>&#8734;</mn>
</math>
"), ("hat{1} bar{1} dot{1} tilde{1} vec{1}", "
<math>
    <mover>
        <mn>1</mn>
        <mo>&#94;</mo>
    </mover>
    <mover>
        <mn>1</mn>
        <mo>&#45;</mo>
    </mover>
    <mover>
        <mn>1</mn>
        <mo>&#8901;</mo>
    </mover>
    <mover>
        <mn>1</mn>
        <mo>&#126;</mo>
    </mover>
    <mover>
        <mn>1</mn>
        <mo>&#8594;</mo>
    </mover>
</math>
"), ("", "<math></math>")
    ];

    result.iter().map(
        |(test_case, answer)|
        (into_v16(test_case), into_v16(answer))
    ).collect()
}

#[test]
fn math_ml_test() {

    for (test_case, answer) in md_samples1() {
        let math_obj = Math::from_mdxt(&test_case);
        let rendered = math_obj.to_math_ml(false);

        if remove_whitespaces(&answer) != remove_whitespaces(&rendered) {
            panic!(
                "input: {}\nanswer: {}\noutput: {}",
                from_v16(&test_case),
                from_v16(&answer),
                from_v16(&rendered),
            );
        }

    }

}

#[test]
fn render_to_html() {
    use std::fs::File;
    use std::io::Write;

    let mut md = vec![];

    for (test_case, _) in md_samples1() {
        md.push(into_v16("[[math]]"));
        md.push(test_case);
        md.push(into_v16("[[/math]]\n\n"));
    }

    let md = from_v16(&md.concat());
    let html = format!(
"
<!DOCTYPE html>
<html>
<head>
    <title>MDxt Reference</title>
</head>
<body>
    <article class=\"markdown\">{}</article>
</body>
</html>
",
        render_to_html_with_default_options(&md)
    );

    let mut f = File::create("math_test.html").unwrap();
    f.write_all(html.as_bytes()).unwrap();

}

#[test]
fn md_to_math_test() {
    assert!(md_to_math(&[]).len() == 0);
    assert!(md_to_math(&into_v16("   ")).len() == 0);
}