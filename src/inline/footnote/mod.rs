pub mod predicate;

#[cfg(test)]
mod testbench;

use super::InlineNode;
use crate::utils::into_v32;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Footnote {
    pub index: usize,
    pub inverse_index: Vec<usize>,
    pub content: InlineNode
}

pub fn footnotes_to_html(footnotes: &mut HashMap<Vec<u32>, Footnote>, toc_rendered: &[u32], class_prefix: &str) -> Vec<u32> {

    let notes = footnotes.values().map(|x| x.clone()).collect::<Vec<Footnote>>();

    let mut result = Vec::with_capacity(3);
    let mut footnote_cites: Vec<(Vec<u32>, usize)> = Vec::with_capacity(notes.len());

    result.push(into_v32(&format!("<hr class=\"{class_prefix}footnote-hr\"/><div class=\"{class_prefix}mdxt-footnote-cites\"><p>")));

    for Footnote {index, inverse_index, content} in notes.into_iter() {

        if inverse_index.len() == 0 {
            continue;
        }

        #[cfg(test)]
        assert_eq!(inverse_index, {let mut ii = inverse_index.clone(); ii.sort(); ii});

        let inverse_indexes = inverse_index.iter().map(
            |ind|
            into_v32(&format!("<a href=\"#footnote-ref-{ind}\"> [{ind}]</a> "))
        ).collect::<Vec<Vec<u32>>>().concat();

        footnote_cites.push((
            vec![
                into_v32(&format!("<div class=\"footnote-cite\"><a id=\"footnote-cite-{index}\"></a>")),
                inverse_indexes,
                content.to_html(toc_rendered, class_prefix),
                into_v32("</div>")
            ].concat(),
            inverse_index[0]
        ));
    }

    // footnotes are defined, but none of them are used
    // in this case, nothing has to be rendered
    if footnote_cites.len() == 0 {
        result.pop();
    }

    else {
        footnote_cites.sort_unstable_by_key(|(_, i)| *i);
        result.push(footnote_cites.into_iter().map(|(c, _)| c).collect::<Vec<Vec<u32>>>().concat());

        result.push(into_v32("</p></div>"));
    }

    result.concat()
}