use std::collections::HashMap;
use crate::inline::footnote::Footnote;

#[derive(Clone)]
pub struct DocData {
    headers: Vec<(usize, Vec<u16>)>,  // (level, content)
    pub has_math: bool,
    pub link_references: HashMap<Vec<u16>, Vec<u16>>,  // (label, destination)
    pub footnote_references: HashMap<Vec<u16>, Footnote>,  // (label, footnote)
    footnote_reference_count: usize,
    pub has_collapsible_table: bool
}

impl Default for DocData {

    fn default() -> Self {
        DocData {
            headers: vec![],
            has_math: false,
            link_references: HashMap::new(),
            footnote_references: HashMap::new(),
            footnote_reference_count: 0,
            has_collapsible_table: false
        }
    }

}

impl DocData {

    pub fn new(
        headers: Vec<(usize, Vec<u16>)>,
        link_references: HashMap<Vec<u16>, Vec<u16>>,
        footnote_references: HashMap<Vec<u16>, Footnote>
    ) -> Self {
        DocData { headers, link_references, footnote_references, footnote_reference_count: 0, has_math: false, has_collapsible_table: false }
    }

    pub fn add_footnote_inverse_index(&mut self, label: &Vec<u16>) -> usize {
        let footnote = self.footnote_references.get_mut(label).unwrap();
        footnote.inverse_index.push(self.footnote_reference_count);
        self.footnote_reference_count += 1;

        self.footnote_reference_count - 1
    }

}