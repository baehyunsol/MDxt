use super::count_cells;
use crate::ast::line::Line;
use crate::utils::into_v16;
use crate::render_to_html_with_default_options;
use crate::testbench::remove_whitespaces;

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
", "
<table>
    <thead>
        <th> a </th>
        <th> b </th>
        <th> c </th>
    </thead>
    <tbody>
        <tr>
            <td> math inside a table </td>
            <td> \\( &#124;a&#124; \\) </td>
            <td> the pipe shouldn&apos;t break a cell </td>
        </tr>
        <tr>
            <td> <code class=\"short\">&#124;</code> </td>
            <td> a pipe in a *cell </td>
            <td> inter-cell highlights* </td>
        </tr>
    </tbody>
</table>
"), ("
|Left aligned Column |Centered Column |Right aligned Column |
|:-------------------|:--------------:|--------------------:|
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
cut
|        Left        |     Center     |        Right        |

", "
<table>
    <thead>
        <th> <div class=\"align_left\">Left aligned Column</div> </th>
        <th> <div class=\"align_center\">Centered Column </div> </th>
        <th> <div class=\"align_right\">Right aligned Column </div> </th>
    </thead>
    <tbody>
        <tr>
            <td> <div class=\"align_left\"> Left </div> </td>
            <td> <div class=\"align_center\"> Center </div> </td>
            <td> <div class=\"align_right\"> Right </div> </td>
        </tr>
        <tr>
            <td> <div class=\"align_left\"> Left </div> </td>
            <td> <div class=\"align_center\"> Center </div> </td>
            <td> <div class=\"align_right\"> Right </div> </td>
        </tr>
        <tr>
            <td> <div class=\"align_left\"> Left </div> </td>
            <td> <div class=\"align_center\"> Center </div> </td>
            <td> <div class=\"align_right\"> Right </div> </td>
        </tr>
        <tr>
            <td> <div class=\"align_left\"> Left </div> </td>
            <td> <div class=\"align_center\"> Center </div> </td>
            <td> <div class=\"align_right\"> Right </div> </td>
        </tr>
    </tbody>
</table>

<p>cut |        Left        |     Center     |        Right        |</p>
"), ("
|  not  |   a   | table |
|-------|:-----:|

|  not  | a table |
|-------|---------|------|

|  not  | a table
|-------|---------|

|  not  | a table |
|-------|---------
", "
<p>
    |  not  |   a   | table |
    |-------|:-----:|
</p>
<p>
    |  not  | a table |
    |-------|---------|------|
</p>
<p>
    |  not  | a table
    |-------|---------|
</p>
<p>
    |  not  | a table |
    |-------|---------
</p>
"), ("
|  valid  |  table  |
|---------|---------|
| okay    |
| okay    | okay    | ignored |
| not a row
| not a row
| not     | in the  | table   |
", "
<table>
    <thead>
        <th> valid </th>
        <th> table </th>
    </thead>
    <tbody>
        <tr>
            <td> okay </td>
            <td></td>
        </tr>
        <tr>
            <td> okay </td>
            <td> okay </td>
        </tr>
    </tbody>
</table>

<p>| not a row | not a row | not | in the | table |</p>
"), ("
|-----|-----|
|-----|-----|-----|
|-----|-----|
|-----|-----|-----|
|-----|-----|-----|
|-----|-----|
|-----|-----|-----|
", "
<p>
    |-----|-----|
    |-----|-----|-----|
    |-----|-----|
</p>

<table>
    <thead>
        <th>-----</th>
        <th>-----</th>
        <th>-----</th>
    </thead>
    <tbody>
        <tr>
            <td>-----</td>
            <td>-----</td>
            <td></td>
        </tr>
        <tr>
            <td>-----</td>
            <td>-----</td>
            <td>-----</td>
        </tr>
    </tbody>
</table>
"),
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
fn footnote_test() {

    for (md, html) in table_samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v16(&rendered)) != remove_whitespaces(&into_v16(html)) {
            panic!("{} \n\n {}", md, rendered);
        }

    }

}