#[cfg(test)]
mod testbench;

use crate::ast::line::Line;
use crate::utils::{drop_while, take_and_drop_while, lowercase};

pub fn parse_header(line: &Line) -> (usize, Vec<u16>) {  // (level, content)
    let (sharps, sharps_removed) = take_and_drop_while(&line.content, '#' as u16);
    let indents_removed = drop_while(&sharps_removed, ' ' as u16);

    (sharps.len(), indents_removed)
}

pub fn normalize_header(content: &[u16]) -> Vec<u16> {
    content.iter().filter(
        |c| '0' as u16 <= **c && **c <= '9' as u16 ||
        'a' as u16 <= **c && **c <= 'z' as u16 ||
        'A' as u16 <= **c && **c <= 'Z' as u16 ||
        '가' as u16 <= **c && **c <= '힣' as u16 ||  // korean
        'ㄱ' as u16 <= **c && **c <= 'ㅣ' as u16 ||  // korean
        'ぁ' as u16 <= **c && **c <= 'ヺ' as u16 || // japanese
        '-' as u16 == **c || '_' as u16 == **c || ' ' as u16 == **c
    ).map(
        |c| if *c == ' ' as u16 {
            '-' as u16
        } else {
            lowercase(c)
        }
    ).collect::<Vec<u16>>()
}