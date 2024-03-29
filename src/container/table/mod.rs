mod alignment;
mod cell;
pub mod macros;
mod sort;

#[cfg(test)]
mod testbench;

use alignment::parse_alignments;
use cell::{Cell, get_colspan, row_to_cells};
use macros::{TableMacros, try_parse_macro};
use crate::ast::{doc_data::DocData, line::Line};
use crate::escape::BACKSLASH_ESCAPE_OFFSET;
use crate::inline::parse::{escape_code_spans, is_code_span_marker_begin, is_code_span_marker_end};
use crate::inline::macros::predicate::is_special_macro;
use crate::inline::math::escape_inside_math_blocks;
use crate::render::render_option::RenderOption;
use crate::utils::{drop_while, from_v32, into_v32};

#[derive(Clone)]
pub struct Table {
    header: Vec<Vec<Cell>>,
    cells: Vec<Vec<Cell>>,

    // `cells[i].len()` differs due to colspans
    cols: usize,
    collapsible: bool,
    default_hidden: bool,
    headless: bool,

    // html attributes
    id: Option<Vec<u32>>,
    classes: Vec<Vec<u32>>,

    sort: bool,
    index: usize,
}

impl Table {

    // if the colspan of cells[0][0] is 3, get(0, 0), get(0, 1) and get(0, 2) would all return the same cell
    fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        match self.cells.get(row) {
            Some(row) => {
                let mut curr_col = 0;

                for cell in row.iter() {
                    if curr_col >= col {
                        return Some(cell);
                    }

                    curr_col += cell.colspan;
                }

                if curr_col >= col {
                    row.last()
                }

                else {
                    None
                }
            },
            _ => None,
        }
    }

    // it has at least two lines: header, and delimiter
    // it assumes all the lines are valid table rows
    pub fn from_lines(
        headers: &Vec<Line>,
        mut rows: &[Line],
        alignments: &Line,
        index: usize,
    ) -> Self {
        let alignments = parse_alignments(&alignments);
        let cols = alignments.len();

        let header = headers.iter().map(
            |row| row_to_cells(row, cols, &alignments)
        ).collect::<Vec<Vec<Cell>>>();

        // configured by table-wide macros
        let (
            mut collapsible,
            mut default_hidden,
            mut headless,
            mut id,
            mut classes,
            mut sort,
        ) = (false, false, false, None, vec![], false);

        // if rows[0] is `|!![[whatever macro ...]] [[another macro...]]|`
        if !rows.is_empty()
            && !rows[0].content.is_empty()
            && is_special_macro(&rows[0].content[1..(rows[0].content.len() - 1)])
        {
            let TableMacros {
                collapsible: collapsible_,
                default_hidden: default_hidden_,
                headless: headless_,
                id: id_,
                classes: classes_,
                sort: sort_,
            } = try_parse_macro(&rows[0].content);
            collapsible = collapsible_;
            default_hidden = default_hidden_;
            headless = headless_;
            id = id_;
            classes = classes_;
            sort = sort_;

            rows = &rows[1..];
        }

        let cells = rows.iter().map(
            |row| row_to_cells(row, cols, &alignments)
        ).collect::<Vec<Vec<Cell>>>();

        Table {
            header, cells, cols,
            collapsible, default_hidden, headless,
            id, classes, sort,
            index,
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

        if self.sort {
            let sort_data = self.get_sort_data();
            todo!();
        }
    }

    pub fn to_html(&self, toc_rendered: &[u32], class_prefix: &str) -> Vec<u32> {
        let mut result = Vec::with_capacity(6 + self.header.len() + 3 * self.cells.len());
        let mut classes = vec![];

        if self.headless {
            classes.push(format!("{class_prefix}headless-table"));
        }

        for class in self.classes.iter() {
            classes.push(format!("{class_prefix}{}", from_v32(class)));
        }

        result.push(into_v32(&format!(
            "<table{}{}>",
            if classes.len() > 0 {
                format!(
                    " class=\"{}\"",
                    classes.join(" "),
                )
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!(
                    " id=\"{}\"",
                    from_v32(id),
                )
            } else {
                String::new()
            }
        )));

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

        if !self.cells.is_empty() {
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
    row.split(|c| *c == '|' as u32).map(|cell| get_colspan(&drop_while(cell, ' ' as u32))).sum::<usize>() - 2
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
