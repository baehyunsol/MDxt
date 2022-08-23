use super::predicate::{read_macro, check_and_parse_macro_inline};
use crate::inline::InlineNode;
use crate::utils::{into_v16, from_v16};
use crate::render::render_option::RenderOption;
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