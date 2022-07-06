pub mod predicate;
pub mod html;

#[cfg(test)]
mod testbench;

use crate::ast::line::Line;
use crate::utils::{take_and_drop_while, remove_whitespaces, into_v16, lowercase, to_int};
use crate::ast::parse::ParseState;
use predicate::{is_line_num, is_highlight};

pub use html::to_html;

pub struct FencedCode {
    language: Vec<u16>,
    content: Vec<u16>,
    line_num: Option<usize>,
    highlights: Vec<usize>
}

impl FencedCode {

    pub fn new(content: Vec<u16>, language: Vec<u16>, line_num: Option<usize>, highlights: Vec<usize>) -> Self {
        FencedCode { language, content, line_num, highlights }
    }

    pub fn to_html(&self) -> Vec<u16> {
        todo!()
    }

}

// it assumes that the given line is a valid code fence
pub fn read_code_fence_info(line: &Line) -> ParseState {
    let (fence, mut info_string) = take_and_drop_while(&line.content, line.content[0]);
    info_string = remove_whitespaces(&info_string).iter().map(lowercase).collect();

    let mut language = into_v16("");
    let mut line_num = None;
    let mut highlights = vec![];

    let arguments = info_string.split(|c| *c == ',' as u16).collect::<Vec<&[u16]>>();

    for argument in arguments.iter() {

        if is_line_num(argument) {

            if argument.len() == 8 {
                line_num = Some(1);
            }

            else {
                line_num = Some(to_int(&argument[9..argument.len() - 1]).unwrap() as usize);
            }

        }

        else if is_highlight(argument) {
            todo!();
        }

        else {
            language = argument.to_vec();
        }

    }

    ParseState::CodeFence {
        language,
        line_num,
        highlights,
        code_fence_size: fence.len(),
        is_tilde_fence: line.content[0] == '~' as u16
    }
}