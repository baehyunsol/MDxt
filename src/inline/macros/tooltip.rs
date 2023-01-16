use crate::inline::{footnote::Footnote, InlineNode};
use crate::utils::{from_v16, into_v16};

// Error message to show when referencing a tooltip that doesn't exist
pub fn dummy_tooltip(label: &[u16]) -> Footnote {
    Footnote {
        index: 0,
        inverse_index: vec![],
        content: InlineNode::Raw(into_v16(&format!("Error! Undefined tooltip label: {}", from_v16(label))))
    }
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