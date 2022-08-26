use super::predicate::{read_macro, check_and_parse_macro_inline};
use crate::inline::InlineNode;
use crate::utils::{into_v16, from_v16};
use crate::render::render_option::RenderOption;
use crate::render_to_html_with_default_options;
use crate::ast::doc_data::DocData;

fn valid_macros() -> Vec<(Vec<u16>, Vec<u16>)> {  // valid macro, normalized
    let macros = vec![
        ("[[br]]", "br"),
        ("[[blue]] [[/blue]]", "blue"),
        ("[[red ]] ... [[/ red]]", "red"),
        ("[[Red_]] ... [[/Red]]", "red"),
        ("[[char = 44032]]", "char=44032"),
        ("[[highlight = red]] ... [[/highlight]]", "highlight=red"),
        //("[[icon = github, size = 24]]", "icon=github,size=24")
    ];

    macros.iter().map(|(case, answer)| (into_v16(case), into_v16(answer))).collect()
}

fn invalid_macros() -> Vec<Vec<u16>> {
    let macros = vec![
        "[ [red]]", "[[red] ]",
        "[[big!!]]",
        "[[[icon = github, size = 24]]",
        "[[]]", "[[ ]]", "[[__]]"
    ];

    macros.iter().map(|m| into_v16(m)).collect()
}

#[test]
fn macro_test() {
    let valid = valid_macros();
    let invalid = invalid_macros();

    let valid_cases = valid.iter().map(|m| read_macro(&m.0, 0).unwrap()).collect::<Vec<Vec<u16>>>();
    let valid_answers = valid.iter().map(|m| m.1.clone()).collect::<Vec<Vec<u16>>>();

    if valid_cases != valid_answers {
        panic!(
            "{:?}\n{:?}",
            valid_cases.iter().map(|c| from_v16(c)).collect::<Vec<String>>(),
            valid_answers.iter().map(|c| from_v16(c)).collect::<Vec<String>>(),
        );
    }

    let invalid_cases = invalid.iter().map(|m| read_macro(m, 0)).collect::<Vec<Option<Vec<u16>>>>();

    if !invalid_cases.iter().all(|i| i.is_none()) {
        panic!("{:?}", invalid_cases);
    }

    let valid_cases_parsed = valid.iter().map(|m| check_and_parse_macro_inline(&m.0, 0, &mut DocData::default(), &mut RenderOption::default())).collect::<Vec<Option<(InlineNode, usize)>>>();

    for (index, parsed) in valid_cases_parsed.iter().enumerate() {

        if parsed.is_none() {
            panic!("failed to parse ({}, {})", from_v16(&valid[index].0), from_v16(&valid[index].1));
        }

    }

}

fn render_character_reference() -> Vec<u16> {
    let mut char_names = super::character::CHAR_NAMES.clone().into_iter().collect::<Vec<Vec<u16>>>();
    char_names.sort();

    let mut md_lines = vec![];

    md_lines.push(String::from("# MDxt Character Reference\n\n"));
    md_lines.push(String::from("| [[colspan=2]] MDxt Character Reference |\n"));
    md_lines.push(String::from("| MDxt | Result |\n"));
    md_lines.push(String::from("|------|--------|\n"));

    for char_name in char_names {
        md_lines.push(format!("| \\[[char={}]] | [[char={}]] |\n", from_v16(&char_name), from_v16(&char_name)));
    }

    into_v16(&md_lines.concat())
}

#[test]
fn render_to_html() {
    use std::fs::File;
    use std::io::Write;

    let md = render_character_reference();

    let md = from_v16(&md);
    let html = format!(
"
<!DOCTYPE html>
<html>
<head>
    <title>MDxt Math Reference</title>
</head>
<body>
    <article class=\"markdown\">{}</article>
</body>
</html>
",
        render_to_html_with_default_options(&md)
    );

    let mut f = File::create("character_test.html").unwrap();
    f.write_all(html.as_bytes()).unwrap();

}