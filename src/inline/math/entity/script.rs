use super::{Entity, vec_to_math_ml};
use crate::utils::into_v16;

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

    pub fn to_math_ml(&self) -> Vec<u16> {

        if self.pre_sup.len() == 0 && self.pre_sub.len() == 0 {

            if self.post_sup.len() == 0 {
                vec![
                    into_v16("<msub>"),
                    vec_to_math_ml(&self.content, true),
                    vec_to_math_ml(&self.post_sub, true),
                    into_v16("</msub>")
                ].concat()
            }

            else if self.post_sub.len() == 0 {
                vec![
                    into_v16("<msup>"),
                    vec_to_math_ml(&self.content, true),
                    vec_to_math_ml(&self.post_sup, true),
                    into_v16("</msup>")
                ].concat()
            }

            else {
                vec![
                    into_v16("<msubsup>"),
                    vec_to_math_ml(&self.content, true),
                    vec_to_math_ml(&self.post_sub, true),
                    vec_to_math_ml(&self.post_sup, true),
                    into_v16("</msubsup>")
                ].concat()
            }

        }

        else {
            vec![
                into_v16("<mmultiscripts>"),
                vec_to_math_ml(&self.content, true),
                script_or_none(&self.post_sub),
                script_or_none(&self.post_sup),
                into_v16("<mprescripts/>"),
                script_or_none(&self.pre_sub),
                script_or_none(&self.pre_sup),
                into_v16("</mmultiscripts>")
            ].concat()
        }

    }

}

fn script_or_none(vec: &Vec<Entity>) -> Vec<u16> {

    if vec.len() == 0 {
        into_v16("<none/>")
    }

    else {
        vec_to_math_ml(vec, true)
    }

}