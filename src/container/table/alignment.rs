use crate::ast::line::Line;
use crate::utils::{strip_whitespaces, into_v16};

#[derive(Clone)]
pub enum TableAlignment {
    Left, Center, Right, None
}

impl TableAlignment {

    pub fn opening_tag(&self, class_prefix: &str) -> Vec<u16> {
        match self {
            TableAlignment::Left => into_v16(&format!("<div class=\"{}align-left\">", class_prefix)),
            TableAlignment::Center => into_v16(&format!("<div class=\"{}align-center\">", class_prefix)),
            TableAlignment::Right => into_v16(&format!("<div class=\"{}align-right\">", class_prefix)),
            TableAlignment::None => vec![]
        }
    }

    pub fn closing_tag(&self) -> Vec<u16> {
        match self {
            TableAlignment::None => vec![],
            _ => into_v16("</div>")
        }
    }

}

pub fn parse_alignments(line: &Line) -> Vec<TableAlignment> {
    let cells = line.content.split(|c| *c == '|' as u16).collect::<Vec<&[u16]>>();

    // the first and the last elements are due to the trailing and leading pipes
    // they should be removed
    let cells = &cells[1..cells.len() - 1];
    
    cells.iter().map(
        |c| {
            let c = strip_whitespaces(c);

            if c[0] == ':' as u16 && c[c.len() - 1] == ':' as u16 {
                TableAlignment::Center
            }

            else if c[c.len() - 1] == ':' as u16 {
                TableAlignment::Right
            }

            else if c[0] == ':' as u16 {
                TableAlignment::Left
            }

            else {
                TableAlignment::None
            }
        }
    ).collect()
}