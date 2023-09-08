use super::{ZERO_ARG_FUNCTIONS, ONE_ARG_FUNCTIONS, TWO_ARG_FUNCTIONS, THREE_ARG_FUNCTIONS, FIVE_ARG_FUNCTIONS, parse::is_space};
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {

    // it's only used by `is_valid`
    static ref FUNCTION_NAMES: HashSet<Vec<u32>> = {
        let mut result = ZERO_ARG_FUNCTIONS
            .union(&ONE_ARG_FUNCTIONS).map(|f| f.clone()).collect::<HashSet<Vec<u32>>>()
            .union(&TWO_ARG_FUNCTIONS).map(|f| f.clone()).collect::<HashSet<Vec<u32>>>()
            .union(&THREE_ARG_FUNCTIONS).map(|f| f.clone()).collect::<HashSet<Vec<u32>>>()
            .union(&FIVE_ARG_FUNCTIONS).map(|f| f.clone()).collect::<HashSet<Vec<u32>>>();

        result.insert(vec![109, 97, 116]);  // "mat"

        result
    };
}

pub fn is_valid(word: &[u32], arguments: &Vec<Vec<u32>>) -> bool {

    is_space(word)
    || FUNCTION_NAMES.contains(word) && (
        (ZERO_ARG_FUNCTIONS.contains(word) && arguments.len() == 0)
        || (ONE_ARG_FUNCTIONS.contains(word) && arguments.len() == 1)
        || (TWO_ARG_FUNCTIONS.contains(word) && arguments.len() == 2)
        || (THREE_ARG_FUNCTIONS.contains(word) && arguments.len() == 3)
        || (FIVE_ARG_FUNCTIONS.contains(word) && arguments.len() == 5)
        || word == &[109, 97, 116]  // "mat"
    )

}