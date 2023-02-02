use crate::ast::line::Line;
use crate::utils::{strip_whitespaces, into_v32};

#[derive(Clone)]
pub enum TableAlignment {
    Left, Center, Right, None
}

impl TableAlignment {

    pub fn opening_tag(&self, class_prefix: &str) -> Vec<u32> {
        match self {
            TableAlignment::Left => into_v32(&format!("<div class=\"{}align-left\">", class_prefix)),
            TableAlignment::Center => into_v32(&format!("<div class=\"{}align-center\">", class_prefix)),
            TableAlignment::Right => into_v32(&format!("<div class=\"{}align-right\">", class_prefix)),
            TableAlignment::None => vec![]
        }
    }

    pub fn closing_tag(&self) -> Vec<u32> {
        match self {
            TableAlignment::None => vec![],
            _ => into_v32("</div>")
        }
    }

}

pub fn parse_alignments(line: &Line) -> Vec<TableAlignment> {
    let cells = line.content.split(|c| *c == '|' as u32).collect::<Vec<&[u32]>>();

    // the first and the last elements are due to the trailing and leading pipes
    // they should be removed
    let cells = &cells[1..cells.len() - 1];
    
    cells.iter().map(
        |c| {
            let c = strip_whitespaces(c);

            if c[0] == ':' as u32 && c[c.len() - 1] == ':' as u32 {
                TableAlignment::Center
            }

            else if c[c.len() - 1] == ':' as u32 {
                TableAlignment::Right
            }

            else if c[0] == ':' as u32 {
                TableAlignment::Left
            }

            else {
                TableAlignment::None
            }
        }
    ).collect()
}