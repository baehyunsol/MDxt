use crate::inline::parse::{is_code_span_marker_begin, get_code_span_marker_end_index};

/*
 * (' ', 32)
 * ('!', 33)
 * ('"', 34)
 * ('#', 35)
 * ('$', 36)
 * ('%', 37)
 * ('&', 38)
 * ("'", 39)
 * ('(', 40)
 * (')', 41)
 * ('*', 42)
 * ('+', 43)
 * (',', 44)
 * ('-', 45)
 * ('.', 46)
 * ('/', 47)
 * ('0', 48)
 * ('1', 49)
 * ('2', 50)
 * ('3', 51)
 * ('4', 52)
 * ('5', 53)
 * ('6', 54)
 * ('7', 55)
 * ('8', 56)
 * ('9', 57)
 * (':', 58)
 * (';', 59)
 * ('<', 60)
 * ('=', 61)
 * ('>', 62)
 * ('?', 63)
 * ('@', 64)
 * ('A', 65)
 * ('B', 66)
 * ('C', 67)
 * ('D', 68)
 * ('E', 69)
 * ('F', 70)
 * ('G', 71)
 * ('H', 72)
 * ('I', 73)
 * ('J', 74)
 * ('K', 75)
 * ('L', 76)
 * ('M', 77)
 * ('N', 78)
 * ('O', 79)
 * ('P', 80)
 * ('Q', 81)
 * ('R', 82)
 * ('S', 83)
 * ('T', 84)
 * ('U', 85)
 * ('V', 86)
 * ('W', 87)
 * ('X', 88)
 * ('Y', 89)
 * ('Z', 90)
 * ('[', 91)
 * ('\\', 92)
 * (']', 93)
 * ('^', 94)
 * ('_', 95)
 * ('`', 96)
 * ('a', 97)
 * ('b', 98)
 * ('c', 99)
 * ('d', 100)
 * ('e', 101)
 * ('f', 102)
 * ('g', 103)
 * ('h', 104)
 * ('i', 105)
 * ('j', 106)
 * ('k', 107)
 * ('l', 108)
 * ('m', 109)
 * ('n', 110)
 * ('o', 111)
 * ('p', 112)
 * ('q', 113)
 * ('r', 114)
 * ('s', 115)
 * ('t', 116)
 * ('u', 117)
 * ('v', 118)
 * ('w', 119)
 * ('x', 120)
 * ('y', 121)
 * ('z', 122)
 * ('{', 123)
 * ('|', 124)
 * ('}', 125)
 * ('~', 126)
 */

#[inline]
pub fn into_v32(s: &str) -> Vec<u32> {
    s.chars().map(|c| c as u32).collect()
}

#[inline]
pub fn from_v32(v: &[u32]) -> String {
    v.iter().map(|n| char::from_u32(*n).expect(&format!("{n:#x} is not a valid char-point"))).collect()
}

pub fn drop_while(v: &[u32], c: u32) -> Vec<u32> {

    let mut index = 0;

    while index < v.len() {

        if v[index] != c {
            break;
        }

        index += 1;
    }

    v[index..].to_vec()
}

#[cfg(test)]  // for now, it's only used by test functions
pub fn take_while(v: &[u32], c: u32) -> Vec<u32> {

    let mut index = 0;

    while index < v.len() {

        if v[index] != c {
            break;
        }

        index += 1;
    }

    v[0..index].to_vec()
}

pub fn take_and_drop_while(v: &[u32], c: u32) -> (Vec<u32>, Vec<u32>) {

    let mut index = 0;

    while index < v.len() {

        if v[index] != c {
            break;
        }

        index += 1;
    }

    (v[0..index].to_vec(), v[index..].to_vec())
}

pub fn get_bracket_end_index(v: &[u32], index: usize) -> Option<usize> {
    get_partner_index(v, index, '[' as u32, ']' as u32)
}

pub fn get_parenthesis_end_index(v: &[u32], index: usize) -> Option<usize> {
    get_partner_index(v, index, '(' as u32, ')' as u32)
}

pub fn get_curly_brace_end_index(v: &[u32], index: usize) -> Option<usize> {
    get_partner_index(v, index, '{' as u32, '}' as u32)
}

fn get_partner_index(v: &[u32], begin_index: usize, s: u32, p: u32) -> Option<usize> {
    let mut stack: i32 = 0;
    let mut index = begin_index;

    while index < v.len() {

        if v[index] == s {
            stack += 1;
        }

        // ignores brackets inside code spans
        else if is_code_span_marker_begin(v, index) {
            index = get_code_span_marker_end_index(v, index);
        }

        else if v[index] == p {
            stack -= 1;

            if stack == 0 {
                return Some(index);
            }

        }

        index += 1;
    }

    None
}

#[inline]
pub fn lowercase(c: &u32) -> u32 {

    if 'A' as u32 <= *c && *c <= 'Z' as u32 {
        c + 32
    }

    else {
        *c
    }

}

pub fn is_alphabet(c: &u32) -> bool {
    'A' as u32 <= *c && *c <= 'Z' as u32 || 'a' as u32 <= *c && *c <= 'z' as u32
}

pub fn is_numeric(c: &u32) -> bool {
    '0' as u32 <= *c && *c <= '9' as u32
}

pub fn is_alpha_numeric(c: &u32) -> bool {
    is_alphabet(c) || is_numeric(c)
}

pub fn collapse_whitespaces(content: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(content.len());
    let mut consecutive_whitespace = false;

    for c in content.iter() {

        if *c == ' ' as u32 {

            if !consecutive_whitespace {
                result.push(' ' as u32);
                consecutive_whitespace = true;
            }

        }

        else {
            result.push(*c);
            consecutive_whitespace = false;
        }

    }

    result
}

pub fn strip_whitespaces(content: &[u32]) -> Vec<u32> {

    if content.is_empty() {
        return vec![];
    }

    let mut start_index = 0;

    while start_index < content.len() {

        if content[start_index] != ' ' as u32 {
            break;
        }

        start_index += 1;
    }

    let mut end_index = content.len() - 1;

    while end_index > 0 {

        if content[end_index] != ' ' as u32 {
            break;
        }

        end_index -= 1;
    }

    if start_index < end_index + 1 {
        content[start_index..end_index + 1].to_vec()
    }

    else {
        vec![]
    }

}

pub fn to_int(string: &[u32]) -> Option<u32> {

    if string.is_empty() {
        return None;
    }

    let mut result = 0;
    let size_limit = u32::MAX / 10 - 1;

    for c in string.iter() {

        if result > size_limit {
            // instead of raising an error, it returns None
            // that's because rendering a markdown document should not fail!
            return None;
        }

        if *c < '0' as u32 || *c > '9' as u32 {
            return None;
        }

        result *= 10;
        result += *c - 48;
    }

    Some(result)
}

pub fn inclusive_split(content: &[u32], delim: u32) -> Vec<&[u32]> {

    let mut last_index = 0;
    let mut result = vec![];

    for (ind, c) in content.iter().enumerate() {

        if *c == delim {
            result.push(&content[last_index..(ind + 1)]);
            last_index = ind + 1;
        }

    }

    if last_index < content.len() {
        result.push(&content[last_index..]);
    }

    if result.is_empty() {
        // i want to return `vec![vec![]]` this case, but the borrow checker doesn't let me `result.push(&vec![])`
        result.push(&content[0..0]);
    }

    result
}

pub fn remove_whitespaces(line: &[u32]) -> Vec<u32> {
    line.iter().filter(
        |c| **c != ' ' as u32 && **c != '\n' as u32 && **c != '\t' as u32
    ).map(|c| *c).collect()
}

pub fn log10(n: usize) -> usize {

    if n < 10 {
        0
    }

    else if n < 100 {
        1
    }

    else if n < 1_000 {
        2
    }

    else {
        log10(n / 1_000) + 3
    }

}

pub fn add_styles_to_html(html: &str) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open("./styles/markdown.css").unwrap();
    let mut css = String::new();
    f.read_to_string(&mut css).unwrap();

    format!(
"
<!DOCTYPE html>
<html>
<head>
    <title>MDxt Reference</title>
    <style>{css}</style>
</head>
<body style=\"padding-left: 36px\">
    <article class=\"markdown\">{html}</article>
</body>
</html>
",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partner_test() {
        let test1 = into_v32("[name](link)");

        assert_eq!(get_bracket_end_index(&test1, 0), Some(5));
        assert_eq!(get_bracket_end_index(&test1, 1), None);
        assert_eq!(get_parenthesis_end_index(&test1, 6), Some(11));
        assert_eq!(get_parenthesis_end_index(&test1, 7), None);

        let test2 = into_v32("[[macro]]");

        assert_eq!(get_bracket_end_index(&test2, 0), Some(8));
        assert_eq!(get_bracket_end_index(&test2, 1), Some(7));
        assert_eq!(get_bracket_end_index(&test2, 2), None);
    }

    #[test]
    fn whitespace_test() {
        let sample1 = into_v32(" F  OO BA R  ");
        let sample2 = into_v32("   A    ");
        let sample3 = into_v32("   ");

        assert_eq!(collapse_whitespaces(&sample1), into_v32(" F OO BA R "));
        assert_eq!(strip_whitespaces(&sample1), into_v32("F  OO BA R"));

        assert_eq!(collapse_whitespaces(&sample2), into_v32(" A "));
        assert_eq!(strip_whitespaces(&sample2), into_v32("A"));

        assert_eq!(collapse_whitespaces(&sample3), into_v32(" "));
        assert_eq!(strip_whitespaces(&sample3), into_v32(""));
    }

    #[test]
    fn whiles_test() {
        let samples = vec![  // (content, char, take, drop)
            ("", ' ', "", ""),
            (" ", ' ', " ", ""),
            ("### Header3", '#', "###", " Header3"),
            ("### Header3", ' ', "", "### Header3"),
        ];

        let samples = samples.iter().map(
            |(content, character, take, drop)|
            (into_v32(content), *character as u32, into_v32(take), into_v32(drop))
        ).collect::<Vec<(Vec<u32>, u32, Vec<u32>, Vec<u32>)>>();

        for (sample, character, taken_answer, dropped_answer) in samples.iter() {
            let taken_actual = take_while(sample, *character);
            assert_eq!(&taken_actual, taken_answer);

            let dropped_actual = drop_while(sample, *character);
            assert_eq!(&dropped_actual, dropped_answer);

            let (taken_actual, dropped_actual) = take_and_drop_while(sample, *character);
            assert_eq!(&taken_actual, taken_answer);
            assert_eq!(&dropped_actual, dropped_answer);
        }
    }

    #[test]
    fn split_test() {
        let samples = vec![
            ("100100", '1'),
            ("100100", '0'),
            ("01001100101011000101011", '1'),
            ("01001100101011000101011", '0'),
            ("", '1'),
            ("1", '1'),
            ("11", '1'),
            ("11", '2'),
        ];

        let samples: Vec<(Vec<u32>, u32)> = samples.into_iter().map(
            |(string, delim)| (into_v32(string), delim as u32)
        ).collect();

        for (string, delim) in samples.into_iter() {
            let splits1: Vec<Vec<u32>> = inclusive_split(&string, delim).into_iter().map(|s| s.to_vec()).collect();
            let mut splits2: Vec<Vec<u32>> = string.split(|c| *c == delim).map(|s| s.to_vec()).collect();

            for i in 0..(splits2.len() - 1) {
                splits2[i].push(delim);
            }

            if string.last() == Some(&delim) {
                splits2.pop();
            }

            if splits1.len() != splits2.len() {
                panic!(
                    "Assertion error on `splits1.len() == splits2.len()`\nstring: {:?}\nsplits1: {:?}\nsplits2: {:?}",
                    from_v32(&string),
                    splits1.iter().map(|sp| from_v32(sp)).collect::<Vec<String>>(),
                    splits2.iter().map(|sp| from_v32(sp)).collect::<Vec<String>>(),
                );
            };

            for index in 0..splits1.len() {

                if splits1[index] != splits2[index] {
                    panic!(
                        "A failure at `split_test`\nstring: {:?}\nsplits1: {:?}\nsplits2: {:?}",
                        from_v32(&string),
                        splits1.iter().map(|sp| from_v32(sp)).collect::<Vec<String>>(),
                        splits2.iter().map(|sp| from_v32(sp)).collect::<Vec<String>>(),
                    );
                }

            }

        }

    }

}