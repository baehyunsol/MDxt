use super::{Math, md_to_math};
use super::{ZERO_ARG_FUNCTIONS, ONE_ARG_FUNCTIONS, TWO_ARG_FUNCTIONS, THREE_ARG_FUNCTIONS, FIVE_ARG_FUNCTIONS};
use crate::utils::{from_v32, into_v32, remove_whitespaces};
use crate::{render_to_html, RenderOption, RenderResult};
use crate::testbench::HXML_LOCK;

fn samples() -> Vec<(Vec<u32>, Vec<u32>)> {  // Vec<(test_case, answer)>
    let result = vec![
("sum{x}", "
<math>
    <mi>sum</mi>
    <mo>{</mo>
    <mi>x</mi>
    <mo>}</mo>
</math>
"), ("sqrt{3}{4}{5}", "
<math>
    <mi>sqrt</mi>
    <mo>{</mo>
    <mn>3</mn>
    <mo>}</mo>
    <mo>{</mo>
    <mn>4</mn>
    <mo>}</mo>
    <mo>{</mo>
    <mn>5</mn>
    <mo>}</mo>
</math>
"), ("frac{1}{1}{1}", "
<math>
    <mi>frac</mi>
    <mo>{</mo>
    <mn>1</mn>
    <mo>}</mo>
    <mo>{</mo>
    <mn>1</mn>
    <mo>}</mo>
    <mo>{</mo>
    <mn>1</mn>
    <mo>}</mo>
</math>
"), ("sum{n=1}{+inf} frac{1}{sup{n}{2}} = frac{sup{pi}{2}}{6}", "
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
    <mo>&lt;</mo>
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
"), ("mat{}{}{}", "
<math>
    <mtext>Error: Empty Row</mtext>
</math>
"), ("(mat{{1}{2}{3}}{{4}{5}{6}}{{7}{9}{10}})", "
<math>
    <mo>(</mo>
    <mtable>
        <mtr>
            <mtd><mn>1</mn></mtd>
            <mtd><mn>2</mn></mtd>
            <mtd><mn>3</mn></mtd>
        </mtr>
        <mtr>
            <mtd><mn>4</mn></mtd>
            <mtd><mn>5</mn></mtd>
            <mtd><mn>6</mn></mtd>
        </mtr>
        <mtr>
            <mtd><mn>7</mn></mtd>
            <mtd><mn>9</mn></mtd>
            <mtd><mn>10</mn></mtd>
        </mtr>
    </mtable>
    <mo>)</mo>
</math>"), ("", "<math></math>")
    ];

    result.iter().map(
        |(test_case, answer)|
        (into_v32(test_case), into_v32(answer))
    ).collect()
}

#[test]
fn math_ml_test() {

    for (test_case, answer) in samples() {
        let math_obj = Math::from_mdxt(&test_case);
        let rendered = math_obj.to_math_ml(false);

        if remove_whitespaces(&answer) != remove_whitespaces(&rendered) {
            panic!(
                "input: {}\nanswer: {}\noutput: {}",
                from_v32(&test_case),
                from_v32(&answer),
                from_v32(&rendered),
            );
        }

    }

}

fn escape_math(math: &[u32]) -> Vec<u32> {

    let mut result = Vec::with_capacity(math.len() * 5 / 4);

    for c in math.iter() {

        if *c == '|' as u32 {
            result.push('\\' as u32);
        }

        result.push(*c);
    }

    result
}

fn render_math_reference() -> Vec<u32> {

    let mut result = vec![];
    result.push(into_v32("# Math in MDxt\n\n"));
    result.push(into_v32("| Table of Contents |\n"));
    result.push(into_v32("|-|\n"));
    result.push(into_v32("|!![[collapsible]]|\n"));
    result.push(into_v32("|[[toc]]|\n\n"));
    result.push(into_v32("## Examples\n\n"));
    result.push(into_v32("| [[colspan=2]] examples |\n"));
    result.push(into_v32("| mdxt | result |\n"));
    result.push(into_v32("|-|-|\n"));

    for (test_case, _) in samples() {

        // too short to be a meaningful example
        if test_case.len() < 32 {
            continue;
        }

        result.push(into_v32("|\\[[math]]"));
        result.push(escape_math(&test_case));
        result.push(into_v32("[[/math]]|[[math]]"));
        result.push(test_case);
        result.push(into_v32("[[/math]]|\n"));
    }

    result.push(into_v32("\n\n## Special Entities\n\n"));

    let mut entities = ZERO_ARG_FUNCTIONS.iter().map(|e| from_v32(e)).collect::<Vec<String>>();
    entities.sort();

    for entity in entities.iter() {
        result.push(into_v32(&format!(
            "\n\n### {entity}\n\n{}`[[math]]{entity}[[/math]]` [[char=rarr]] [[math]]{entity}[[/math]]",
            if entity == "br" {
                "It works only with multiline math macros. It breaks new line.[[br]]TODO: This entity may not be seen on some browsers.\n\n"
            } else {
                ""
            }
        )));
    }

    result.push(into_v32("\n\n## Functions with one argument\n\n"));

    let mut entities = ONE_ARG_FUNCTIONS.iter().map(|e| from_v32(e)).collect::<Vec<String>>();
    entities.sort();

    for entity in entities.iter() {
        result.push(into_v32(&format!(
            "\n\n### {entity}\n\n`[[math]]123{entity}{}456{}789[[/math]]` [[char=rarr]] [[math]]123{entity}{}456{}789[[/math]]",
            '{', '}',
            '{', '}'
        )));
    }

    result.push(into_v32("\n\n## Functions with two arguments\n\n"));

    let mut entities = TWO_ARG_FUNCTIONS.iter().map(|e| from_v32(e)).collect::<Vec<String>>();
    entities.sort();

    for entity in entities.iter() {
        result.push(into_v32(&format!(
            "\n\n### {entity}\n\n`[[math]]123{entity}{}456{}{}789{}012[[/math]]` [[char=rarr]] [[math]]123{entity}{}456{}{}789{}012[[/math]]",
            '{', '}', '{', '}',
            '{', '}', '{', '}'
        )));
    }

    result.push(into_v32("\n\n## Functions with three arguments\n\n"));

    let mut entities = THREE_ARG_FUNCTIONS.iter().map(|e| from_v32(e)).collect::<Vec<String>>();
    entities.sort();

    for entity in entities.iter() {
        result.push(into_v32(&format!(
            "\n\n### {entity}\n\n`[[math]]123{entity}{}456{}{}789{}{}012{}345[[/math]]` [[char=rarr]] [[math]]123{entity}{}456{}{}789{}{}012{}345[[/math]]",
            '{', '}', '{', '}', '{', '}',
            '{', '}', '{', '}', '{', '}'
        )));
    }

    result.push(into_v32("\n\n## Functions with five arguments\n\n"));

    let mut entities = FIVE_ARG_FUNCTIONS.iter().map(|e| from_v32(e)).collect::<Vec<String>>();
    entities.sort();

    for entity in entities.iter() {
        result.push(into_v32(&format!(
            "\n\n### {entity}\n\n`[[math]]123{entity}{}456{}{}789{}{}012{}{}345{}{}678{}901[[/math]]` [[char=rarr]] [[math]]123{entity}{}456{}{}789{}{}012{}{}345{}{}678{}901[[/math]]",
            '{', '}', '{', '}', '{', '}', '{', '}', '{', '}',
            '{', '}', '{', '}', '{', '}', '{', '}', '{', '}'
        )));
    }

    result.push(into_v32("\n\n## Matrices\n\n"));
    let mut mats = vec![];
    mats.push("{{1}{2}{3}}{{4}{5}{6}}{{7}{8}{9}}".to_string());
    mats.push("{{1}{2}{3}}{{4}{5}{6}}{{7}{8}{9}}{{a}{b}{c}}".to_string());
    mats.push("{{1}{2}{3}{4}}{{5}{6}{7}{8}}{{9}{a}{b}{c}}".to_string());
    mats.push("{{1}{2}{3}{4}}{{5}{6}{7}}".to_string());
    mats.push("{}{}{}".to_string());

    for mat in mats.into_iter() {
        result.push(into_v32(&format!("
            \n`[[math]](mat{mat})[[/math]]` [[char=rarr]] [[math]](mat{mat})[[/math]]\n
        ")));
    }

    result.concat()
}

#[test]
fn render_math_to_html() {
    use std::fs::File;
    use std::io::{Read, Write};

    let md = render_math_reference();

    if crate::PRINT_TEST_PAGES { println!("\n\n{}\n\n", from_v32(&md)); }

    let md = from_v32(&md);
    let RenderResult {
        content: html,
        ..
    } = render_to_html(
        &md,
        RenderOption {
            xml: true,
            ..RenderOption::default()
        }
    );

    let lock = unsafe { HXML_LOCK.lock().unwrap() };

    hxml::into_dom(html.clone()).unwrap();

    // TODO: be sure that all the math ml entities are properly rendered

    drop(lock);

    let mut f = File::open("./styles/markdown.css").unwrap();
    let mut css = String::new();
    f.read_to_string(&mut css).unwrap();
    let html = format!(
"
<!DOCTYPE html>
<html>
<head>
    <title>MDxt Math Reference</title>
    <style>{css}</style>
</head>
<body>
    <article class=\"markdown\">{html}</article>
</body>
</html>
",
    );

    let mut f = File::create("math_test.html").unwrap();
    f.write_all(html.as_bytes()).unwrap();

}

#[test]
fn md_to_math_test() {
    assert!(md_to_math(&[]).is_empty());
    assert!(md_to_math(&into_v32("   ")).is_empty());
}
