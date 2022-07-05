pub mod predicate;

use crate::ast::line::Line;
use crate::ast::parse::ParseState;
use crate::utils::take_and_drop_while;

// it assumes that the given line is a valid code fence
pub fn read_code_fence_info(line: &Line) -> ParseState {
    let (fence, info_string) = take_and_drop_while(&line.content, line.content[0]);

    ParseState::CodeFence {
        language: todo!(),
        line_num: todo!(),
        code_fence_size: fence.len(),
        is_tilde_fence: line.content[0] == '~' as u16
    }
}