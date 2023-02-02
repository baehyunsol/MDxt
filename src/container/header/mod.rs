#[cfg(test)]
mod testbench;

use crate::ast::line::Line;
use crate::utils::{drop_while, lowercase, take_and_drop_while};

pub fn parse_header(line: &Line) -> (usize, Vec<u32>) {  // (level, content)
    let (sharps, sharps_removed) = take_and_drop_while(&line.content, '#' as u32);
    let indents_removed = drop_while(&sharps_removed, ' ' as u32);

    (sharps.len(), indents_removed)
}

pub fn normalize_header(content: &[u32]) -> Vec<u32> {
    content.iter().filter(
        |c| '0' as u32 <= **c && **c <= '9' as u32
        || 'a' as u32 <= **c && **c <= 'z' as u32
        || 'A' as u32 <= **c && **c <= 'Z' as u32
        || '가' as u32 <= **c && **c <= '힣' as u32
        || 'ㄱ' as u32 <= **c && **c <= 'ㅣ' as u32  // Korean
        || 'ぁ' as u32 <= **c && **c <= 'ヺ' as u32  // Japanese
        || '-' as u32 == **c || '_' as u32 == **c || ' ' as u32 == **c
    ).map(
        |c| if *c == ' ' as u32 {
            '-' as u32
        } else {
            lowercase(c)
        }
    ).collect::<Vec<u32>>()
}