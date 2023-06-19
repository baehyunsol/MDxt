use super::{Entity, vec_to_math_ml};
use crate::utils::into_v32;

#[derive(Clone)]
pub struct UnderOver {
    content: Vec<Entity>,
    under: Vec<Entity>,
    over: Vec<Entity>,
    display_style: bool  // makes summation and integration signs bigger
}

impl UnderOver {

    pub fn new(content: Vec<Entity>, under: Vec<Entity>, over: Vec<Entity>, display_style: bool) -> Self {
        UnderOver { content, under, over, display_style }
    }

    pub fn to_math_ml(&self) -> Vec<u32> {

        let display_style = if self.display_style {
            " displaystyle=\"true\""
        } else {
            ""
        };

        if self.under.len() == 0 {
            vec![
                into_v32(&format!("<mover{display_style}>")),
                vec_to_math_ml(&self.content, true),  // `<mover>` has exactly two children, so `single_element` is true.
                vec_to_math_ml(&self.over, true),
                vec![60, 47, 109, 111, 118, 101, 114, 62],  // into_v32("</mover>")
            ].concat()
        }

        else if self.over.len() == 0 {
            vec![
                into_v32(&format!("<munder{display_style}>")),
                vec_to_math_ml(&self.content, true),  // `<munder>` has exactly two children, so `single_element` is true.
                vec_to_math_ml(&self.under, true),
                vec![60, 47, 109, 117, 110, 100, 101, 114, 62],  // into_v32("</munder>")
            ].concat()
        }

        else {
            vec![
                into_v32(&format!("<munderover{display_style}>")),
                vec_to_math_ml(&self.content, true),  // `<munderover>` has exactly three children, so `single_element` is true.
                vec_to_math_ml(&self.under, true),
                vec_to_math_ml(&self.over, true),
                vec![60, 47, 109, 117, 110, 100, 101, 114, 111, 118, 101, 114, 62],  // into_v32("</munderover>")
            ].concat()
        }

    }

}