pub mod predicate;

#[cfg(test)]
mod testbench;

use crate::utils::into_v16;
use super::InlineNode;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Footnote {
    pub index: usize,
    pub inverse_index: Vec<usize>,
    pub content: InlineNode
}

pub fn footnotes_to_html(footnotes: &mut HashMap<Vec<u16>, Footnote>, toc_rendered: &[u16], class_prefix: &str) -> Vec<u16> {

    let mut notes = footnotes.values().map(|x| x.clone()).collect::<Vec<Footnote>>();
    notes.sort_unstable_by_key(|Footnote { index, .. }| *index);

    let mut result = Vec::with_capacity(notes.len());

    result.push(into_v16(&format!("<hr class=\"{}footnote-hr\"/><p>", class_prefix)));

    for Footnote {index, inverse_index, content} in notes.into_iter() {

        if inverse_index.len() == 0 {
            continue;
        }

        let inverse_indexes = inverse_index.iter().map(
            |ind|
            into_v16(&format!("<a href=\"#footnote-ref-{}\"> [{}]</a> ", ind, ind))
        ).collect::<Vec<Vec<u16>>>().concat();

        result.push(
            vec![
                into_v16(&format!("<a id=\"footnote-cite-{}\"></a>", index)),
                into_v16(&format!("{}. ", index)),
                inverse_indexes,
                content.to_html(toc_rendered, class_prefix),
                into_v16("<br/>")
            ].concat()
        );
    }

    result.push(into_v16("</p>"));

    result.concat()
}