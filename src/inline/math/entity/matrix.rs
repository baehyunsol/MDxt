use super::super::{md_to_math, parse::get_arguments};
use super::Entity;
use crate::utils::from_v32;

#[derive(Clone)]
pub struct Matrix {
    elements: Vec<Vec<Vec<Entity>>>,
}

impl Matrix {
    pub fn new(elements: Vec<Vec<Vec<Entity>>>) -> Self {
        Matrix { elements }
    }

    pub fn to_math_ml(&self) -> Vec<u32> {
        let mut result = vec![];
        result.push(vec![60, 109, 116, 97, 98, 108, 101, 62]);  // into_v32("<mtable>")

        for row in self.elements.iter() {
            result.push(vec![60, 109, 116, 114, 62]);  // into_v32("<mtr>")

            for element in row.iter() {
                result.push(vec![60, 109, 116, 100, 62]);  // into_v32("<mtd>")

                for entity in element.iter() {
                    result.push(entity.to_math_ml());
                }

                result.push(vec![60, 47, 109, 116, 100, 62]);  // into_v32("</mtd>")
            }

            result.push(vec![60, 47, 109, 116, 114, 62]);  // into_v32("</mtr>")
        }

        result.push(vec![60, 47, 109, 116, 97, 98, 108, 101, 62]);  // into_v32("</mtable>")

        result.concat()
    }
}

pub fn try_parse_matrix(arguments: &Vec<Vec<u32>>) -> Result<Vec<Vec<Vec<Entity>>>, String> {
    let mut result = vec![];
    let mut cols = usize::MAX;

    for row in arguments.iter() {
        let (elements, end_index) = get_arguments(row, 0);

        // TODO: make sure that there's no extra stuff after `end_index`

        if let Some(e) = find_extra_stuff(row, end_index) {
            return Err(format!("Unexpected input: {}", from_v32(&e)));
        }

        if cols == usize::MAX {
            cols = elements.len();

            if cols == 0 {
                return Err(String::from("Empty Row"));
            }
        } else if cols != elements.len() {
            return Err(format!("Expected {cols} elements, but got {} elements", elements.len()));
        }

        result.push(elements.iter().map(|el| md_to_math(el)).collect());
    }

    Ok(result)
}

fn find_extra_stuff(r: &[u32], mut index: usize) -> Option<Vec<u32>> {
    while let Some(c) = r.get(index) {
        if *c == '}' as u32 {
            return None;
        }

        if *c != ' ' as u32 && *c != '\n' as u32 {
            return Some(r[index..].to_vec());
        }

        index += 1;
    }

    None
}
