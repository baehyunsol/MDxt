use super::{Entity, vec_to_math_ml};

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

        if self.index.is_empty() {
            vec![
                vec![60, 109, 115, 113, 114, 116, 62],  // into_v32("<msqrt>")
                vec_to_math_ml(&self.content, false),  // <msqrt> has several children, `single_element` doesn't have to be true.
                vec![60, 47, 109, 115, 113, 114, 116, 62],  // into_v32("</msqrt>")
            ].concat()
        }

        else {
            vec![
                vec![60, 109, 114, 111, 111, 116, 62],  // into_v32("<mroot>")
                vec_to_math_ml(&self.content, true),  // `<mroot>` has exactly two children, so `single_element` is true.
                vec_to_math_ml(&self.index, true),
                vec![60, 47, 109, 114, 111, 111, 116, 62],  // into_v32("</mroot>")
            ].concat()
        }

    }

}