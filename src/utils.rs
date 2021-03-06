pub fn into_v16(s: &str) -> Vec<u16> {
    String::from(s).encode_utf16().filter(|c| *c != 13).collect()
}

pub fn from_v16(v: &[u16]) -> String {

    if cfg!(test) {
        String::from_utf16(v).unwrap()
    }

    else {
        String::from_utf16_lossy(v)
    }

}

pub fn drop_while(v: &[u16], c: u16) -> Vec<u16> {

    let mut index = 0;

    while index < v.len() {

        if v[index] != c {
            break;
        }

        index += 1;
    }

    v[index..].to_vec()
}

pub fn take_while(v: &[u16], c: u16) -> Vec<u16> {

    let mut index = 0;

    while index < v.len() {

        if v[index] != c {
            break;
        }

        index += 1;
    }

    v[0..index].to_vec()
}

pub fn take_and_drop_while(v: &[u16], c: u16) -> (Vec<u16>, Vec<u16>) {

    let mut index = 0;

    while index < v.len() {

        if v[index] != c {
            break;
        }

        index += 1;
    }

    (v[0..index].to_vec(), v[index..].to_vec())
}

pub fn get_bracket_end_index(v: &[u16], index: usize) -> Option<usize> {
    get_partner_index(v, index, '[' as u16, ']' as u16)
}

pub fn get_parenthesis_end_index(v: &[u16], index: usize) -> Option<usize> {
    get_partner_index(v, index, '(' as u16, ')' as u16)
}

pub fn get_curly_brace_end_index(v: &[u16], index: usize) -> Option<usize> {
    get_partner_index(v, index, '{' as u16, '}' as u16)
}

fn get_partner_index(v: &[u16], begin_index: usize, s: u16, p: u16) -> Option<usize> {

    let mut stack: i32 = 0;

    for index in begin_index..v.len() {

        if v[index] == s {
            stack += 1;
        }

        else if v[index] == p {
            stack -= 1;

            if stack == 0 {
                return Some(index);
            }

        }

    }

    None
}

pub fn lowercase(c: &u16) -> u16 {

    if 'A' as u16 <= *c && *c <= 'Z' as u16 {
        c + 32
    }

    else {
        *c
    }

}

pub fn is_alphabet(c: u16) -> bool {
    'A' as u16 <= c && c <= 'Z' as u16 || 'a' as u16 <= c && c <= 'z' as u16
}

pub fn is_numeric(c: &u16) -> bool {
    '0' as u16 <= *c && *c <= '9' as u16
}

pub fn collapse_whitespaces(content: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(content.len());
    let mut consecutive_whitespace = false;

    for c in content.iter() {

        if *c == ' ' as u16 {

            if !consecutive_whitespace {
                result.push(' ' as u16);
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

pub fn strip_whitespaces(content: &[u16]) -> Vec<u16> {

    if content.len() == 0 {
        return vec![];
    }

    let mut start_index = 0;

    while start_index < content.len() {

        if content[start_index] != ' ' as u16 {
            break;
        }

        start_index += 1;
    }

    let mut end_index = content.len() - 1;

    while end_index > 0 {

        if content[end_index] != ' ' as u16 {
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

pub fn to_int(string: &[u16]) -> Option<u32> {

    if string.len() == 0 {
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

        if *c < '0' as u16 || *c > '9' as u16 {
            return None;
        }

        result *= 10;
        result += *c as u32 - 48;
    }

    Some(result)
}

pub fn remove_whitespaces(line: &[u16]) -> Vec<u16> {
    line.iter().filter(
        |c| **c != ' ' as u16 && **c != '\n' as u16 && **c != '\t' as u16
    ).map(|c| *c).collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn partner_test() {
        use crate::utils::{get_parenthesis_end_index, get_bracket_end_index, into_v16};
        let test1 = into_v16("[name](link)");

        assert_eq!(get_bracket_end_index(&test1, 0), Some(5));
        assert_eq!(get_bracket_end_index(&test1, 1), None);
        assert_eq!(get_parenthesis_end_index(&test1, 6), Some(11));
        assert_eq!(get_parenthesis_end_index(&test1, 7), None);

        let test2 = into_v16("[[macro]]");

        assert_eq!(get_bracket_end_index(&test2, 0), Some(8));
        assert_eq!(get_bracket_end_index(&test2, 1), Some(7));
        assert_eq!(get_bracket_end_index(&test2, 2), None);
    }

    #[test]
    fn whitespace_test() {
        use crate::utils::{into_v16, collapse_whitespaces, strip_whitespaces};
        let sample1 = into_v16(" F  OO BA R  ");
        let sample2 = into_v16("   A    ");
        let sample3 = into_v16("   ");

        assert_eq!(collapse_whitespaces(&sample1), into_v16(" F OO BA R "));
        assert_eq!(strip_whitespaces(&sample1), into_v16("F  OO BA R"));

        assert_eq!(collapse_whitespaces(&sample2), into_v16(" A "));
        assert_eq!(strip_whitespaces(&sample2), into_v16("A"));

        assert_eq!(collapse_whitespaces(&sample3), into_v16(" "));
        assert_eq!(strip_whitespaces(&sample3), into_v16(""));
    }

    #[test]
    fn whiles_test() {
        use crate::utils::{into_v16, take_while, drop_while, take_and_drop_while};

        let samples = vec![  // (content, char, take, drop)
            ("", ' ', "", ""),
            (" ", ' ', " ", ""),
            ("### Header3", '#', "###", " Header3"),
            ("### Header3", ' ', "", "### Header3"),
        ];

        let samples = samples.iter().map(
            |(content, character, take, drop)|
            (into_v16(content), *character as u16, into_v16(take), into_v16(drop))
        ).collect::<Vec<(Vec<u16>, u16, Vec<u16>, Vec<u16>)>>();

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

}