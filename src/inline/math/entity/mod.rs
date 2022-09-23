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
    Space(usize),
    Identifier(Vec<u16>),    // <mi>
    Operator(Vec<u16>),      // <mo>
    Number(Vec<u16>),        // <mn>
    RawString(Vec<u16>),     // string inside `<mtext>`
    Character(u16),          // &#xxx;
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

    pub fn new_identifier(identifier: Vec<u16>) -> Self {
        Entity::Identifier(identifier)
    }

    pub fn new_number(number: Vec<u16>) -> Self {
        Entity::Number(number)
    }

    pub fn new_operator(operator: Vec<u16>) -> Self {
        Entity::Operator(operator)
    }

    pub fn to_math_ml(&self) -> Vec<u16> {

        match self {
            Entity::Space(space) => into_v16(&format!("<mspace width=\"{}\"/>", calc_space(*space))),
            Entity::Root(root) => root.to_math_ml(),
            Entity::Fraction(fraction) => fraction.to_math_ml(),
            Entity::UnderOver(underover) => underover.to_math_ml(),
            Entity::Script(script) => script.to_math_ml(),
            Entity::Character(character) => {
                let tag = if 913 <= *character && *character <= 969 {  // Alpha to omega
                    "mi"
                } else if *character == 8734 {  // infty
                    "mn"
                } else {
                    "mo"
                };
                into_v16(&format!("<{}>&#{};</{}>", tag, character, tag))
            }
            Entity::Identifier(identifier) => vec![
                into_v16("<mi>"),
                identifier.clone(),
                into_v16("</mi>"),
            ].concat(),
            Entity::Number(number) => vec![
                into_v16("<mn>"),
                number.clone(),
                into_v16("</mn>"),
            ].concat(),
            // `++` -> `<mo>+</mo><mo>+</mo>`
            Entity::Operator(operator) => {
                operator.iter().map(
                    |op|
                    vec![
                        into_v16("<mo>"),
                        vec![*op],
                        into_v16("</mo>"),
                    ].concat()
                ).collect::<Vec<Vec<u16>>>().concat()
            },
            Entity::RawString(string) => vec![
                into_v16("<mtext>"),
                string.clone(),
                into_v16("</mtext>"),
            ].concat(),
        }

    }

}

fn calc_space(space: usize) -> String {
    format!(
        "{}{}em",
        space / 3,
        if space % 3 == 0 {
            ""
        } else if space % 3 == 1 {
            ".333"
        } else {
            ".667"
        }
    )
}

#[derive(PartialEq)]
enum StringState {
    Identifier,  // <mi>
    Number,      // <mn>
    Operator,     // <mo>
}

fn get_string_state(character: &u16) -> StringState {

    if is_alphabet(character) {
        StringState::Identifier
    }

    else if is_numeric(character) {
        StringState::Number
    }

    else {
        StringState::Operator
    }

}

pub fn parse_raw_data(string: &[u16]) -> Vec<Entity> {

    if string.len() == 0 {
        vec![]
    }

    else {
        let mut curr_state = get_string_state(&string[0]);

        let mut last_index = 0;
        let mut result = vec![];

        for (curr_index, c) in string.iter().enumerate() {

            match curr_state {
                StringState::Identifier if !is_alphabet(c) => {
                    result.push(Entity::new_identifier(string[last_index..curr_index].to_vec()));
                    last_index = curr_index;
                    curr_state = get_string_state(c);
                },
                StringState::Number if !is_numeric(c) && *c != '.' as u16 => {
                    result.push(Entity::new_number(string[last_index..curr_index].to_vec()));
                    last_index = curr_index;
                    curr_state = get_string_state(c);
                },
                StringState::Operator if is_alphabet(c) || is_numeric(c) => {
                    result.push(Entity::new_operator(string[last_index..curr_index].to_vec()));
                    last_index = curr_index;
                    curr_state = get_string_state(c);
                },
                _ => {}
            }

        }

        if last_index < string.len() {

            match curr_state {
                StringState::Identifier => {
                    result.push(Entity::new_identifier(string[last_index..].to_vec()));
                }
                StringState::Number => {
                    result.push(Entity::new_number(string[last_index..].to_vec()));
                }
                StringState::Operator => {
                    result.push(Entity::new_operator(string[last_index..].to_vec()));
                }
            }

        }

        result
    }

}

pub fn vec_to_math_ml(vec: &Vec<Entity>, single_element: bool) -> Vec<u16> {
    let result = vec.iter().map(
        |entity| entity.to_math_ml()
    ).collect::<Vec<Vec<u16>>>().concat();

    if count_entity(vec) > 1 && single_element {
        vec![
            into_v16("<mrow>"),
            result,
            into_v16("</mrow>")
        ].concat()
    }

    else if count_entity(vec) == 0 {
        into_v16("<mo>&nbsp;</mo>")
    }

    else {
        result
    }

}

// `vec.len()` and `count_entity(vec)` are different sometimes
fn count_entity(vec: &Vec<Entity>) -> usize {
    let mut result = 0;

    for entity in vec.iter() {
        match entity {
            // `++` -> `<mo>+</mo><mo>+</mo>`
            Entity::Operator(operator) => {
                result += operator.len();
            }
            _ => {
                result += 1;
            }
        }
    }

    result
}