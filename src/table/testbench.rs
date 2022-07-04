use super::count_cells;
use crate::ast::line::Line;
use crate::utils::into_v16;

fn row_samples() -> Vec<(String, usize, bool)> {  // (row, cell_count, is_delimiter)
    let result = vec![
        ("|a|b|c|", 3, false),
        ("a|b|c|", 0, false),
        ("|a|b|c", 0, false),
        ("| a | b | `c|` |", 3, false),
        ("| a | b | [[math]] |c| [[/math]] |", 3, false),
        ("|---|:---:|-|", 3, true),
        ("|---|:---: |-|", 3, true),
        ("|---|:-- -:|-|", 3, false),
        ("|---|:--:-:|-|", 3, false),
    ];

    result.into_iter().map(
        |(row, cell_count, is_delimiter)| (row.to_string(), cell_count, is_delimiter)
    ).collect()
}

fn table_samples() -> Vec<(String, String)> {
    let result = vec![
        ("
|            a            |            b            |            c            |
|-------------------------|-------------------------|-------------------------|
| math inside a table     | [[math]] |a| [[/math]]  | the pipe shouldn't break a cell |
|           `|`           | a pipe in a *cell       | inter-cell highlights*  |
", ""), ("
|Left aligned Column |Centered Column |Right aligned Column |
|:-------------------|:--------------:|--------------------:|
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
cut
|        Left        |     Center     |        Right        |

", ""), ("
|  not  |   a   | table |
|-------|:-----:|

|  not  | a table |
|-------|---------|------|
", ""), ("
|  valid  |  table  |
|---------|---------|
| okay    |
| okay    | okay    | ignored |
| not a row
| not a row
| not     | in the  | table   |
", ""), ("
|-----|-----|
|-----|-----|-----|
|-----|-----|
|-----|-----|-----|
|-----|-----|-----|
|-----|-----|
|-----|-----|-----|
", ""),
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn cell_count_test() {
    let mut failures = vec![];

    for (row, cell_count, is_delimiter) in row_samples().into_iter() {
        let line = Line::from_raw(&into_v16(&row));

        if !line.is_table_row() {

            if cell_count != 0 || is_delimiter {
                failures.push(format!(
                    "row: {}\nIt's a valid row, but `.is_table_row()` is false.",
                    row
                ));
            }

        }

        else if line.is_table_delimiter() != is_delimiter {
            failures.push(format!(
                "row: {}\n`line.is_table_delimiter()`: {}, `is_delimiter`: {}",
                row,
                line.is_table_delimiter(),
                is_delimiter
            ));
        }

        else if cell_count != count_cells(&into_v16(&row), false) {
            failures.push(format!(
                "row: {}\nIt has {} cells, but `count_cells` says it's {}",
                row, cell_count, count_cells(&into_v16(&row), false)
            ));
        }

    }

    if failures.len() > 0 {
        panic!(
            "Cell count test: {} case(s) out of {} cases have failed!\n\n{}",
            failures.len(),
            row_samples().len(),
            failures.join("\n\n")
        );
    }

}

#[test]
fn table_test() {
    todo!()
}