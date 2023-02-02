// `[[abc]](def)` is a macro, not a link!

pub mod predicate;

#[cfg(test)]
mod testbench;

use crate::utils::{collapse_whitespaces, lowercase, strip_whitespaces};

// [  F  OO ] -> [f oo]
pub fn normalize_link_label(original: &[u32]) -> Vec<u32> {
    strip_whitespaces(&collapse_whitespaces(original)).iter().map(lowercase).collect()
}
