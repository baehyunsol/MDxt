#[cfg(test)]
mod testbench;

use crate::ast::line::{Line, to_raw};
use crate::escape::{undo_backslash_escapes, undo_html_escapes};
use crate::utils::{from_v16, into_v16, strip_whitespaces};
use yaml_rust::{Yaml, YamlLoader};

pub fn parse_metadata(lines: &Vec<Line>) -> Option<(Yaml, usize)> {  // Option<(metadata, end_index)>

    if lines.len() < 3 {
        return None;
    }

    let yaml_delim = into_v16("---");

    if strip_whitespaces(&lines[0].content) != yaml_delim {
        return None;
    }

    let mut index = 1;

    while index < lines.len() {

        if strip_whitespaces(&lines[index].content) == yaml_delim {
            break;
        }

        index += 1;
    }

    if index != lines.len() {
        let mut yaml = lines[0..index].iter().map(to_raw).collect::<Vec<Vec<u16>>>().join(&['\n' as u16][..]);
        yaml = undo_backslash_escapes(&undo_html_escapes(&yaml));

        match YamlLoader::load_from_str(&from_v16(&yaml)) {
            Ok(data) if data.len() > 0 => Some((data[0].clone(), index)),
            _ => None
        }

    }

    else {
        None
    }

}