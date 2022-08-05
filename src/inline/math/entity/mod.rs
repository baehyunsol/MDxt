mod fraction;
mod root;
mod script;
mod underover;

use crate::utils::{into_v16, is_alphabet, is_numeric};
use fraction::Fraction;
use root::Root;
use script::Script;
use underover::UnderOver;

#[derive(Clone)]
pub enum Entity {
    Root(Box<Root>),
    Fraction(Box<Fraction>),
    Script(Box<Script>),
    UnderOver(Box<UnderOver>),
    String(Vec<u16>),
    RawString(Vec<u16>),
    Character(u16)
}

impl Entity {

    pub fn new_root(index: Vec<Entity>, content: Vec<Entity>) -> Self {
        Entity::Root(Box::new(Root::new(index, content)))
    }

    pub fn new_fraction(numer: Vec<Entity>, denom: Vec<Entity>, display_style: bool, no_line: bool) -> Self {
        Entity::Fraction(Box::new(Fraction::new(numer, denom, display_style, no_line)))
    }

    pub fn new_script(
        content: Vec<Entity>,
        pre_sup: Vec<Entity>,
        post_sup: Vec<Entity>,
        pre_sub: Vec<Entity>,
        post_sub: Vec<Entity>
    ) -> Self {
        Entity::Script(Box::new(Script::new(content, pre_sup, post_sup, pre_sub, post_sub)))
    }

    pub fn new_underover(content: Vec<Entity>, under: Vec<Entity>, over: Vec<Entity>, display_style: bool) -> Self {
        Entity::UnderOver(Box::new(UnderOver::new(content, under, over, display_style)))
    }

    pub fn new_character(character: u16) -> Self {
        Entity::Character(character)
    }

    pub fn new_string(string: &str) -> Self {
        Entity::String(into_v16(string))
    }

    // it always returns a single element
    // e.g) 
    // <mrow><mn>1</mn><mo>+</mo><mn>2</mn></mrow>
    // instead of
    // <mn>1</mn><mo>+</mo><mn>2</mn>
    pub fn to_math_ml(&self) -> Vec<u16> {

        match self {
            Entity::Root(root) => root.to_math_ml(),
            Entity::Fraction(fraction) => fraction.to_math_ml(),
            Entity::UnderOver(underover) => underover.to_math_ml(),
            Entity::Script(script) => script.to_math_ml(),
            Entity::Character(character) => into_v16(&format!("<mo>&#{};</mo>", character)),
            Entity::RawString(string) => vec![
                into_v16("<mtext>"),
                string.clone(),
                into_v16("</mtext>"),
            ].concat(),
            Entity::String(string) => to_math_ml_string(string),
            _ => todo!()
        }

    }

}

#[derive(PartialEq)]
enum StringState {
    Identifier,  // <mi>
    Number,      // <mn>
    Operand,     // <mo>
}

fn get_string_state(character: &u16) -> StringState {

    if is_alphabet(character) {
        StringState::Identifier
    }

    else if is_numeric(character) {
        StringState::Number
    }

    else {
        StringState::Operand
    }

}

pub fn to_math_ml_string(string: &[u16]) -> Vec<u16> {

    if string.len() == 0 {
        into_v16("<mrow></mrow>")
    }

    else {
        let mut curr_state = get_string_state(&string[0]);

        let mut last_index = 0;
        let mut has_multiple_states = false;
        let mut result = vec![];

        for (curr_index, c) in string.iter().enumerate() {

            match curr_state {
                StringState::Identifier if !is_alphabet(c) => {
                    result.push(vec![
                        into_v16("<mi>"),
                        string[last_index..curr_index].to_vec(),
                        into_v16("</mi>"),
                    ].concat());
                    last_index = curr_index;
                    curr_state = get_string_state(c);
                    has_multiple_states = true;
                },
                StringState::Number if !is_numeric(c) && *c != '.' as u16 => {
                    result.push(vec![
                        into_v16("<mn>"),
                        string[last_index..curr_index].to_vec(),
                        into_v16("</mn>"),
                    ].concat());
                    last_index = curr_index;
                    curr_state = get_string_state(c);
                    has_multiple_states = true;
                },
                StringState::Operand if is_alphabet(c) || is_numeric(c) => {
                    result.push(vec![
                        into_v16("<mo>"),
                        string[last_index..curr_index].to_vec(),
                        into_v16("</mo>"),
                    ].concat());
                    last_index = curr_index;
                    curr_state = get_string_state(c);
                    has_multiple_states = true;
                },
                _ => {}
            }

        }

        if last_index < string.len() {

            match curr_state {
                StringState::Identifier => {
                    result.push(vec![
                        into_v16("<mi>"),
                        string[last_index..].to_vec(),
                        into_v16("</mi>"),
                    ].concat());
                }
                StringState::Number => {
                    result.push(vec![
                        into_v16("<mn>"),
                        string[last_index..].to_vec(),
                        into_v16("</mn>"),
                    ].concat());
                }
                StringState::Operand => {
                    result.push(vec![
                        into_v16("<mo>"),
                        string[last_index..].to_vec(),
                        into_v16("</mo>"),
                    ].concat());
                }
            }

        }

        if has_multiple_states {
            vec![
                into_v16("<mrow>"),
                result.concat(),
                into_v16("</mrow>"),
            ].concat()
        }

        else {
            result.concat()
        }

    }

}

pub fn vec_to_math_ml(vec: &Vec<Entity>, single_element: bool) -> Vec<u16> {
    let result = vec.iter().map(
        |entity| entity.to_math_ml()
    ).collect::<Vec<Vec<u16>>>().concat();

    if vec.len() > 1 && single_element {
        vec![
            into_v16("<mrow>"),
            result,
            into_v16("</mrow>")
        ].concat()
    }

    else if vec.len() == 0 {
        into_v16("<mo>&nbsp;</mo>")
    }

    else {
        result
    }

}