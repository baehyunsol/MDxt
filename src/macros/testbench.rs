use crate::utils::into_v16;
use super::predicate::read_macro;

fn valid_macros() -> Vec<(Vec<u16>, Vec<u16>)> {  // case, answer
    let macros = vec![
        ("[[blue]]", "blue"),
        ("[[red ]]", "red"),
        ("[[Red_]]", "red"),
        ("[[icon = github, size = 24]]", "icon=github,size=24")
    ];

    macros.iter().map(|(case, answer)| (into_v16(case), into_v16(answer))).collect()
}

fn invalid_macros() -> Vec<Vec<u16>> {
    let macros = vec![
        "[ [red]]", "[[red] ]",
        "[[[icon = github, size = 24]]",
        "[[big!!]]",
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

    assert_eq!(valid_cases, valid_answers);

    let invalid_cases = invalid.iter().map(|m| read_macro(m, 0)).collect::<Vec<Option<Vec<u16>>>>();

    assert!(invalid_cases.iter().all(|i| i.is_none()));
}