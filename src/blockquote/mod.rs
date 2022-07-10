use crate::render::render_option::RenderOption;
use crate::ast::MdData;
use crate::ast::line::Line;

pub struct Blockquote {}

impl Blockquote {

    pub fn to_html(&self) -> Vec<u16> {
        todo!()
    }

    pub fn parse_inlines(&mut self, md_data: &mut MdData, options: &RenderOption) {
        todo!()
    }

    pub fn from_lines(lines: &Vec<Line>) -> Self {
        todo!()
    }

}