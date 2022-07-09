#[cfg(test)]
mod testbench;

use crate::inline::InlineNode;
use crate::ast::line::{Line, add_br_if_needed};
use crate::ast::MdData;
use crate::render::render_option::RenderOption;
use crate::utils::{is_numeric, to_int};

pub struct List {
    list_type: ListType,
    start_index: usize,
    elements: Vec<Element>
}

impl List {

    pub fn from_lines(lines: &Vec<Line>) -> Self {

        let mut indentation_stack = vec![i32::MIN];
        let mut elements = Vec::with_capacity(lines.len());
        let (list_type, start_index) = get_list_type_and_start_index(&lines[0]);
        let mut curr_element = vec![];

        for line in lines.iter() {

            if !line.is_ordered_list() && list_type.is_ordered() || !line.is_unordered_list() && list_type.is_unordered() {
                curr_element.push(line.clone());
                continue;
            }

            if curr_element.len() > 0 {
                elements.push(Element::new(
                    indentation_stack.len(),
                    &curr_element.iter().map(add_br_if_needed).collect::<Vec<Vec<u16>>>().join(&[' ' as u16][..])
                ));
            }

            let head = indentation_stack[indentation_stack.len() - 1];
            let curr_indent = line.indent as i32;

            if head + 1 < curr_indent {
                indentation_stack.push(curr_indent);
            }

            else if head - 1 > curr_indent {

                while indentation_stack[indentation_stack.len() - 1] > curr_indent {
                    indentation_stack.pop().unwrap();
                }

                if indentation_stack[indentation_stack.len() - 1] + 1 < curr_indent {
                    indentation_stack.push(curr_indent);
                }

            }

            else {
                let last_index = indentation_stack.len() - 1;
                indentation_stack[last_index] = curr_indent;
            }

            curr_element = vec![remove_marker(&line)];
        }

        List { elements, list_type, start_index }
    }

    pub fn to_html(&self) -> Vec<u16> {
        todo!()
    }

    pub fn parse_inlines(&mut self, md_data: &mut MdData, options: &RenderOption) {
        todo!()
    }

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

enum ListType {
    Unordered,
    Ordered(Marker)
}

impl ListType {

    fn is_ordered(&self) -> bool {
        match self {
            ListType::Ordered(_) => true,
            _ => false
        }
    }

    fn is_unordered(&self) -> bool {
        match self {
            ListType::Ordered(_) => false,
            _ => true
        }
    }

}

enum Marker {
    Number, UpperAlpha, LowerAlpha, UpperRoman, LowerRoman
}

struct Element {
    level: usize,
    content: InlineNode
}

impl Element {

    fn new(level: usize, content: &[u16]) -> Self {
        Element {
            level,
            content: InlineNode::Raw(content.to_vec())
        }
    }

}