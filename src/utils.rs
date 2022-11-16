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

pub fn is_alphabet(c: &u16) -> bool {
    'A' as u16 <= *c && *c <= 'Z' as u16 || 'a' as u16 <= *c && *c <= 'z' as u16
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

pub fn inclusive_split(content: &[u16], delim: u16) -> Vec<&[u16]> {

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

    if result.len() == 0 {
        // i want to return `vec![vec![]]` this case, but the borrow checker doesn't let me `result.push(&vec![])`
        result.push(&content[0..0]);
    }

    result
}

pub fn remove_whitespaces(line: &[u16]) -> Vec<u16> {
    line.iter().filter(
        |c| **c != ' ' as u16 && **c != '\n' as u16 && **c != '\t' as u16
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partner_test() {
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

        let samples: Vec<(Vec<u16>, u16)> = samples.into_iter().map(
            |(string, delim)| (into_v16(string), delim as u16)
        ).collect();

        for (string, delim) in samples.into_iter() {
            let splits1: Vec<Vec<u16>> = inclusive_split(&string, delim).into_iter().map(|s| s.to_vec()).collect();
            let mut splits2: Vec<Vec<u16>> = string.split(|c| *c == delim).map(|s| s.to_vec()).collect();

            for i in 0..(splits2.len() - 1) {
                splits2[i].push(delim);
            }

            if string.len() > 0 && delim == string[string.len() - 1] {
                splits2.pop();
            }

            if splits1.len() != splits2.len() {
                panic!(
                    "Assertion error on `splits1.len() == splits2.len()`\nstring: {:?}\nsplits1: {:?}\nsplits2: {:?}",
                    from_v16(&string),
                    splits1.iter().map(|sp| from_v16(sp)).collect::<Vec<String>>(),
                    splits2.iter().map(|sp| from_v16(sp)).collect::<Vec<String>>(),
                );
            };

            for index in 0..splits1.len() {

                if splits1[index] != splits2[index] {
                    panic!(
                        "A failure at `split_test`\nstring: {:?}\nsplits1: {:?}\nsplits2: {:?}",
                        from_v16(&string),
                        splits1.iter().map(|sp| from_v16(sp)).collect::<Vec<String>>(),
                        splits2.iter().map(|sp| from_v16(sp)).collect::<Vec<String>>(),
                    );
                }

            }

        }

    }

}