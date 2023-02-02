mod render;

#[cfg(test)]
mod testbench;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ICONS: HashMap<Vec<u32>, (Vec<u32>, usize)> = self::render::data();
}

pub fn get_icon(name: &Vec<u32>, size: usize, color: Option<(u8, u8, u8)>, standalone: bool) -> Option<Vec<u32>> {

    match ICONS.get(name) {
        Some((svg, _)) => Some(self::render::format(svg, size, color, standalone)),
        _ => None
    }

}