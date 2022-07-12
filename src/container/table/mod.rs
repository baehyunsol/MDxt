mod alignment;
mod cell;
pub mod macros;

#[cfg(test)]
mod testbench;

use alignment::parse_alignments;
use cell::{Cell, row_to_cells, get_colspan};
use macros::{is_macro_row, try_parse_macro};
use crate::ast::{doc_data::DocData, line::Line};
use crate::render::render_option::RenderOption;
use crate::inline::parse::{escape_code_spans, is_code_span_marker_begin, is_code_span_marker_end};
use crate::inline::math::escape_inside_math_blocks;
use crate::escape::BACKSLASH_ESCAPE_MARKER;
use crate::utils::into_v16;

pub struct Table {
    header: Vec<Vec<Cell>>,
    cells: Vec<Vec<Cell>>,
    collapsible: bool,
    default_hidden: bool,
    index: usize
}

impl Table {

    // it has at least two lines: header, and delimiter
    // it assumes all the lines are valid table rows
    pub fn from_lines(headers: &Vec<Line>, mut rows: &[Line], alignments: &Line, index: usize) -> Self {
        let alignments = parse_alignments(&alignments);

        let header = headers.iter().map(|row| row_to_cells(row, alignments.len(), &alignments)).collect::<Vec<Vec<Cell>>>();

        // configured by table-wide macros
        let (mut collapsible, mut default_hidden) = (false, false);

        if rows.len() > 0 && is_macro_row(&rows[0]) {
            let (collapsible_, default_hidden_) = try_parse_macro(&rows[0]);
            collapsible = collapsible_;
            default_hidden = default_hidden_;

            rows = &rows[1..];
        }

        let cells = rows.iter().map(|row| row_to_cells(row, alignments.len(), &alignments)).collect::<Vec<Vec<Cell>>>();

        Table {
            header, cells,
            collapsible, default_hidden,
            index
        }
    }

    pub fn parse_inlines(&mut self, doc_data: &mut DocData, render_option: &RenderOption) {

        // this branch should not be in the `parse_inlines` function!!!
        if self.collapsible {
            doc_data.has_collapsible_table = true;
        }

        self.header.iter_mut().for_each(
            |row| {
                row.iter_mut().for_each(
                    |cell| {cell.content.parse_raw(doc_data, render_option);}
                );
            }
        );

        self.cells.iter_mut().for_each(
            |row| {
                row.iter_mut().for_each(
                    |cell| {cell.content.parse_raw(doc_data, render_option);}
                );
            }
        );

    }

    pub fn to_html(&self) -> Vec<u16> {
        let mut result = Vec::with_capacity(6 + self.header.len() + 3 * self.cells.len());
        result.push(into_v16("<table>"));

        let collapsible_head = if self.collapsible {
            let default_value = if self.default_hidden {
                " collapsed"
            } else {
                ""
            };

            format!(" id=\"table-collapse-toggle-{}\" class=\"collapsible{}\" onclick =\"collapse_table('{}')\"", self.index, default_value, self.index)
        } else {
            String::new()
        };

        result.push(into_v16(&format!("<thead{}>", collapsible_head)));
        self.header.iter().for_each(
            |row| {
                result.push(into_v16("<tr>"));
                result.push(row.iter().map(|c| c.to_html(true)).collect::<Vec<Vec<u16>>>().concat());
                result.push(into_v16("</tr>"));
            }
        );
        result.push(into_v16("</thead>"));

        let collapsible_body = if self.collapsible {
            let default_value = if self.default_hidden {
                " class=\"invisible\""
            } else {
                ""
            };

            format!(" id=\"collapsible-table-{}\"{}", self.index, default_value)
        } else {
            String::new()
        };

        if self.cells.len() > 0 {
            result.push(into_v16(&format!("<tbody{}>", collapsible_body)));
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

// it escapes `|` inside code spans and math macros
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
