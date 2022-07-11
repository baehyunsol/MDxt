mod tasklist;

#[cfg(test)]
mod testbench;

use crate::inline::InlineNode;
use crate::ast::line::{Line, add_br_if_needed};
use crate::ast::MdData;
use crate::render::render_option::RenderOption;
use crate::utils::{is_numeric, to_int, into_v16};
use tasklist::{parse_task_list, TaskMarker};

#[derive(Clone)]
pub struct List {
    list_type: ListType,
    start_index: usize,
    elements: Vec<ElementOrSublist>
}

impl List {

    pub fn from_lines(lines: &Vec<Line>) -> Self {
        let (mut list, mut index) = from_lines_recursive(&lines[..], 0);

        /*
        It's needed in ordered to parse
        ```
                - 1
              - 1
            - 1
          - 1
        - 1
        ```
        */
        while index < lines.len() {
            let (new_list, new_index) = from_lines_recursive(&lines[..], index);
            index = new_index;

            list.elements = vec![
                list.elements,
                new_list.elements
            ].concat();
        }

        list
    }

    pub fn to_html(&self) -> Vec<u16> {

        let start_index = if self.start_index != 1 {
            format!(" start=\"{}\"", self.start_index)
        } else {
            "".to_string()
        };

        let (opening_tag, closing_tag) = match &self.list_type {
            ListType::Unordered => (
                format!("<ul{}>", start_index),
                "</ul>".to_string()
            ),
            ListType::Ordered(marker) => match marker {
                Marker::Number => (
                    format!("<ol type=\"1\"{}>", start_index),
                    "</ol>".to_string()
                ),
                Marker::UpperAlpha => (
                    // Although the current syntax of MDex doesn't support setting start_index of this marker, I'll leave it like this for future
                    format!("<ol type=\"A\"{}>", start_index),
                    "</ol>".to_string()
                ),
                Marker::LowerAlpha => (
                    // Although the current syntax of MDex doesn't support setting start_index of this marker, I'll leave it like this for future
                    format!("<ol type=\"a\"{}>", start_index),
                    "</ol>".to_string()
                ),
                Marker::UpperRoman => (
                    // Although the current syntax of MDex doesn't support setting start_index of this marker, I'll leave it like this for future
                    format!("<ol type=\"I\"{}>", start_index),
                    "</ol>".to_string()
                ),
                Marker::LowerRoman => (
                    // Although the current syntax of MDex doesn't support setting start_index of this marker, I'll leave it like this for future
                    format!("<ol type=\"i\"{}>", start_index),
                    "</ol>".to_string()
                ),
            }
        };

        let mut result = Vec::with_capacity(self.elements.len() * 3 + 2);

        result.push(into_v16(&opening_tag));

        for element in self.elements.iter() {

            match element {
                ElementOrSublist::Element{ content, task_list } => {
                    result.push(into_v16("<li>"));

                    match task_list {
                        Some(marker) => match marker {
                            TaskMarker::Unchecked => {
                                result.push(into_v16("<div class=\"unchecked_box\"></div>"));
                            },
                            TaskMarker::Checked => {
                                result.push(into_v16("<div class=\"checked_box\"><span class=\"checkmark\"></span></div>"));
                            },
                            TaskMarker::Triangle => {
                                result.push(into_v16("<div class=\"checked_box\"><span class=\"triangle\"></span></div>"));
                            },
                        },
                        _ => {}
                    }

                    result.push(content.to_html());
                    result.push(into_v16("</li>"));
                }
                ElementOrSublist::Sublist(sublist) => {
                    result.pop().unwrap();  // </li>  // the first element is `ElementOrSublist::Element`
                    result.push(sublist.to_html());
                    result.push(into_v16("</li>"));
                }
            }

        }

        result.push(into_v16(&closing_tag));

        result.concat()
    }

    pub fn parse_inlines(&mut self, md_data: &mut MdData, options: &RenderOption) {

        for element in self.elements.iter_mut() {

            match element {
                ElementOrSublist::Element{ content, .. } => {content.parse_raw(md_data, options);}
                ElementOrSublist::Sublist(sublist) => {sublist.parse_inlines(md_data, options);}
            }

        }

    }

}

fn from_lines_recursive(lines: &[Line], mut curr_index: usize) -> (List, usize) {
    let (list_type, start_index) = get_list_type_and_start_index(&lines[curr_index]);
    let mut elements = Vec::with_capacity(lines.len());
    let mut curr_indent = lines[curr_index].indent;
    let mut curr_element = vec![];
    let mut curr_task_marker = None;

    while curr_index < lines.len() {

        if lines[curr_index].is_ordered_list() || lines[curr_index].is_unordered_list() {

            if curr_element.len() > 0 {
                elements.push(
                    ElementOrSublist::new_element(
                        &curr_element.iter().map(add_br_if_needed).collect::<Vec<Vec<u16>>>().join(&[' ' as u16][..]),
                        curr_task_marker
                    )
                );
                curr_task_marker = None;
                curr_element = vec![];
            }

            if lines[curr_index].indent + 1 < curr_indent {
                break;
            }

            // sublist
            else if curr_indent + 1 < lines[curr_index].indent {
                let (sublist, new_index) = from_lines_recursive(lines, curr_index);
                elements.push(ElementOrSublist::new_sublist(Box::new(sublist)));
                curr_index = new_index;
                continue;
            }

            else {
                curr_indent = lines[curr_index].indent;
            }

            let (line, task_list_marker) = parse_task_list(&remove_marker(&lines[curr_index]));

            curr_task_marker = task_list_marker;
            curr_element = vec![line];
        }

        else {
            curr_element.push(lines[curr_index].clone());
        }

        curr_index += 1;
    }

    if curr_element.len() > 0 {
        elements.push(
            ElementOrSublist::new_element(
                &curr_element.iter().map(add_br_if_needed).collect::<Vec<Vec<u16>>>().join(&[' ' as u16][..]),
                curr_task_marker
            )
        );
    }

    (
        List {
            list_type, start_index, elements
        },
        curr_index
    )
}

fn get_list_type_and_start_index(line: &Line) -> (ListType, usize) {

    #[cfg(test)]
    assert!(line.is_ordered_list() || line.is_unordered_list());

    match line.content[0] {
        x if x == '-' as u16 || x == '*' as u16 => (
            ListType::Unordered, 1
        ),
        x if x == 'a' as u16 => (
            ListType::Ordered(Marker::LowerAlpha), 1
        ),
        x if x == 'A' as u16 => (
            ListType::Ordered(Marker::UpperAlpha), 1
        ),
        x if x == 'i' as u16 => (
            ListType::Ordered(Marker::LowerRoman), 1
        ),
        x if x == 'I' as u16 => (
            ListType::Ordered(Marker::UpperRoman), 1
        ),
        x if is_numeric(&x) => {
            let num_end_index = line.content.iter().position(|c| *c == '.' as u16).unwrap();
            let num = &line.content[0..num_end_index];

            (ListType::Ordered(Marker::Number), to_int(num).unwrap() as usize)
        }
        _ => panic!()
    }

}

fn remove_marker(line: &Line) -> Line {
    let content = match line.content[0] {
        x if x == '-' as u16 || x == '*' as u16 => (
            line.content[2..line.content.len()].to_vec()
        ),
        _ => {
            let marker_end_index = line.content.iter().position(|c| *c == '.' as u16).unwrap();

            line.content[marker_end_index + 2..line.content.len()].to_vec()
        }
    };

    Line::new(content, 0)
}

#[derive(Clone)]
enum ListType {
    Unordered,
    Ordered(Marker)
}

#[derive(Clone)]
enum Marker {
    Number, UpperAlpha, LowerAlpha, UpperRoman, LowerRoman
}

#[derive(Clone)]
enum ElementOrSublist {
    Element{
        content: InlineNode,
        task_list: Option<TaskMarker>
    },
    Sublist(Box<List>)
}

impl ElementOrSublist {

    fn new_element(content: &[u16], task_list: Option<TaskMarker>) -> Self {
        ElementOrSublist::Element{
            content: InlineNode::Raw(content.to_vec()),
            task_list
        }
    }

    fn new_sublist(list: Box<List>) -> Self {
        ElementOrSublist::Sublist(list)
    }

}