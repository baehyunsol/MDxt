use super::escape_pipes;
use super::alignment::TableAlignment;
use crate::inline::InlineNode;
use crate::ast::line::Line;
use crate::utils::into_v16;

pub struct Cell {
    pub content: InlineNode,
    pub alignment: TableAlignment
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(InlineNode::Raw(vec![]))
    }
}

impl Cell {

    pub fn new(content: InlineNode) -> Self {
        Cell {
            content,
            alignment: TableAlignment::None
        }
    }

    pub fn to_html(&self, is_header: bool) -> Vec<u16> {

        if is_header {
            vec![
                into_v16("<th>"),
                self.alignment.opening_tag(),
                self.content.to_html(),
                self.alignment.closing_tag(),
                into_v16("</th>")
            ].concat()
        }

        else {
            vec![
                into_v16("<td>"),
                self.alignment.opening_tag(),
                self.content.to_html(),
                self.alignment.closing_tag(),
                into_v16("</td>")
            ].concat()
        }

    }

}

pub fn row_to_cells(row: &Line, num_of_cells: usize, alignments: &Vec<TableAlignment>) -> Vec<Cell> {
    let content = escape_pipes(&row.content);

    // the first and the last element of `cells` is empty, because the line has trailing and leading pipes.
    // the empty elements should be eliminated
    let cells = content.split(|c| *c == '|' as u16).collect::<Vec<&[u16]>>();

    let mut cells = cells[1..cells.len() - 1].iter().map(
        |c| Cell::new(InlineNode::Raw(c.to_vec()))
    ).collect::<Vec<Cell>>();

    while cells.len() > num_of_cells {
        cells.pop();
    }

    while cells.len() < num_of_cells {
        cells.push(Cell::default());
    }

    for (index, alignment) in alignments.iter().enumerate() {
        cells[index].alignment = alignment.clone();
    }

    cells
}