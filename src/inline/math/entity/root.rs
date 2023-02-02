use super::{Entity, vec_to_math_ml};
use crate::utils::into_v32;

#[derive(Clone)]
pub struct Root {
    index: Vec<Entity>,
    content: Vec<Entity>
}

impl Root {

    pub fn new(index: Vec<Entity>, content: Vec<Entity>) -> Self {
        Root { index, content }
    }

    pub fn to_math_ml(&self) -> Vec<u32> {

        if self.index.len() == 0 {
            vec![
                into_v32("<msqrt>"),
                vec_to_math_ml(&self.content, false),  // <msqrt> has several children, `single_element` doesn't have to be true.
                into_v32("</msqrt>")
            ].concat()
        }

        else {
            vec![
                into_v32("<mroot>"),
                vec_to_math_ml(&self.content, true),  // `<mroot>` has exactly two children, so `single_element` is true.
                vec_to_math_ml(&self.index, true),
                into_v32("</mroot>")
            ].concat()
        }

    }

}