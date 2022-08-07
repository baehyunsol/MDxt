use super::{ZERO_ARG_FUNCTIONS, ONE_ARG_FUNCTIONS, TWO_ARG_FUNCTIONS, FIVE_ARG_FUNCTIONS};
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {

    // it's only used by `is_valid`
    static ref FUNCTION_NAMES: HashSet<Vec<u16>> = {
        ZERO_ARG_FUNCTIONS
            .union(&ONE_ARG_FUNCTIONS).map(|f| f.clone()).collect::<HashSet<Vec<u16>>>()
            .union(&TWO_ARG_FUNCTIONS).map(|f| f.clone()).collect::<HashSet<Vec<u16>>>()
            .union(&FIVE_ARG_FUNCTIONS).map(|f| f.clone()).collect::<HashSet<Vec<u16>>>()
    };
}

pub fn is_valid(word: &[u16], arguments: &Vec<Vec<u16>>) -> bool {

    FUNCTION_NAMES.contains(word) && (
        (ZERO_ARG_FUNCTIONS.contains(word) && arguments.len() == 0)
        || (ONE_ARG_FUNCTIONS.contains(word) && arguments.len() == 1)
        || (TWO_ARG_FUNCTIONS.contains(word) && arguments.len() == 2)
        || (FIVE_ARG_FUNCTIONS.contains(word) && arguments.len() == 5)
    )

}