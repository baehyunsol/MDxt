mod alignment;
mod cell;

#[cfg(test)]
mod testbench;

use alignment::{TableAlignment, parse_alignments};
use cell::{Cell, row_to_cells, get_colspan};
use crate::ast::{MdData, line::Line};
use crate::render::render_option::RenderOption;
use crate::inline::parse::{escape_code_spans, is_code_span_marker_begin, is_code_span_marker_end};
use crate::inline::math::escape_inside_math_blocks;
use crate::escape::BACKSLASH_ESCAPE_MARKER;
use crate::utils::into_v16;

pub struct Table {
    header: Vec<Vec<Cell>>,
    alignments: Vec<TableAlignment>,
    cells: Vec<Vec<Cell>>
}

impl Table {

    // it has at least two lines: header, and delimiter
    // it assumes all the lines are valid table rows
    pub fn from_lines(headers: &Vec<Line>, rows: &Vec<Line>, alignments: &Line) -> Self {
        let alignments = parse_alignments(&alignments);

        let header = headers.iter().map(|row| row_to_cells(row, alignments.len(), &alignments)).collect::<Vec<Vec<Cell>>>();

        let cells = rows.iter().map(|row| row_to_cells(row, alignments.len(), &alignments)).collect::<Vec<Vec<Cell>>>();

        Table {
            header, alignments, cells
        }
    }

    pub fn parse_inlines(&mut self, md_data: &mut MdData, render_option: &RenderOption) {

        self.header.iter_mut().for_each(
            |row| {
                row.iter_mut().for_each(
                    |cell| {cell.content.parse_raw(md_data, render_option);}
                );
            }
        );

        self.cells.iter_mut().for_each(
            |row| {
                row.iter_mut().for_each(
                    |cell| {cell.content.parse_raw(md_data, render_option);}
                );
            }
        );

    }

    pub fn to_html(&self) -> Vec<u16> {
        let mut result = Vec::with_capacity(6 + self.header.len() + 3 * self.cells.len());
        result.push(into_v16("<table>"));

        result.push(into_v16("<thead>"));
        self.header.iter().for_each(
            |row| {
                result.push(into_v16("<tr>"));
                result.push(row.iter().map(|c| c.to_html(true)).collect::<Vec<Vec<u16>>>().concat());
                result.push(into_v16("</tr>"));
            }
        );
        result.push(into_v16("</thead>"));

        if self.cells.len() > 0 {
            result.push(into_v16("<tbody>"));
            self.cells.iter().for_each(
                |row| {
                    result.push(into_v16("<tr>"));
                    result.push(row.iter().map(|c| c.to_html(false)).collect::<Vec<Vec<u16>>>().concat());
                    result.push(into_v16("</tr>"));
                }
            );
            result.push(into_v16("</tbody>"));
        }

        result.push(into_v16("</table>"));
        result.concat()
    }

}

// it does not check whether the row is valid
pub fn count_cells(row: &[u16], pipes_escaped: bool) -> usize {

    if !pipes_escaped {
        return count_cells(&escape_pipes(row), true);
    }

    // the `.split` method generates 2 extra elements, the trailing and leading empty cells
    // they should be removed
    row.split(|c| *c == '|' as u16).map(|cell| get_colspan(cell)).sum::<usize>() - 2
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
