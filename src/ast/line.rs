use crate::utils::into_v32;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub content: Rc<Vec<u32>>,
    pub indent: usize
}


impl Line {

    pub fn new(content: Vec<u32>, indent: usize) -> Line {
        Line { content: Rc::new(content), indent }
    }

    pub fn from_raw(raw: &[u32]) -> Line {
        let mut indent = 0;
        let mut index = 0;

        while index < raw.len() {

            if raw[index] == ' ' as u32 {
                indent += 1;
            }

            else if raw[index] == '\t' as u32 {
                indent += 4;
            }

            else {
                break;
            }

            index += 1;
        }

        Line {
            content: Rc::new(raw[index..].to_vec()),
            indent
        }

    }

    pub fn to_raw(&self) -> Vec<u32> {
        vec![vec![' ' as u32; self.indent], self.content.to_vec()].concat()
    }

    // if self.indent > n, it subtracts n, otherwise, it sets indent to 0
    pub fn try_sub_indent(&self, n: usize) -> Self {
        Line {
            content: self.content.clone(),
            indent: self.indent.max(n) - n
        }
    }

    #[cfg(test)]
    pub fn from_raw_string(string: &str) -> Line {
        Line::from_raw(&into_v32(string))
    }

}


pub fn code_to_lines(code: &[u32]) -> Vec<Line> {
    code.split(
        |c| *c == '\n' as u32
    ).map(
        |ln| Line::from_raw(&ln.to_vec())
    ).collect::<Vec<Line>>()
}

pub fn add_br_if_needed(line: &[u32]) -> Vec<u32> {
    if line.len() > 1 && line[line.len() - 1] == '\\' as u32 {
        vec![
            line[0..(line.len() - 1)].to_vec(),
            into_v32("[[br]]")  // will later be converted to `<br/>`
        ].concat()
    }

    else if line.len() > 2 && line[line.len() - 1] == ' ' as u32 && line[line.len() - 2] == ' ' as u32 {
        vec![
            line[0..(line.len() - 2)].to_vec(),
            into_v32("[[br]]")  // will later be converted to `<br/>`
        ].concat()
    }

    else {
        line.to_vec()
    }
}