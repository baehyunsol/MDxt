use crate::inline::InlineNode;

pub struct List {
    elements: Vec<Element>
}

pub struct Element {
    level: usize,
    content: InlineNode
}