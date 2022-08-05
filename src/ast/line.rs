use crate::utils::into_v16;

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

}


pub fn code_to_lines(code: &[u16]) -> Vec<Line> {

    code.split(
        |c| *c == '\n' as u16
    ).map(
        |ln| Line::from_raw(&ln.to_vec())
    ).collect::<Vec<Line>>()
}

pub fn to_raw(line: &Line) -> Vec<u16> {
    vec![vec![' ' as u16; line.indent], line.content.clone()].concat()
}

pub fn add_br_if_needed(line: &Line) -> Vec<u16> {

    if line.content.len() > 1 && line.content[line.content.len() - 1] == '\\' as u16 {
        vec![
            line.content[0..(line.content.len() - 1)].to_vec(),
            into_v16("[[br]]")  // will later be converted to `<br/>`
        ].concat()
    }

    else if line.content.len() > 2 && line.content[line.content.len() - 1] == ' ' as u16 && line.content[line.content.len() - 2] == ' ' as u16 {
        vec![
            line.content[0..(line.content.len() - 2)].to_vec(),
            into_v16("[[br]]")  // will later be converted to `<br/>`
        ].concat()
    }

    else {
        line.content.clone()
    }

}