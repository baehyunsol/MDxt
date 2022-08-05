use crate::container::codefence::FencedCode;
use crate::inline::footnote::Footnote;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DocData {
    pub headers: Vec<(usize, Vec<u16>)>,  // (level, content)
    pub has_math: bool,
    pub link_references: HashMap<Vec<u16>, Vec<u16>>,  // (label, destination)
    pub footnote_references: HashMap<Vec<u16>, Footnote>,  // (label, footnote)
    footnote_reference_count: usize,
    pub has_toc: bool,
    pub has_collapsible_table: bool,
    pub fenced_code_contents: HashMap<usize, Vec<u16>>  // HashMap<index, content>
}

impl Default for DocData {

    fn default() -> Self {
        DocData {
            headers: vec![],
            has_math: false,
            link_references: HashMap::new(),
            footnote_references: HashMap::new(),
            footnote_reference_count: 0,
            has_toc: false,
            has_collapsible_table: false,
            fenced_code_contents: HashMap::new()
        }
    }

}

impl DocData {

    pub fn new(
        headers: Vec<(usize, Vec<u16>)>,
        link_references: HashMap<Vec<u16>, Vec<u16>>,
        footnote_references: HashMap<Vec<u16>, Footnote>
    ) -> Self {
        DocData { headers, link_references, footnote_references, .. Self::default() }
    }

    pub fn add_footnote_inverse_index(&mut self, label: &Vec<u16>) -> usize {
        let footnote = self.footnote_references.get_mut(label).unwrap();
        footnote.inverse_index.push(self.footnote_reference_count);
        self.footnote_reference_count += 1;

        self.footnote_reference_count - 1
    }

    pub fn add_fenced_code_content(&mut self, fenced_code: &FencedCode) {

        if fenced_code.copy_button {
            self.fenced_code_contents.insert(fenced_code.index, fenced_code.get_raw_content());
        }

    }

}