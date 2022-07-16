pub mod predicate;
pub mod html;
mod syntect;

#[cfg(test)]
mod testbench;

use crate::ast::line::Line;
use crate::ast::parse::ParseState;
use crate::utils::{take_and_drop_while, remove_whitespaces, into_v16, lowercase, to_int};
use crate::escape::{undo_backslash_escapes, undo_html_escapes};
use predicate::{is_line_num, is_highlight, is_copy_button, parse_arguments};

#[derive(Clone)]
pub struct FencedCode {
    language: Vec<u16>,
    content: Vec<u16>,
    line_num: Option<usize>,
    pub copy_button: bool,
    highlights: Vec<usize>,
    pub index: usize,
}

impl FencedCode {

    pub fn new(content: Vec<u16>, language: Vec<u16>, line_num: Option<usize>, highlights: Vec<usize>, copy_button: bool, index: usize) -> Self {
        FencedCode {
            language,
            content: undo_backslash_escapes(&content),
            line_num,
            copy_button,
            highlights,
            index
        }
    }

    pub fn get_raw_content(&self) -> Vec<u16> {
        undo_html_escapes(&self.content)
    }

}

// it assumes that the given line is a valid code fence
pub fn read_code_fence_info(line: &Line, fenced_code_count: usize) -> ParseState {
    let (fence, mut info_string) = take_and_drop_while(&line.content, line.content[0]);
    info_string = remove_whitespaces(&info_string).iter().map(lowercase).collect();

    let mut language = into_v16("");
    let mut line_num = None;
    let mut highlights = vec![];
    let mut copy_button = None;

    let arguments = parse_arguments(&info_string);

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
            highlights = parse_arguments(&argument[10..argument.len() - 1]).iter().filter_map(
                |arg| to_int(arg)
            ).map(
                |n| n as usize
            ).collect();
        }

        else if is_copy_button(argument) {

            // copy_button || copy_button(true)
            if argument.len() == 11 || argument.len() == 17 {
                copy_button = Some(true);
            }

            else {
                copy_button = Some(false);
            }

        }

        else {
            language = argument.to_vec();
        }

    }

    // if the `copy_button` argument is not given, but `line_num` is enabled, `copy_button` is also enabled.
    // otherwise, it's default to disabled
    let copy_button = match copy_button {
        Some(b) => b,
        None => line_num.is_some()
    };

    ParseState::CodeFence {
        language,
        line_num,
        highlights,
        copy_button,
        code_fence_size: fence.len(),
        is_tilde_fence: line.content[0] == '~' as u16,
        index: fenced_code_count
    }
}