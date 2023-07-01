#[cfg(test)]
mod testbench;

use crate::ast::line::Line;
use crate::escape::{undo_backslash_escapes, undo_html_escapes};
use crate::utils::{from_v32, into_v32, strip_whitespaces};
use yaml_rust::{Yaml, YamlLoader};

pub fn parse_metadata(lines: &Vec<Line>) -> Option<(Yaml, usize)> {  // Option<(metadata, end_index)>

    if lines.len() < 3 {
        return None;
    }

    let yaml_delim = into_v32("---");

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
        let mut yaml = lines[0..index].iter().map(|line| line.to_raw()).collect::<Vec<Vec<u32>>>().join(&['\n' as u32][..]);
        yaml = undo_backslash_escapes(&undo_html_escapes(&yaml));

        match YamlLoader::load_from_str(&from_v32(&yaml)) {
            Ok(data) if !data.is_empty() => Some((data[0].clone(), index)),
            _ => None
        }

    }

    else {
        None
    }

}