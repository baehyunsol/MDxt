use super::predicate::*;
use super::{InlineNode, DecorationType};

impl InlineNode {

    pub fn from_md(content: &[u16]) -> Self {

        let mut index = 0;
        let mut result: Vec<Box<InlineNode>> = vec![];

        while index < content.len() {

            match is_code_span(content, index) {
                Bool::True(end) => {
                    return InlineNode::Complex(
                        vec![
                            Box::new(InlineNode::Raw(content[0..index].to_vec())),
                            Box::new(InlineNode::CodeSpan(content[index + 1..end].to_vec())),  // inline elements inside a codespan are not rendered
                            Box::new(Self::from_md(&content[end + 1..content.len() - 1]))
                        ]
                    );
                },
                _ => {}
            }

            match is_italic(content, index) {
                Bool::True(end) => {
                    return InlineNode::Complex(
                        vec![
                            Box::new(InlineNode::Raw(content[0..index].to_vec())),
                            Box::new(InlineNode::Decoration {
                                deco_type: DecorationType::Italic,

                                // `Self::from_md` always returns `InlineNode::Raw` or `InlineNode::Complex`, both of which can be converted to a Vec<Box<InlineNode>>
                                content: Self::from_md(&content[index + 1..end]).to_vec()
                            }),
                            Box::new(Self::from_md(&content[end + 1..content.len() - 1]))
                        ]
                    );
                },
                _ => {}
            }

            index += 1;
        }

        // there're no inline elements in the content
        InlineNode::Raw(content.to_vec())
    }

}