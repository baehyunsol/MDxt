// `[[abc]](def)` is a macro, not a link!

pub mod predicate;

#[cfg(test)]
mod testbench;

use crate::utils::{collapse_whitespaces, lowercase, strip_whitespaces, is_alphabet, is_numeric};

// [  F  OO ] -> [f oo]
pub fn normalize_link_label(original: &[u32]) -> Vec<u32> {
    strip_whitespaces(&collapse_whitespaces(original)).iter().map(lowercase).collect()
}

// see [reference](reference.html#multimedia-types)
pub fn is_youtube(v: &[u32]) -> bool {

    if v.len() > 12 {
        let mut index = v.len() - 1;

        while index > 0 && is_numeric(&v[index]) {
            index -= 1;
        }

        index > 10 && v[index] == '=' as u32 && v[index - 1] == 't' as u32 && v[index - 2] == '?' as u32 && is_youtube(&v[0..(index - 2)])
    }

    else if v.len() > 9 {
        v.iter().all(
            |c| is_alphabet(c) || is_numeric(c) || *c == '_' as u32 || *c == '-' as u32
        )
    }

    else {
        false
    }

}