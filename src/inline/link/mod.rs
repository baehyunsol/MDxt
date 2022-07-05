// `[[abc]](def)` is a macro, not a link!

pub mod predicate;

#[cfg(test)]
mod testbench;

use crate::utils::{collapse_whitespaces, strip_whitespaces, lowercase};

// [  F  OO ] -> [f oo]
pub fn normalize_link(original: &[u16]) -> Vec<u16> {
    strip_whitespaces(&collapse_whitespaces(original)).into_iter().map(lowercase).collect()
}
