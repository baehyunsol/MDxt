use crate::utils::{from_v16, into_v16};

#[cfg(test)]
use crate::testbench::debugger::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub content: Vec<u16>,
    pub indent: usize
}


impl Line {

    pub fn new(content: Vec<u16>, indent: usize) -> Line {
        Line { content, indent }
    }

    pub fn from_raw(raw: &[u16]) -> Line {

        let mut indent = 0;
        let mut index = 0;

        while index < raw.len() {

            if raw[index] == ' ' as u16 {
                indent += 1;
            }

            else if raw[index] == '\t' as u16 {
                indent += 4;
            }

            else {
                break;
            }

            index += 1;
        }

        Line {
            content: raw[index..].to_vec(),
            indent
        }

    }

    pub fn to_raw(&self) -> Vec<u16> {
        vec![vec![' ' as u16; self.indent], self.content.clone()].concat()
    }

    #[cfg(test)]
    pub fn from_raw_string(string: &str) -> Line {
        Line::from_raw(&into_v16(string))
    }

}


pub fn code_to_lines(code: &[u16]) -> Vec<Line> {

    code.split(
        |c| *c == '\n' as u16
    ).map(
        |ln| Line::from_raw(&ln.to_vec())
    ).collect::<Vec<Line>>()
}

pub fn add_br_if_needed(line: &[u16]) -> Vec<u16> {
    #[cfg(test)] {
        push_call_stack("add_br_if_needed", &from_v16(&line));
        pop_call_stack();
    }

    if line.len() > 1 && line[line.len() - 1] == '\\' as u16 {
        vec![
            line[0..(line.len() - 1)].to_vec(),
            into_v16("[[br]]")  // will later be converted to `<br/>`
        ].concat()
    }

    else if line.len() > 2 && line[line.len() - 1] == ' ' as u16 && line[line.len() - 2] == ' ' as u16 {
        vec![
            line[0..(line.len() - 2)].to_vec(),
            into_v16("[[br]]")  // will later be converted to `<br/>`
        ].concat()
    }

    else {
        line.to_vec()
    }

}