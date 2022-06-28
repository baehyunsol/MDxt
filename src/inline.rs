mod predicate;
mod parse;

#[cfg(test)]
mod testbench;

use crate::utils::into_v16;

pub enum InlineNode {
    Raw(Vec<u16>),
    Complex(Vec<Box<InlineNode>>),
    CodeSpan(Vec<u16>),
    Decoration {
        deco_type: DecorationType,
        content: Vec<Box<InlineNode>>
    }
}

pub enum DecorationType {
    Bold, Italic, Underline, Deletion, Subscript, Superscript,
    Macro(InlineMacro)
}

pub enum InlineMacro {
    Align,
    Color,
    Size,
    SpecialCharacter(u16),
    Svg {
        name: Vec<u16>,
        size: u16
    }
}

impl InlineNode {

    pub fn to_html(&self) -> Vec<u16> {
        match self {
            InlineNode::Raw(content) => content.clone(),

            InlineNode::CodeSpan(content) => vec![
                into_v16("<code class=\"short\">"),
                content.clone(),
                into_v16("</code>")
            ].concat(),

            InlineNode::Complex(content) => content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),

            InlineNode::Decoration {deco_type, content} => match deco_type {
                DecorationType::Italic => vec![
                    into_v16("<em>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</em>")
                ].concat(),
                DecorationType::Bold => vec![
                    into_v16("<strong>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</strong>")
                ].concat(),
                DecorationType::Underline => vec![
                    into_v16("<u>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</u>")
                ].concat(),
                DecorationType::Deletion => vec![
                    into_v16("<del>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</del>")
                ].concat(),
                DecorationType::Subscript => vec![
                    into_v16("<sub>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</sub>")
                ].concat(),
                DecorationType::Superscript => vec![
                    into_v16("<sup>"),
                    content.iter().map(|node| node.to_html()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("</sup>")
                ].concat(),
                DecorationType::Macro(_) => todo!()
            }
        }
    }

    pub fn to_md(&self) -> Vec<u16> {
        match self {
            InlineNode::Raw(content) => content.clone(),

            InlineNode::CodeSpan(content) => vec![
                into_v16("`"),
                content.clone(),
                into_v16("`")
            ].concat(),

            InlineNode::Complex(content) => content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),

            InlineNode::Decoration {deco_type, content} => match deco_type {
                DecorationType::Italic => vec![
                    into_v16("*"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("*")
                ].concat(),
                DecorationType::Bold => vec![
                    into_v16("**"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("**")
                ].concat(),
                DecorationType::Underline => vec![
                    into_v16("~_"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("_~")
                ].concat(),
                DecorationType::Deletion => vec![
                    into_v16("~~"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("~~")
                ].concat(),
                DecorationType::Subscript => vec![
                    into_v16("~"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("~")
                ].concat(),
                DecorationType::Superscript => vec![
                    into_v16("^"),
                    content.iter().map(|node| node.to_md()).collect::<Vec<Vec<u16>>>().concat(),
                    into_v16("^")
                ].concat(),
                DecorationType::Macro(_) => todo!()
            }
        }
    }

    fn to_vec(self) -> Vec<Box<InlineNode>> {

        match self {
            InlineNode::Raw(_) => vec![Box::new(self)],
            InlineNode::Complex(vec) => vec,
            _ => panic!("oh no!")
        }

    }

}

const INLINE_CODESPAN_MARKER1: u16 = u16::MAX - 1999;
const INLINE_CODESPAN_MARKER2: u16 = u16::MAX - 1998;
const INLINE_CODESPAN_MARKER3: u16 = u16::MAX - 1997;
const INLINE_CODESPAN_MARKER4: u16 = u16::MAX - 1996;