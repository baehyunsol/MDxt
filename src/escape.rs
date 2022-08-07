pub use crate::inline::parse::{escape_code_spans, undo_code_span_escapes};
use crate::utils::into_v16;

/*
`<`s are converted to `&lt` and `&gt`, always!
`>`s are kept untouched because they could be part of a blockquote
backslashes are always escaped.
*/
pub fn escape_htmls(content: &[u16]) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len() + content.len() / 4);

    for c in content.iter() {

        match *c {
            38 => {  // &
                result.push('&' as u16);
                result.push('a' as u16);
                result.push('m' as u16);
                result.push('p' as u16);
                result.push(';' as u16);
            },
            60 => {  // <
                result.push('&' as u16);
                result.push('l' as u16);
                result.push('t' as u16);
                result.push(';' as u16);
            },
            62 => {  // >
                result.push('&' as u16);
                result.push('g' as u16);
                result.push('t' as u16);
                result.push(';' as u16);
            },
            34 => {  // "
                result.push('&' as u16);
                result.push('q' as u16);
                result.push('u' as u16);
                result.push('o' as u16);
                result.push('t' as u16);
                result.push(';' as u16);
            },
            39 => {  // '
                result.push('&' as u16);
                result.push('a' as u16);
                result.push('p' as u16);
                result.push('o' as u16);
                result.push('s' as u16);
                result.push(';' as u16);
            },
            _ => {
                result.push(*c);
            }
        }
    }

    result
}

// characters in syntax highlighted texts may not be escaped
pub fn undo_html_escapes(content: &[u16]) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        match is_html_escaped(content, index) {
            None => {
                result.push(content[index]);
            }
            Some((c, i)) => {
                result.push(c);
                index = i;
            }
        }

        index += 1;
    }

    result
}


fn is_html_escaped(content: &[u16], index: usize) -> Option<(u16, usize)> {

    if content[index] == '&' as u16 && index + 3 < content.len() {

        if content[index + 1] == 'a' as u16 {

            if index + 4 < content.len()
                && content[index + 2] == 'm' as u16
                && content[index + 3] == 'p' as u16
                && content[index + 4] == ';' as u16
            {
                return Some(('&' as u16, index + 4));
            }

            if content[index + 2] == 'p' as u16
                && content[index + 3] == 'o' as u16
                && content[index + 4] == 's' as u16
                && index + 5 < content.len()
                && content[index + 5] == ';' as u16
            {
                return Some(('\'' as u16, index + 5));
            }

        }

        else if content[index + 1] == 'l' as u16
            && content[index + 2] == 't' as u16
            && content[index + 3] == ';' as u16
        {
            return Some(('<' as u16, index + 3));
        }

        else if content[index + 1] == 'g' as u16
            && content[index + 2] == 't' as u16
            && content[index + 3] == ';' as u16
        {
            return Some(('>' as u16, index + 3));
        }

        else if content[index + 1] == 'q' as u16
            && content[index + 2] == 'u' as u16
            && content[index + 3] == 'o' as u16
            && content[index + 4] == 't' as u16
            && index + 5 < content.len()
            && content[index + 5] == ';' as u16
        {
            return Some(('"' as u16, index + 5));
        }

    }

    None
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

// it's an illegal unicode, which is appropriate to be used as an internal meta character
pub const BACKSLASH_ESCAPE_MARKER: u16 = 0xd804;

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