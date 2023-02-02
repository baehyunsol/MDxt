use crate::ast::line::Line;
use crate::utils::drop_while;

// line.content doesn't contain bullet/marker
pub fn parse_task_list(line: &Line) -> (Line, Option<TaskMarker>) {
    let whitespace_trimed = drop_while(&line.content, ' ' as u32);

    let is_task_list = whitespace_trimed.len() > 4
        && whitespace_trimed[0] == '[' as u32
        && whitespace_trimed[2] == ']' as u32
        && whitespace_trimed[3] == ' ' as u32
        && (
            whitespace_trimed[1] == ' ' as u32
            || whitespace_trimed[1] == 'x' as u32
            || whitespace_trimed[1] == 'X' as u32
            || whitespace_trimed[1] == '^' as u32
    );

    if !is_task_list {
        (line.clone(), None)
    }

    else if whitespace_trimed[1] == ' ' as u32 {
        (Line::new(whitespace_trimed[4..].to_vec(), line.indent), Some(TaskMarker::Unchecked))
    }

    else if whitespace_trimed[1] == '^' as u32 {
        (Line::new(whitespace_trimed[4..].to_vec(), line.indent), Some(TaskMarker::Triangle))
    }

    else {
        (Line::new(whitespace_trimed[4..].to_vec(), line.indent), Some(TaskMarker::Checked))
    }

}

#[derive(Clone)]
pub enum TaskMarker {
    Unchecked, Checked, Triangle
}