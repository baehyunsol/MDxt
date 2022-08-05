use super::{ZERO_ARG_FUNCTIONS, ONE_ARG_FUNCTIONS, TWO_ARG_FUNCTIONS};

pub fn is_valid(word: &[u16], arguments: &Vec<Vec<u16>>) -> bool {

    if ZERO_ARG_FUNCTIONS.contains(word) {
        arguments.len() == 0
    }

    else if ONE_ARG_FUNCTIONS.contains(word) {
        arguments.len() == 1
    }

    else if TWO_ARG_FUNCTIONS.contains(word) {
        arguments.len() == 2
    }

    else {
        false
    }

}