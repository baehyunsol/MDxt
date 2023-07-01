use super::count_cells;
use crate::ast::line::Line;
use crate::utils::{into_v32, remove_whitespaces};
use crate::render_to_html_with_default_options;

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
        <tr>
            <th> a </th>
            <th> b </th>
            <th> c </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> math inside a table </td>
            <td> <math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mo>|</mo><mi>a</mi><mo>|</mo></math> </td>
            <td> the pipe shouldn&apos;t break a cell </td>
        </tr>
        <tr>
            <td> <code class=\"inline-code-span\">&#124;</code> </td>
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
        <tr>
            <th> <div class=\"align-left\">Left aligned Column</div> </th>
            <th> <div class=\"align-center\">Centered Column </div> </th>
            <th> <div class=\"align-right\">Right aligned Column </div> </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> <div class=\"align-left\"> Left </div> </td>
            <td> <div class=\"align-center\"> Center </div> </td>
            <td> <div class=\"align-right\"> Right </div> </td>
        </tr>
        <tr>
            <td> <div class=\"align-left\"> Left </div> </td>
            <td> <div class=\"align-center\"> Center </div> </td>
            <td> <div class=\"align-right\"> Right </div> </td>
        </tr>
        <tr>
            <td> <div class=\"align-left\"> Left </div> </td>
            <td> <div class=\"align-center\"> Center </div> </td>
            <td> <div class=\"align-right\"> Right </div> </td>
        </tr>
        <tr>
            <td> <div class=\"align-left\"> Left </div> </td>
            <td> <div class=\"align-center\"> Center </div> </td>
            <td> <div class=\"align-right\"> Right </div> </td>
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
|||
|---------|---------|
| okay    |
| okay    | okay    | ignored |
| not a row
| not a row
| not     | in the  | table   |
", "
<table>
    <thead>
        <tr>
            <th> valid </th>
            <th> table </th>
        </tr>
        <tr>
            <th> </th>
            <th> </th>
        </tr>
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

|a|b|
|-|-|
", "
<p>
    |-----|-----|
    |-----|-----|-----|
    |-----|-----|
</p>

<table>
    <thead>
        <tr>
            <th>-----</th>
            <th>-----</th>
            <th>-----</th>
        </tr>
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

<table>
    <thead>
        <tr>
            <th> a </th>
            <th> b </th>
        </tr>
    </thead>
</table>
"), ("
|         [[colspan = 6]] Shopping List         |
| [[colspan = 3]] Food  | [[colspan = 3]] Drink |
|-------|:-----:|-------|:-----:|-------|-------|
| Bread | Cake  | Pie   | Beer  | Water | Coffee|
| None  | Center| None  | Center| None  | None  |
| Foo   | [[colspan = 4]] *Bar*         |
", "
<table>
    <thead>
        <tr>
            <th colspan=\"6\"> Shopping List </th>
        </tr>
        <tr>
            <th colspan=\"3\"> Food </th>
            <th colspan=\"3\"> <div class=\"align-center\"> Drink </div> </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> Bread </td>
            <td> <div class=\"align-center\"> Cake </div> </td>
            <td> Pie </td>
            <td> <div class=\"align-center\"> Beer </div> </td>
            <td> Water </td>
            <td> Coffee </td>
        </tr>
        <tr>
            <td> None </td>
            <td> <div class=\"align-center\"> Center </div> </td>
            <td> None </td>
            <td> <div class=\"align-center\"> Center </div> </td>
            <td> None </td>
            <td> None </td>
        </tr>
        <tr>
            <td> Foo </td>
            <td colspan=\"4\"> <div class=\"align-center\"> <em> Bar </em> </div> </td>
            <td> </td>
        </tr>
    </tbody>
</table>
"), ("
|a|b|c|
|-|-|-|
 a|b|c
 a|b|c
|a|b|c|
|a|b|c|
|-|-|-|
|a|b|c|

|a|b|c|
|-|-|-|
|a|b|c|
", "
<table>
    <thead>
        <tr>
            <th> a </th>
            <th> b </th>
            <th> c </th>
        </tr>
    </thead>
</table>

<p>a|b|c a|b|c</p>

<table>
    <thead>
        <tr>
            <th> a </th>
            <th> b </th>
            <th> c </th>
        </tr>
        <tr>
            <th> a </th>
            <th> b </th>
            <th> c </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> a </td>
            <td> b </td>
            <td> c </td>
        </tr>
    </tbody>
</table>

<table>
    <thead>
        <tr>
            <th> a </th>
            <th> b </th>
            <th> c </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> a </td>
            <td> b </td>
            <td> c </td>
        </tr>
    </tbody>
</table>
"), ("
## Table 1
|a|b|c|  
|-|-|-|
## Table 2
 |a|b|c| 
|-|-|-|
  |a|b|c| 
## Table 3
|a|b|c|
|a|b|c|
|-|-|-|
||||", "
<h2 id=\"table-1\">Table 1</h2>

<table>
    <thead>
        <tr>
            <th> a </th>
            <th> b </th>
            <th> c </th>
        </tr>
    </thead>
</table>

<h2 id=\"table-2\">Table 2</h2>

<table>
    <thead>
        <tr><th>a</th><th>b</th><th>c</th></tr>
    </thead>
    <tbody>
        <tr><td>a</td><td>b</td><td>c</td></tr>
    </tbody>
</table>

<h2 id=\"table-3\">Table 3</h2>

<table>
    <thead>
        <tr>
            <th> a </th>
            <th> b </th>
            <th> c </th>
        </tr>
        <tr>
            <th> a </th>
            <th> b </th>
            <th> c </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> </td>
            <td> </td>
            <td> </td>
        </tr>
    </tbody>
</table>
"), ("
| an  | escaped | pipe |
|-----|---------|------|
|*\\|*|\\|\\|\\||\\|\\||
", "
<table>
    <thead>
        <tr>
            <th> an </th>
            <th> escaped </th>
            <th> pipe </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><em>&#124;</em></td>
            <td>&#124;&#124;&#124;</td>
            <td>&#124;&#124;</td>
        </tr>
    </tbody>
</table>
"), ("
| Click Me! (Default shown)              |
|----------------------------------------|
|!![[collapsible, default=shown]]        |
| Hi, there!                             |

| Click Me! (Default hidden)            |
|---------------------------------------|
|!![[collapsible, default=hidden]]      |
| Hi, there!                            |
", "
<table>
    <thead id=\"table-collapse-toggle-0\" class=\"collapsible\" onclick =\"collapse_table('0')\">
        <tr><th> Click Me! (Default shown) </th></tr>
    </thead>
    <tbody id=\"collapsible-table-0\">
        <tr><td> Hi, there! </td></tr>
    </tbody>
</table>
<table>
    <thead id=\"table-collapse-toggle-1\" class=\"collapsible collapsed\" onclick =\"collapse_table('1')\">
        <tr><th> Click Me! (Default hidden) </th></tr>
    </thead>
    <tbody id=\"collapsible-table-1\" class=\"invisible\">
        <tr><td> Hi, there! </td></tr>
    </tbody>
</table>

<script>function collapse_table(n) {
    var head = document.getElementById(\"table-collapse-toggle-\" + n);
    head.classList.toggle(\"collapsed\");

    var content = document.getElementById(\"collapsible-table-\" + n);
    content.classList.toggle(\"invisible\");
}</script>
"), ("
| This table head is not shown.      |
|------------------------------------|
|!![[headless]]                      |
| This is a headless table.          |
| This is another row of the table.  |
", "
<table class=\"headless-table\">
    <tbody>
        <tr><td>This is a headless table.</td></tr>
        <tr><td>This is another row of the table.</td></tr>
    </tbody>
</table>")
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn cell_count_test() {
    let mut failures = vec![];

    for (row, cell_count, is_delimiter) in row_samples().into_iter() {
        let line = Line::from_raw(&into_v32(&row));

        if !line.is_table_row() {

            if cell_count != 0 || is_delimiter {
                failures.push(format!(
                    "row: {row}\nIt's a valid row, but `.is_table_row()` is false.",
                ));
            }

        }

        else if line.is_table_delimiter() != is_delimiter {
            failures.push(format!(
                "row: {row}\n`line.is_table_delimiter()`: {}, `is_delimiter`: {is_delimiter}",
                line.is_table_delimiter(),
            ));
        }

        else if cell_count != count_cells(&into_v32(&row), false) {
            failures.push(format!(
                "row: {row}\nIt has {cell_count} cells, but `count_cells` says it's {}",
                count_cells(&into_v32(&row), false)
            ));
        }

    }

    if !failures.is_empty() {
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

    for (md, html) in table_samples().iter() {
        let rendered = render_to_html_with_default_options(md);
        let rendered_v32 = into_v32(&rendered);
        let html_v32 = into_v32(html);

        if remove_whitespaces(&rendered_v32) != remove_whitespaces(&html_v32) {
            panic!("{md} \n\n {rendered}");
        }

        if rendered.contains("<td> ")
            || rendered.contains(" </td>")
            || rendered.contains("<th> ")
            || rendered.contains(" </th>")
        {
            panic!("A table's trailing whitespaces are not trimmed properly: {rendered}");
        }

    }

}