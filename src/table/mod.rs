mod cell;

#[cfg(test)]
mod testbench;

use cell::Cell;
use crate::ast::line::Line;
use crate::inline::parse::{escape_code_spans, is_code_span_marker_begin, is_code_span_marker_end};
use crate::math::escape_inside_math_blocks;
use crate::escape::BACKSLASH_ESCAPE_MARKER;

pub struct Table {
    cells: Vec<Vec<Cell>>
}

impl Table {
    pub fn from_lines(lines: &Vec<Line>) -> Self {}
}

// it does not check whether the row is valid
pub fn count_cells(row: &[u16], pipes_escaped: bool) -> usize {

    if !pipes_escaped {
        return count_cells(&escape_pipes(row), true);
    }

    row.iter().filter(|c| **c == '|' as u16).collect::<Vec<&u16>>().len() - 1
}

// it does not check whether the delimiter is valid
pub fn count_delimiter_cells(delimiter: &[u16]) -> usize {
    count_cells(delimiter, true)
}

// it escapes `|` inside codespans and math macros
pub fn escape_pipes(content: &[u16]) -> Vec<u16> {

    let mut content = escape_code_spans(content);
    content = escape_inside_math_blocks(content);

    let mut result = Vec::with_capacity(content.len() * 5 / 4);
    let mut index = 0;
    let mut is_inside_code_span = false;

    while index < content.len() {

        if is_code_span_marker_begin(&content, index) {
            is_inside_code_span = true;
        }

        else if is_code_span_marker_end(&content, index) {
            is_inside_code_span = false;
        }

        if is_inside_code_span && content[index] == '|' as u16 {
            result.push(BACKSLASH_ESCAPE_MARKER);
            result.push(u16::MAX - '|' as u16);
        }

        else {
            result.push(content[index]);
        }

        index += 1;
    }

    result
}