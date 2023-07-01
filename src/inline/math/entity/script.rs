use super::{Entity, vec_to_math_ml};
use crate::utils::into_v32;

#[derive(Clone)]
pub struct Script {
    content: Vec<Entity>,
    pre_sup: Vec<Entity>,
    post_sup: Vec<Entity>,
    pre_sub: Vec<Entity>,
    post_sub: Vec<Entity>
}

impl Script {

    pub fn new(
        content: Vec<Entity>,
        pre_sup: Vec<Entity>,
        post_sup: Vec<Entity>,
        pre_sub: Vec<Entity>,
        post_sub: Vec<Entity>
    ) -> Self {
        Script { content, pre_sup, post_sup, pre_sub, post_sub }
    }

    pub fn to_math_ml(&self) -> Vec<u32> {

        if self.pre_sup.is_empty() && self.pre_sub.is_empty() {

            if self.post_sup.is_empty() {
                vec![
                    vec![60, 109, 115, 117, 98, 62],  // into_v32("<msub>")
                    vec_to_math_ml(&self.content, true),
                    vec_to_math_ml(&self.post_sub, true),
                    vec![60, 47, 109, 115, 117, 98, 62],  // into_v32("</msub>")
                ].concat()
            }

            else if self.post_sub.is_empty() {
                vec![
                    vec![60, 109, 115, 117, 112, 62],  // into_v32("<msup>")
                    vec_to_math_ml(&self.content, true),
                    vec_to_math_ml(&self.post_sup, true),
                    vec![60, 47, 109, 115, 117, 112, 62],  // into_v32("</msup>")
                ].concat()
            }

            else {
                vec![
                    vec![60, 109, 115, 117, 98, 115, 117, 112, 62],  // into_v32("<msubsup>")
                    vec_to_math_ml(&self.content, true),
                    vec_to_math_ml(&self.post_sub, true),
                    vec_to_math_ml(&self.post_sup, true),
                    vec![60, 47, 109, 115, 117, 98, 115, 117, 112, 62],  // into_v32("</msubsup>")
                ].concat()
            }

        }

        else {
            vec![
                into_v32("<mmultiscripts>"),
                vec_to_math_ml(&self.content, true),
                script_or_none(&self.post_sub),
                script_or_none(&self.post_sup),
                into_v32("<mprescripts/>"),
                script_or_none(&self.pre_sub),
                script_or_none(&self.pre_sup),
                into_v32("</mmultiscripts>")
            ].concat()
        }

    }

}

fn script_or_none(vec: &Vec<Entity>) -> Vec<u32> {

    if vec.is_empty() {
        vec![60, 110, 111, 110, 101, 47, 62]  // into_v32("<none/>")
    }

    else {
        vec_to_math_ml(vec, true)
    }

}