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
        <mrow><mo>+</mo><mo>&#8734;</mo></mrow>
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
            <mo>&#960;</mo>
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
            <mo>&#8734;</mo>
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
    <mo>)=</mo>
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

"), ("alpha beta gamma Alpha Beta Gamma", "

"), ("broken sqrt{1", "

"), ("text{delta} delta", "

"), ("sub{Phi}{E} = oint{S}{} E circ d A", "

"), ("1+1=2", "

"), ("space sspace ssspace", "

"), ("root{4}{sup{|a|}{4} + sup{|b|}{4} + sup{|c|}{4} + sup{|d|}{4}} leq sup{(a+b+c+d)}{4} < inf", "

"), ("", "")
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
fn integration_test() {}  // inside mdxt

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