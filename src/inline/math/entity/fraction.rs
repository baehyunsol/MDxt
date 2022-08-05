use super::{Entity, vec_to_math_ml};
use crate::utils::into_v16;

#[derive(Clone)]
pub struct Fraction {
    numer: Vec<Entity>,
    denom: Vec<Entity>,
    display_style: bool,  // true for cfrac
    no_line: bool
}

impl Fraction {

    pub fn new(numer: Vec<Entity>, denom: Vec<Entity>, display_style: bool, no_line: bool) -> Self {
        Fraction { numer, denom, display_style, no_line }
    }

    pub fn to_math_ml(&self) -> Vec<u16> {
        let opening = format!(
            "<mfrac displaystyle=\"{}\"{}>",
            self.display_style,
            if self.no_line {
                " linethickness=\"0\""
            } else {
                ""
            }
        );

        let opening = into_v16(&opening);

        vec![
            opening,
            vec_to_math_ml(&self.numer, true),  // `<mfrac>` has exactly two children, so `single_element` is true.
            vec_to_math_ml(&self.denom, true),
            into_v16("</mfrac>")
        ].concat()
    }

}