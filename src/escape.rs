pub fn escape_htmls(content: &[u32]) -> Vec<u32> {
    content.iter().map(
        // into_v32("&<>\"\'") -> [38, 60, 62, 34, 39]
        |c| if [38, 60, 62, 34, 39].contains(c) {
            *c + HTML_ESCAPE_OFFSET
        }

        else {
            *c
        }
    ).collect()
}

// <special_form> -> c
pub fn undo_html_escapes(content: &[u32]) -> Vec<u32> {
    content.iter().map(
        |c| if HTML_ESCAPE_OFFSET <= *c && *c < BACKSLASH_ESCAPE_OFFSET {
            *c - HTML_ESCAPE_OFFSET
        } else {
            *c
        }
    ).collect()
}

// <special_form> -> &__;
pub fn render_html_escapes(content: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(content.len());

    for c in content.iter() {

        if HTML_ESCAPE_OFFSET <= *c && *c < BACKSLASH_ESCAPE_OFFSET {
            result.push('&' as u32);

            match *c - HTML_ESCAPE_OFFSET {
                62 => {  // >
                    result.push('g' as u32);
                    result.push('t' as u32);
                }
                60 => {  // <
                    result.push('l' as u32);
                    result.push('t' as u32);
                }
                38 => {  // &
                    result.push('a' as u32);
                    result.push('m' as u32);
                    result.push('p' as u32);
                }
                39 => {  // '
                    result.push('a' as u32);
                    result.push('p' as u32);
                    result.push('o' as u32);
                    result.push('s' as u32);
                }
                34 => {  // "
                    result.push('q' as u32);
                    result.push('u' as u32);
                    result.push('o' as u32);
                    result.push('t' as u32);
                }
                n => {
                    result.push('#' as u32);

                    for ch in format!("{n}").chars() {
                        result.push(ch as u32);
                    }

                }
            }

            result.push(';' as u32);
        }

        else {
            result.push(*c);
        }

    }

    result
}

// \c -> <special_form>
pub fn escape_backslashes(content: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        if content[index] != '\\' as u32 {
            result.push(content[index]);
        }

        // content[index] is '\\', but not escaped
        else if index == content.len() - 1 || content[index + 1] == '\n' as u32 {
            result.push('\\' as u32);
        }

        // already escaped by `escape_htmls`
        else if HTML_ESCAPE_OFFSET <= content[index + 1] && content[index + 1] < BACKSLASH_ESCAPE_OFFSET {
            result.push(content[index + 1] - HTML_ESCAPE_OFFSET + BACKSLASH_ESCAPE_OFFSET);
            index += 1;
        }

        else {
            result.push(BACKSLASH_ESCAPE_OFFSET + content[index + 1]);
            index += 1;
        }

        index += 1;
    }

    result
}

// <special_form> -> \c
pub fn undo_backslash_escapes(content: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(content.len());

    for c in content.iter() {

        if BACKSLASH_ESCAPE_OFFSET <= *c && *c < META_CHARACTER_OFFSET {
            result.push('\\' as u32);
            result.push(*c - BACKSLASH_ESCAPE_OFFSET);
        }

        else {
            result.push(*c);
        }

    }

    result
}

// <special_form> -> &#__;
pub fn render_backslash_escapes(content: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(content.len() * 5 / 4);

    for c in content.iter() {

        if BACKSLASH_ESCAPE_OFFSET <= *c && *c < META_CHARACTER_OFFSET {
            result.push('&' as u32);
            result.push('#' as u32);

            for num in format!("{}", *c - BACKSLASH_ESCAPE_OFFSET).chars() {
                result.push(num as u32);
            }

            result.push(';' as u32);
        }

        else {
            result.push(*c);
        }

    }

    result
}

// <special_form> -> c
pub fn render_backslash_escapes_raw(content: &[u32]) -> Vec<u32> {
    content.iter().map(
        |c| if BACKSLASH_ESCAPE_OFFSET <= *c && *c < META_CHARACTER_OFFSET {
            *c - BACKSLASH_ESCAPE_OFFSET
        } else {
            *c
        }
    ).collect()
}

// remove newline characters that are not '\n' (ex: '\r')
// handle characters that do not fit in u32 (ex: `🦈` -> `[[char=129432]]`)
// handle characters that may collide with meta characters
//  -> MDxt uses \0xdfe0 ~ \0xdfff as meta characters
// it also does what `escape_htmls` does
pub fn preprocess(content: &String) -> Vec<u32> {
    let mut result = Vec::with_capacity(content.len() * 5 / 4);

    for c in content.chars() {
        let i = c as u32;

        if i < 55204 {

            if i > 10 && i < 14 {
                continue;
            }

            else if i == 0x200d || i == '>' as u32 || i == '<' as u32
                || i == '&' as u32 || i == '\'' as u32 || i == '"' as u32
            {
                result.push(HTML_ESCAPE_OFFSET + i);
            }

            else {
                result.push(i);
            }

        }

        // characters that do not fit in u16
        else {
            result.push(HTML_ESCAPE_OFFSET + i);
        }

    }

    result
}

/*
 *  0x000_000 ~ 0x1ff_fff: normal characters
 *  0x200_000 ~ 0x3ff_fff: html escapes       // always escaped
 *  0x400_000 ~ 0x5ff_fff: backslash escapes  // not escaped inside code spans
 *  0x600_000 ~ 0x600_fff: meta characters
 */

pub const HTML_ESCAPE_OFFSET: u32 = 0x200_000;
pub const BACKSLASH_ESCAPE_OFFSET: u32 = 0x400_000;
pub const META_CHARACTER_OFFSET: u32 = 0x600_000;

#[cfg(test)]
mod tests {

    #[test]
    fn escape_undo_test() {
        use crate::testbench::random;
        use crate::escape::*;

        for i in 0..256 {
            let test_case: Vec<u32> = (0..512).map(|j| (random(i * 8191 + j * 37) % 128) as u32).collect();
            assert_eq!(undo_html_escapes(&escape_htmls(&test_case)), test_case);
            assert_eq!(undo_backslash_escapes(&escape_backslashes(&test_case)), test_case);
        }

    }

    #[test]
    fn backslash_escape_test() {
        use crate::utils::{into_v32, from_v32};
        use crate::escape::*;

        let input = into_v32("\\a\\\\\\\n\\*\\");
        let output = into_v32("&#97;&#92;\\\n&#42;\\");

        assert_eq!(
            from_v32(&render_backslash_escapes(&escape_backslashes(&input))),
            from_v32(&output),
        );

        assert_eq!(
            from_v32(&render_backslash_escapes_raw(&escape_backslashes(&input))),
            String::from("a\\\\\n*\\"),
        );
    }

}