pub use crate::inline::parse::{escape_code_spans, undo_code_span_escapes};
use crate::utils::into_v16;

pub fn escape_htmls(content: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(content.len() + content.len() / 4);

    for c in content.iter() {

        if into_v16("&<>\"\'").contains(c) {
            result.push(HTML_ESCAPE_MARKER);
            result.push(u16::MAX - *c);
        }

        else {
            result.push(*c);
        }

    }

    result
}

// <special_form> -> c
pub fn undo_html_escapes(content: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        if content[index] != HTML_ESCAPE_MARKER {
            result.push(content[index]);
        }

        else {
            result.push(u16::MAX - content[index + 1]);
            index += 1;
        }

        index += 1;
    }

    result
}

// <special_form> -> &__;
pub fn render_html_escapes(content: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        if content[index] != HTML_ESCAPE_MARKER {
            result.push(content[index]);
        }

        else {
            let escaped_char = u16::MAX - content[index + 1];
            result.push('&' as u16);

            match escaped_char {
                62 => {  // >
                    result.push('g' as u16);
                    result.push('t' as u16);
                }
                60 => {  // <
                    result.push('l' as u16);
                    result.push('t' as u16);
                }
                38 => {  // &
                    result.push('a' as u16);
                    result.push('m' as u16);
                    result.push('p' as u16);
                }
                39 => {  // '
                    result.push('a' as u16);
                    result.push('p' as u16);
                    result.push('o' as u16);
                    result.push('s' as u16);
                }
                34 => {  // "
                    result.push('q' as u16);
                    result.push('u' as u16);
                    result.push('o' as u16);
                    result.push('t' as u16);
                }
                _ => {
                    panic!("unexpected char: {}", escaped_char);
                }
            }

            result.push(';' as u16);
            index += 1;
        }

        index += 1;
    }

    result
}

// \c -> <special_form>
pub fn escape_backslashes(content: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        if content[index] != '\\' as u16 {
            result.push(content[index]);
        }

        // content[index] is '\\', but not escaped
        else if index == content.len() - 1 || content[index + 1] == '\n' as u16 {
            result.push('\\' as u16);
        }

        else {
            result.push(BACKSLASH_ESCAPE_MARKER);
            result.push(u16::MAX - content[index + 1]);
            index += 1;
        }

        index += 1;
    }

    result
}

// <special_form> -> \c
pub fn undo_backslash_escapes(content: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        if content[index] != BACKSLASH_ESCAPE_MARKER {
            result.push(content[index]);
        }

        else {
            result.push('\\' as u16);
            result.push(u16::MAX - content[index + 1]);
            index += 1;
        }

        index += 1;
    }

    result
}

// <special_form> -> &#__;
pub fn render_backslash_escapes(content: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(content.len() * 5 / 4);
    let mut index = 0;

    while index < content.len() {

        if content[index] != BACKSLASH_ESCAPE_MARKER {
            result.push(content[index]);
        }

        else {
            result.push('&' as u16);
            result.push('#' as u16);

            for num in into_v16(&(u16::MAX - content[index + 1]).to_string()) {
                result.push(num);
            }

            result.push(';' as u16);
            index += 1;
        }

        index += 1;
    }

    result
}

// <special_form> -> c
pub fn render_backslash_escapes_raw(content: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        if content[index] != BACKSLASH_ESCAPE_MARKER {
            result.push(content[index]);
        }

        else {
            result.push(u16::MAX - content[index + 1]);
            index += 1;
        }

        index += 1;
    }

    result
}

pub fn remove_invalid_characters(content: &[u16]) -> Vec<u16> {
    content.iter().filter(
        |c| **c < 11 || **c > 13  // only `\n`, no other newline characters!
    ).map(
        |c| if 0xd7ff < *c && *c < 0xe000 {  // the engine uses these characters as meta characters!
            '?' as u16
        } else {
            *c
        }
    ).collect()
}

// these are illegal unicodes, which are appropriate to be used as internal meta characters
pub const BACKSLASH_ESCAPE_MARKER: u16 = 0xd804;
pub const HTML_ESCAPE_MARKER: u16 = 0xd805;

#[cfg(test)]
mod tests {

    #[test]
    fn escape_undo_test() {
        use crate::testbench::random;
        use crate::escape::*;

        for i in 0..256 {
            let test_case: Vec<u16> = (0..512).map(|j| (random(i * 8191 + j * 37) % 128) as u16).collect();
            assert_eq!(undo_html_escapes(&escape_htmls(&test_case)), test_case);
            assert_eq!(undo_backslash_escapes(&escape_backslashes(&test_case)), test_case);
        }

    }

    #[test]
    fn backslash_escape_test() {
        use crate::utils::{into_v16, from_v16};
        use crate::escape::*;

        let input = into_v16("\\a\\\\\\\n\\*\\");
        let output = into_v16("&#97;&#92;\\\n&#42;\\");

        assert_eq!(
            from_v16(&render_backslash_escapes(&escape_backslashes(&input))),
            from_v16(&output),
        );

        assert_eq!(
            from_v16(&render_backslash_escapes_raw(&escape_backslashes(&input))),
            String::from("a\\\\\n*\\"),
        );
    }

}