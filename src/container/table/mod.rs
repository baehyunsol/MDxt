mod alignment;
mod cell;
pub mod macros;

#[cfg(test)]
mod testbench;

use alignment::parse_alignments;
use cell::{Cell, get_colspan, row_to_cells};
use macros::try_parse_macro;
use crate::ast::{doc_data::DocData, line::Line};
use crate::escape::BACKSLASH_ESCAPE_OFFSET;
use crate::inline::parse::{escape_code_spans, is_code_span_marker_begin, is_code_span_marker_end};
use crate::inline::macros::predicate::is_special_macro;
use crate::inline::math::escape_inside_math_blocks;
use crate::render::render_option::RenderOption;
use crate::utils::into_v32;

#[derive(Clone)]
pub struct Table {
    header: Vec<Vec<Cell>>,
    cells: Vec<Vec<Cell>>,
    collapsible: bool,
    default_hidden: bool,
    headless: bool,
    index: usize
}

impl Table {

    // it has at least two lines: header, and delimiter
    // it assumes all the lines are valid table rows
    pub fn from_lines(
        headers: &Vec<Line>,
        mut rows: &[Line],
        alignments: &Line,
        index: usize
    ) -> Self {
        let alignments = parse_alignments(&alignments);

        let header = headers.iter().map(
            |row| row_to_cells(row, alignments.len(), &alignments)
        ).collect::<Vec<Vec<Cell>>>();

        // configured by table-wide macros
        let (mut collapsible, mut default_hidden, mut headless) = (false, false, false);

        // if rows[0] is `|!![[whatever macro ...]] [[another macro...]]|`
        if rows.len() > 0
            && rows[0].content.len() > 0
            && is_special_macro(&rows[0].content[1..(rows[0].content.len() - 1)])
        {
            let (collapsible_, default_hidden_, headless_) = try_parse_macro(&rows[0].content);
            collapsible = collapsible_;
            default_hidden = default_hidden_;
            headless = headless_;

            rows = &rows[1..];
        }

        let cells = rows.iter().map(
            |row| row_to_cells(row, alignments.len(), &alignments)
        ).collect::<Vec<Vec<Cell>>>();

        Table {
            header, cells,
            collapsible, default_hidden, headless,
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

    pub fn to_html(&self, toc_rendered: &[u32], class_prefix: &str) -> Vec<u32> {
        let mut result = Vec::with_capacity(6 + self.header.len() + 3 * self.cells.len());

        if self.headless {
            // into_v32("<table class=\"headless-table\">")
            result.push(vec![60, 116, 97, 98, 108, 101, 32, 99, 108, 97, 115, 115, 61, 34, 104, 101, 97, 100, 108, 101, 115, 115, 45, 116, 97, 98, 108, 101, 34, 62]);
        }

        else {
            // into_v32("<table>")
            result.push(vec![60, 116, 97, 98, 108, 101, 62]);
        }

        let collapsible_head = if self.collapsible {
            let default_value = if self.default_hidden {
                " collapsed"
            } else {
                ""
            };

            format!(" id=\"table-collapse-toggle-{}\" class=\"{class_prefix}collapsible{default_value}\" onclick =\"collapse_table('{}')\"", self.index, self.index)
        } else {
            String::new()
        };

        if !self.headless {
            result.push(into_v32(&format!("<thead{collapsible_head}>")));
            self.header.iter().for_each(
                |row| {
                    result.push(vec![60, 116, 114, 62]);  // into_v32("<tr>")
                    result.push(row.iter().map(
                        |c| c.to_html(true, toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u32>>>().concat());
                    result.push(vec![60, 47, 116, 114, 62]);  // into_v32("</tr>")
                }
            );
            result.push(vec![60, 47, 116, 104, 101, 97, 100, 62]);  // into_v32("</thead>")
        }

        let collapsible_body = if self.collapsible {
            let default_value = if self.default_hidden {
                format!(" class=\"{class_prefix}invisible\"")
            } else {
                String::new()
            };

            format!(" id=\"collapsible-table-{}\"{default_value}", self.index)
        } else {
            String::new()
        };

        if self.cells.len() > 0 {
            result.push(into_v32(&format!("<tbody{collapsible_body}>")));
            self.cells.iter().for_each(
                |row| {
                    result.push(vec![60, 116, 114, 62]);  // into_v32("<tr>")
                    result.push(row.iter().map(
                        |c| c.to_html(false, toc_rendered, class_prefix)
                    ).collect::<Vec<Vec<u32>>>().concat());
                    result.push(vec![60, 47, 116, 114, 62]);  // into_v32("</tr>")
                }
            );
            result.push(vec![60, 47, 116, 98, 111, 100, 121, 62]);  // into_v32("</tbody>")
        }

        result.push(vec![60, 47, 116, 97, 98, 108, 101, 62]);  // into_v32("</table>")
        result.concat()
    }

}

// it does not check whether the row is valid
pub fn count_cells(row: &[u32], pipes_escaped: bool) -> usize {

    if !pipes_escaped {
        return count_cells(&escape_pipes(row), true);
    }

    // the `.split` method generates 2 extra elements, the trailing and leading empty cells
    // they should be removed
    row.split(|c| *c == '|' as u32).map(|cell| get_colspan(cell)).sum::<usize>() - 2
}

// it does not check whether the delimiter is valid
pub fn count_delimiter_cells(delimiter: &[u32]) -> usize {
    count_cells(delimiter, true)
}

// it escapes `|` inside code spans and math macros
pub fn escape_pipes(content: &[u32]) -> Vec<u32> {

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

        if is_inside_code_span && content[index] == '|' as u32 {
            result.push(BACKSLASH_ESCAPE_OFFSET + '|' as u32);
        }

        else {
            result.push(content[index]);
        }

        index += 1;
    }

    result
}
