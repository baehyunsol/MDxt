use super::alignment::TableAlignment;
use super::escape_pipes;
use crate::ast::line::Line;
use crate::inline::InlineNode;
use crate::inline::macros::{get_macro_name, parse_arguments, predicate::read_macro};
use crate::utils::{get_bracket_end_index, into_v32, strip_whitespaces, to_int};

#[derive(Clone)]
pub struct Cell {
    pub content: InlineNode,
    pub alignment: TableAlignment,
    pub colspan: usize,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(&vec![])
    }
}

impl Cell {
    pub fn new(content: &[u32]) -> Self {
        let whitespaces_striped = strip_whitespaces(content);

        Cell {
            content: InlineNode::Raw(remove_colspan_macro(&whitespaces_striped)),
            colspan: get_colspan(&whitespaces_striped),
            alignment: TableAlignment::None
        }
    }

    pub fn to_html(&self, is_header: bool, toc_rendered: &[u32], class_prefix: &str) -> Vec<u32> {
        let colspan_attr = if self.colspan > 1 {
            format!(" colspan=\"{}\"", self.colspan)
        }

        else {
            String::new()
        };

        if is_header {
            vec![
                into_v32(&format!("<th{colspan_attr}>")),
                self.alignment.opening_tag(class_prefix),
                self.content.to_html(toc_rendered, class_prefix),
                self.alignment.closing_tag(),
                vec![60, 47, 116, 104, 62],  // into_v32("</th>")
            ].concat()
        }

        else {
            vec![
                into_v32(&format!("<td{colspan_attr}>")),
                self.alignment.opening_tag(class_prefix),
                self.content.to_html(toc_rendered, class_prefix),
                self.alignment.closing_tag(),
                vec![60, 47, 116, 100, 62],  // into_v32("</td>")
            ].concat()
        }

    }

    pub fn extract_text(&self) -> Vec<u32> {
        self.content.extract_text()
    }
}

pub fn row_to_cells(row: &Line, num_of_cells: usize, alignments: &Vec<TableAlignment>) -> Vec<Cell> {
    let content = escape_pipes(&row.content);

    // the first and the last element of `cells` is empty, because the line has trailing and leading pipes.
    // the empty elements should be eliminated
    let cells = content.split(|c| *c == '|' as u32).collect::<Vec<&[u32]>>();

    let mut cells = cells[1..(cells.len() - 1)].iter().map(
        |c| Cell::new(c)
    ).collect::<Vec<Cell>>();

    while count_columns(&cells) > num_of_cells {
        cells.pop();
    }

    while count_columns(&cells) < num_of_cells {
        cells.push(Cell::default());
    }

    let mut cell_index = 0;

    for cell in cells.iter_mut() {
        cell.alignment = alignments[cell_index].clone();
        cell_index += cell.colspan;
    }

    cells
}

fn count_columns(cells: &Vec<Cell>) -> usize {
    cells.iter().map(|cell| cell.colspan).sum::<usize>()
}

// it assumes that the whitespaces are already stripped
pub fn get_colspan(content: &[u32]) -> usize {
    match read_macro(&content, 0) {
        Some(m) => {
            let macro_arguments = parse_arguments(&m);
            let macro_name = get_macro_name(&macro_arguments);

            // into_v32("colspan") -> [99, 111, 108, 115, 112, 97, 110]
            if macro_arguments.len() == 1 && macro_arguments[0].len() == 2 && macro_name == [99, 111, 108, 115, 112, 97, 110] {
                match to_int(&macro_arguments[0][1]) {
                    Some(n) if n > 0 => n as usize,
                    _ => 1,
                }
            }

            else {
                1
            }
        },
        _ => 1
    }
}

// it assumes that the whitespaces are already stripped
pub fn remove_colspan_macro(content: &[u32]) -> Vec<u32> {
    match read_macro(&content, 0) {
        Some(m) => {
            let macro_arguments = parse_arguments(&m);
            let macro_name = get_macro_name(&macro_arguments);
            let macro_end_index = get_bracket_end_index(content, 0).unwrap();

            if macro_arguments.len() == 1 
                && macro_arguments[0].len() == 2
                && macro_name == &[99, 111, 108, 115, 112, 97, 110]  // into_v32("colspan") -> [99, 111, 108, 115, 112, 97, 110]
            {
                match to_int(&macro_arguments[0][1]) {
                    Some(n) if n > 0 => content[(macro_end_index + 1)..].to_vec(),
                    _ => content.to_vec()
                }
            }

            else {
                content.to_vec()
            }
        },
        _ => content.to_vec()
    }
}
