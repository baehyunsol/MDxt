use super::read_code_fence_info;
use crate::ast::line::Line;
use crate::ast::parse::ParseState;
use crate::utils::into_v16;

fn fence_samples() -> Vec<(
    String,         // case
    bool,           // is_valid
    bool,           // is_code_fence_begin
    bool,           // is_code_fence_end
    String,         // language
    Option<usize>,  // line_num
    usize,          // fence_size
    bool            // is_tilde_fence
)> {
    let samples = vec![
        ("```", true, true, true, "", None, 3, false),
        ("````", true, true, true, "", None, 4, false),
        ("~~~", true, true, true, "", None, 3, true),
        ("~~~~", true, true, true, "", None, 4, true),
        ("``", false, false, false, "", None, 0, false),
        ("``` `", false, false, false, "", None, 0, false),
        ("```rust `", false, false, false, "", None, 0, false),
        ("`~~", false, false, false, "", None, 0, false),
        ("~~`", false, false, false, "", None, 0, false),
        ("```Rust", true, true, false, "rust", None, 3, false),
        ("``` rust", true, true, false, "rust", None, 3, false),
        ("``` line_num", true, true, false, "", Some(1), 3, false),
        ("```line_num(5)", true, true, false, "", Some(5), 3, false),
        ("```rust, line_num(5)", true, true, false, "rust", Some(5), 3, false),
        ("```line_num(5), rust", true, true, false, "rust", Some(5), 3, false),
        ("~~~Rust", true, true, false, "rust", None, 3, true),
        ("~~~ rust", true, true, false, "rust", None, 3, true),
        ("~~~ line_num", true, true, false, "", Some(1), 3, true),
        ("~~~line_num(5)", true, true, false, "", Some(5), 3, true),
        ("~~~rust, line_num(5)", true, true, false, "rust", Some(5), 3, true),
        ("~~~line_num(5), rust", true, true, false, "rust", Some(5), 3, true),
    ];

    samples.into_iter().map(
        |(
            case,
            is_valid,
            is_code_fence_begin,
            is_code_fence_end,
            language,
            line_num,
            fence_size,
            is_tilde_fence
        )| (
            case.to_string(),
            is_valid,
            is_code_fence_begin,
            is_code_fence_end,
            language.to_string(),
            line_num,
            fence_size,
            is_tilde_fence
        )
    ).collect()
}

#[test]
fn fence_test() {

    #[cfg(not(feature = "test-all"))]
    return;

    let mut failures = vec![];
    let test_cases = fence_samples();

    for (
        case,
        is_valid,
        is_code_fence_begin,
        is_code_fence_end,
        language_answer,
        line_num_answer,
        fence_size_answer,
        is_tilde_fence_answer
    ) in test_cases.iter() {
        let v16 = into_v16(&case);
        let line = Line::from_raw(&v16);

        if *is_valid != (line.is_code_fence_begin() || line.is_code_fence_end()) {
            failures.push(format!(
                "case: {}\nis_valid: answer: {}, actual: {}",
                case, is_valid, line.is_code_fence_begin() || line.is_code_fence_end()
            ));
            continue;
        }

        if !is_valid {
            continue;
        }

        if *is_code_fence_begin != line.is_code_fence_begin() || *is_code_fence_end != line.is_code_fence_end() {
            failures.push(format!(
                "case: {}\nis_code_fence_begin: answer: {}, actual: {}\nis_code_fence_end: answer: {}, actual: {}",
                case,
                is_code_fence_begin, line.is_code_fence_begin(),
                is_code_fence_end, line.is_code_fence_end()
            ));
            continue;
        }

        let (
            language_actual,
            line_num_actual,
            fence_size_actual,
            is_tilde_fence_actual
        ) = match read_code_fence_info(&line) {
            ParseState::CodeFence { language, line_num, code_fence_size, is_tilde_fence } => (
                String::from_utf16(&language).unwrap(), line_num, code_fence_size, is_tilde_fence
            ),
            _ => panic!("This doesn't make sense at all."),
        };

        if language_answer != &language_actual {
            failures.push(format!(
                "case: {}\nlanguage: answer: {:?}, actual: {:?}",
                case, language_answer, language_actual
            ));
            continue;
        }

        if line_num_answer != &line_num_actual {
            failures.push(format!(
                "case: {}\nline_num: answer: {:?}, actual: {:?}",
                case, line_num_answer, line_num_actual
            ));
            continue;
        }

        if fence_size_answer != &fence_size_actual {
            failures.push(format!(
                "case: {}\nfence_size: answer: {:?}, actual: {:?}",
                case, fence_size_answer, fence_size_actual
            ));
            continue;
        }

        if is_tilde_fence_answer != &is_tilde_fence_actual {
            failures.push(format!(
                "case: {}\nis_tilde_fence: answer: {:?}, actual: {:?}",
                case, is_tilde_fence_answer, is_tilde_fence_actual
            ));
            continue;
        }

    }

    if failures.len() > 0 {
        panic!(
            "Codefence fence test: {} case(s) out of {} cases have failed!\n\n{}",
            failures.len(),
            test_cases.len(),
            failures.join("\n\n")
        );
    }

}