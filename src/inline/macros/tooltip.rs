use crate::ast::doc_data::DocData;
use crate::inline::{footnote::Footnote, InlineNode};
use crate::RenderOption;
use crate::utils::{from_v16, into_v16};

// Error message to show when referencing a tooltip that doesn't exist
pub fn dummy_tooltip(label: &[u16]) -> Footnote {
    Footnote {
        index: 0,
        inverse_index: vec![],
        content: InlineNode::Raw(into_v16(&format!("Error! Undefined tooltip label: {}", from_v16(label))))
    }
}

pub fn load_tooltip_message(label: &[u16], doc_data: &mut DocData, render_option: &RenderOption) -> Vec<Box<InlineNode>> {
    let label_key = vec![into_v16("^"), label.to_vec()].concat();
    let mut message = match doc_data.footnote_references.get(&label_key) {
        Some(f) => f.clone(),
        None => dummy_tooltip(&label),  // print error message: "Error! Undefined tooltip label: {}"
    };

    message.content.parse_raw(doc_data, render_option);

    message.content.clone().to_vec()
}

pub fn tooltip_javascript() -> String {
"let tooltips = document.querySelectorAll(\".tooltip-container\");

for (let i = 0; i < tooltips.length; i++) {
    let child = document.getElementById(\"tooltip-message-\" + i);

    document.getElementById(\"tooltip-container-\" + i).addEventListener(\"mousemove\", e => {

        if (e.clientX + child.clientWidth > window.innerWidth) {
            child.style.left = e.clientX - child.clientWidth + \"px\";
        }

        else {
            child.style.left = e.clientX + \"px\";
        }

        if (e.clientY < child.clientHeight + 8) {
            child.style.top = e.clientY + 8 + \"px\";
        }

        else {
            child.style.top = (e.clientY - child.clientHeight - 8) + \"px\";
        }

    });
}".to_string()
}