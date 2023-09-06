use crate::escape::HTML_ESCAPE_OFFSET;
use crate::inline::{INLINE_CODE_SPAN_MARKER1, INLINE_CODE_SPAN_MARKER4};
use crate::utils::is_alpha_numeric;

#[derive(Debug)]
enum ParseState {
    ReadParagraph,
    ReadURL,
    SkipUntil(u32),
}

pub enum UrlOrNot {
    NoUrl(Vec<u32>),

    // (prefix, url, suffix)
    HasUrl(Vec<u32>, Vec<u32>, Vec<u32>),
}

pub fn render_auto_urls(content: &[u32]) -> UrlOrNot {
    let mut last_char = '\n' as u32;
    let mut curr_state = ParseState::ReadParagraph;
    let mut url_buffer = vec![];
    let mut para_buffer = vec![];

    // it adds a dummy character in order to empty `url_buffer` when the iteration finishes
    for (ind, c) in content.iter().chain([32].iter()).enumerate() {
        match &curr_state {
            ParseState::ReadParagraph => {
                if is_alpha_numeric(c) && (
                    last_char == '\n' as u32
                    || last_char == ' ' as u32
                    || last_char == '(' as u32
                ) {
                    curr_state = ParseState::ReadURL;
                    url_buffer.push(*c);
                } else if *c == INLINE_CODE_SPAN_MARKER1 {
                    curr_state = ParseState::SkipUntil(INLINE_CODE_SPAN_MARKER4);
                    para_buffer.push(*c);
                } else {
                    last_char = *c;
                    para_buffer.push(*c);
                }
            },
            ParseState::ReadURL => {
                if is_allowed_char(c) {
                    url_buffer.push(*c);
                } else if *c == ' ' as u32 || *c == '<' as u32 + HTML_ESCAPE_OFFSET
                || (44032 <= *c && *c < 55024) {
                    if contains_url_substr(&url_buffer) {  // is a valid url
                        if has_to_pop_last_char(&url_buffer, last_char == '(' as u32) {
                            return UrlOrNot::HasUrl(
                                para_buffer,
                                url_buffer[..(url_buffer.len() - 1)].to_vec(),
                                if ind - 1 < content.len() {
                                    content[(ind - 1)..].to_vec()
                                } else {
                                    vec![]
                                },
                            );
                        } else {
                            return UrlOrNot::HasUrl(
                                para_buffer,
                                url_buffer,
                                if ind < content.len() {
                                    content[ind..].to_vec()
                                } else {
                                    vec![]
                                },
                            );
                        }
                    } else {
                        para_buffer = vec![
                            para_buffer,
                            url_buffer,
                        ].concat();

                        url_buffer = vec![];
                        para_buffer.push(*c);

                        curr_state = ParseState::ReadParagraph;
                    }
                } else {
                    para_buffer = vec![
                        para_buffer,
                        url_buffer,
                    ].concat();

                    url_buffer = vec![];
                    para_buffer.push(*c);

                    curr_state = ParseState::ReadParagraph;
                }
            },
            ParseState::SkipUntil(s) => {
                para_buffer.push(*c);

                if c == s {
                    curr_state = ParseState::ReadParagraph;
                }
            },
        }
    }

    // we added a dummy character for simpler code
    para_buffer.pop().unwrap();

    UrlOrNot::NoUrl(para_buffer)
}

enum UrlSubStrState {
    Init,
    ReadW(usize),
    ReadDot,
    Expect(Vec<u32>),
}

// returns true if `url` contains `www.`, `.co`, `.or`, `.io`, `.net` or `.wiki`, or starts with `https://` or `http://`
fn contains_url_substr(url: &[u32]) -> bool {
    if url.get(0) == Some(&('h' as u32))
    && url.get(1) == Some(&('t' as u32))
    && url.get(2) == Some(&('t' as u32))
    && url.get(3) == Some(&('p' as u32)) {
        let curr_ind = if url.get(4) == Some(&('s' as u32)) { 5 } else { 4 };

        if url.get(curr_ind) == Some(&(':' as u32))
        && url.get(curr_ind + 1) == Some(&('/' as u32))
        && url.get(curr_ind + 2) == Some(&('/' as u32)) {
            return true;
        }
    }

    let mut curr_state = UrlSubStrState::Init;

    for c in url.iter() {
        match &mut curr_state {
            UrlSubStrState::Init => {
                if *c == 'w' as u32 {
                    curr_state = UrlSubStrState::ReadW(2);
                } else if *c == '.' as u32 {
                    curr_state = UrlSubStrState::ReadDot;
                }
            },
            UrlSubStrState::ReadW(0) => {
                if *c == '.' as u32 {
                    return true;
                }

                else if *c == 'w' as u32 {
                    // doesn't have to modify
                    // curr_state = UrlSubStrState::ReadW(0);
                }

                else {
                    curr_state = UrlSubStrState::Init;
                }
            }
            UrlSubStrState::ReadW(n) => {
                if *c == 'w' as u32 {
                    *n -= 1;
                } else if *c == '.' as u32 {
                    curr_state = UrlSubStrState::ReadDot;
                } else {
                    curr_state = UrlSubStrState::Init;
                }
            }
            UrlSubStrState::ReadDot => {
                if *c == 'c' as u32 || *c == 'i'  as u32 {
                    curr_state = UrlSubStrState::Expect(vec!['o' as u32]);
                } else if *c == 'o' as u32 {
                    curr_state = UrlSubStrState::Expect(vec!['r' as u32]);
                } else if *c == 'n' as u32 {
                    curr_state = UrlSubStrState::Expect(vec!['e' as u32, 't' as u32]);
                } else if *c == 'w' as u32 {  // let's not allow `.www.` :)
                    curr_state = UrlSubStrState::Expect(vec!['i' as u32, 'k' as u32, 'i' as u32]);
                }
            },
            UrlSubStrState::Expect(v) => {
                if *c == v[0] {
                    *v = v[1..].to_vec();

                    if v.is_empty() {
                        return true;
                    }
                } else if *c == '.' as u32 {
                    curr_state = UrlSubStrState::ReadDot;
                } else {
                    curr_state = UrlSubStrState::Init;
                }
            }
        }
    }

    false
}

fn is_allowed_char(c: &u32) -> bool {

    is_alpha_numeric(c)

    // -/#+&=()?!_.,;:*~
    || [45, 47, 35, 43, 38, 61, 40, 41, 63, 33, 95, 46, 44, 59, 58, 42, 126].contains(c)
}

fn has_to_pop_last_char(url: &[u32], inside_paren: bool) -> bool {
    let last_char = url.last().unwrap();

    [63, 33, 95, 46, 44, 58, 42, 126].contains(last_char)
    || *last_char == ')' as u32 && inside_paren
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{into_v32, from_v32};

    #[test]
    fn url_substr_test() {
        let samples = vec![
            ("", false),
            ("https", false),
            ("ww.", false),
            ("www.github.com", true),
            ("a.b", false),
        ];

        for (url, result) in samples.into_iter() {
            assert_eq!(contains_url_substr(&into_v32(url)), result);
        }
    }

    #[test]
    fn auto_url_test() {
        let samples = vec![
            ("a ", "www.github.com", " b"),
            ("(", "www.github.com", ")"),
            ("(", "www.github.com/(abc)/def", ")"),
            ("", "https://github.com)", "  !"),
            ("The url is ", "www.github.com", "."),
            ("The url is ", "baehyunsol.github.io", ", try it!"),
            ("[ ", "https://github.com", " ]"),
        ];

        let samples: Vec<(Vec<u32>, Vec<u32>, Vec<u32>)> = samples.iter().map(
            |(p, u, s)| (into_v32(p), into_v32(u), into_v32(s))
        ).collect();

        for (prefix, url, suffix) in samples.into_iter() {
            match render_auto_urls(&vec![prefix.clone(), url.clone(), suffix.clone()].concat()) {
                UrlOrNot::HasUrl(p, u, s) if p != prefix || u != url || s != suffix => panic!(
                    "expected: ({:?}, {:?}, {:?})\ngot: ({:?}, {:?}, {:?})",
                    from_v32(&prefix),
                    from_v32(&url),
                    from_v32(&suffix),
                    from_v32(&p),
                    from_v32(&u),
                    from_v32(&s),
                ),
                UrlOrNot::NoUrl(c) => panic!(
                    "expected: ({:?}, {:?}, {:?})\ngot: {:?}",
                    from_v32(&prefix),
                    from_v32(&url),
                    from_v32(&suffix),
                    from_v32(&c),
                ),
                _ => { /* Ok */ },
            }
        }

        let no_urls = vec![
            "a.b.c",
            "baehyunsol.",
            "[https://github.com]",
        ];

        let no_urls: Vec<Vec<u32>> = no_urls.iter().map(
            |c| into_v32(c)
        ).collect();

        for content in no_urls.into_iter() {
            match render_auto_urls(&content) {
                UrlOrNot::NoUrl(c) if c == content => { /* Ok */ },
                UrlOrNot::NoUrl(c) => panic!(
                    "original: {:?}\nprocessed: {:?}",
                    from_v32(&content),
                    from_v32(&c),
                ),
                UrlOrNot::HasUrl(p, u, s) => panic!(
                    "original: {:?}\n({:?}, {:?}, {:?})",
                    from_v32(&content),
                    from_v32(&p),
                    from_v32(&u),
                    from_v32(&s),
                ),
            }
        }
    }
}
