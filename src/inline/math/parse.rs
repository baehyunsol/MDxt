use super::entity::{Entity, parse_raw_data};
use super::validate::is_valid;
use super::{ZERO_ARG_FUNCTIONS, ONE_ARG_FUNCTIONS, TWO_ARG_FUNCTIONS, FIVE_ARG_FUNCTIONS};
use crate::utils::{get_curly_brace_end_index, into_v16, is_alphabet, remove_whitespaces};

pub fn md_to_math(content: &[u16]) -> Vec<Entity> {

    let mut last_index = 0;
    let mut curr_index = 0;
    let mut is_reading_alphabets = false;
    let mut result = vec![];

    while curr_index < content.len() {

        if is_alphabet(&content[curr_index]) && !is_reading_alphabets {

            if last_index < curr_index {
                let string = remove_whitespaces(&content[last_index..curr_index]);

                if string.len() > 0 {

                    for entity in parse_raw_data(&string) {
                        result.push(entity);
                    }
                    //result.push(Entity::RawData(string));
                }

            }

            last_index = curr_index;
            is_reading_alphabets = true;
        }

        else if !is_alphabet(&content[curr_index]) && is_reading_alphabets {
            let curr_word = &content[last_index..curr_index];
            let (arguments, end_index) = get_arguments(content, curr_index);

            if is_valid(curr_word, &arguments) {
                result.push(parse(curr_word, &arguments));
                curr_index = end_index;
                last_index = end_index + 1;
            }

            is_reading_alphabets = false;
        }

        curr_index += 1;
    }

    if last_index < curr_index {
        curr_index = curr_index.min(content.len());

        if is_reading_alphabets {
            let curr_word = &content[last_index..curr_index];
            let (arguments, end_index) = get_arguments(content, curr_index);

            if is_valid(curr_word, &arguments) {
                result.push(parse(curr_word, &arguments));
            }

            else {
                let string = remove_whitespaces(&content[last_index..curr_index]);

                if string.len() > 0 {

                    for entity in parse_raw_data(&string) {
                        result.push(entity);
                    }
                    //result.push(Entity::RawData(string));
                }

            }

        }

        else {
            let string = remove_whitespaces(&content[last_index..curr_index]);

            if string.len() > 0 {

                for entity in parse_raw_data(&string) {
                    result.push(entity);
                }
                //result.push(Entity::RawData(string));
            }

        }

    }

    result
}

pub fn parse(word: &[u16], arguments: &Vec<Vec<u16>>) -> Entity {

    if ZERO_ARG_FUNCTIONS.contains(word) && arguments.len() == 0 {

        if *word == into_v16("space") {
            Entity::Character(160)
        }

        else if *word == into_v16("alpha") {
            Entity::Character(945)
        }

        else if *word == into_v16("beta") {
            Entity::Character(946)
        }

        else if *word == into_v16("gamma") {
            Entity::Character(947)
        }

        else if *word == into_v16("delta") {
            Entity::Character(948)
        }

        else if *word == into_v16("epsilon") {
            Entity::Character(949)
        }

        else if *word == into_v16("zeta") {
            Entity::Character(950)
        }

        else if *word == into_v16("eta") {
            Entity::Character(951)
        }

        else if *word == into_v16("theta") {
            Entity::Character(952)
        }

        else if *word == into_v16("iota") {
            Entity::Character(953)
        }

        else if *word == into_v16("kappa") {
            Entity::Character(954)
        }

        else if *word == into_v16("lambda") {
            Entity::Character(955)
        }

        else if *word == into_v16("mu") {
            Entity::Character(956)
        }

        else if *word == into_v16("nu") {
            Entity::Character(957)
        }

        else if *word == into_v16("xi") {
            Entity::Character(958)
        }

        else if *word == into_v16("omicron") {
            Entity::Character(959)
        }

        else if *word == into_v16("pi") {
            Entity::Character(960)
        }

        else if *word == into_v16("rho") {
            Entity::Character(961)
        }

        else if *word == into_v16("sigma") {
            Entity::Character(963)
        }

        else if *word == into_v16("tau") {
            Entity::Character(964)
        }

        else if *word == into_v16("upsilon") {
            Entity::Character(965)
        }

        else if *word == into_v16("phi") {
            Entity::Character(966)
        }

        else if *word == into_v16("chi") {
            Entity::Character(967)
        }

        else if *word == into_v16("psi") {
            Entity::Character(968)
        }

        else if *word == into_v16("omega") {
            Entity::Character(969)
        }

        else if *word == into_v16("Alpha") {
            Entity::Character(913)
        }

        else if *word == into_v16("Beta") {
            Entity::Character(914)
        }

        else if *word == into_v16("Gamma") {
            Entity::Character(915)
        }

        else if *word == into_v16("Delta") {
            Entity::Character(916)
        }

        else if *word == into_v16("Epsilon") {
            Entity::Character(917)
        }

        else if *word == into_v16("Zeta") {
            Entity::Character(918)
        }

        else if *word == into_v16("Eta") {
            Entity::Character(919)
        }

        else if *word == into_v16("Theta") {
            Entity::Character(920)
        }

        else if *word == into_v16("Iota") {
            Entity::Character(921)
        }

        else if *word == into_v16("Kappa") {
            Entity::Character(922)
        }

        else if *word == into_v16("Lambda") {
            Entity::Character(923)
        }

        else if *word == into_v16("Mu") {
            Entity::Character(924)
        }

        else if *word == into_v16("Nu") {
            Entity::Character(925)
        }

        else if *word == into_v16("Xi") {
            Entity::Character(926)
        }

        else if *word == into_v16("Omicron") {
            Entity::Character(927)
        }

        else if *word == into_v16("Pi") {
            Entity::Character(928)
        }

        else if *word == into_v16("Rho") {
            Entity::Character(929)
        }

        else if *word == into_v16("Sigma") {
            Entity::Character(931)
        }

        else if *word == into_v16("Tau") {
            Entity::Character(932)
        }

        else if *word == into_v16("Upsilon") {
            Entity::Character(933)
        }

        else if *word == into_v16("Phi") {
            Entity::Character(934)
        }

        else if *word == into_v16("Chi") {
            Entity::Character(935)
        }

        else if *word == into_v16("Psi") {
            Entity::Character(936)
        }

        else if *word == into_v16("Omega") {
            Entity::Character(937)
        }

        else if *word == into_v16("pm") {
            Entity::Character(177)
        }

        else if *word == into_v16("times") {
            Entity::Character(215)
        }

        else if *word == into_v16("leftarrow") {
            Entity::Character(8592)
        }

        else if *word == into_v16("uparrow") {
            Entity::Character(8593)
        }

        else if *word == into_v16("rightarrow") {
            Entity::Character(8594)
        }

        else if *word == into_v16("downarrow") {
            Entity::Character(8595)
        }

        else if *word == into_v16("forall") {
            Entity::Character(8704)
        }

        else if *word == into_v16("partial") {
            Entity::Character(8706)
        }

        else if *word == into_v16("exist") {
            Entity::Character(8707)
        }

        else if *word == into_v16("empty") || *word == into_v16("null") {
            Entity::Character(8709)
        }

        else if *word == into_v16("triangle") {
            Entity::Character(8710)
        }

        else if *word == into_v16("nabla") {
            Entity::Character(8711)
        }

        else if *word == into_v16("in") {
            Entity::Character(8712)
        }

        else if *word == into_v16("notin") {
            Entity::Character(8713)
        }

        else if *word == into_v16("ni") {
            Entity::Character(8715)
        }

        else if *word == into_v16("notni") {
            Entity::Character(8716)
        }

        else if *word == into_v16("qed") {
            Entity::Character(8718)
        }

        else if *word == into_v16("mp") {
            Entity::Character(8723)
        }

        else if *word == into_v16("circ") {
            Entity::Character(8728)
        }

        else if *word == into_v16("bullet") {
            Entity::Character(8729)
        }

        else if *word == into_v16("prop") {
            Entity::Character(8733)
        }

        else if *word == into_v16("inf") || *word == into_v16("infty") || *word == into_v16("infin") {
            Entity::Character(8734)
        }

        else if *word == into_v16("and") {
            Entity::Character(8743)
        }

        else if *word == into_v16("or") {
            Entity::Character(8744)
        }

        else if *word == into_v16("cap") {
            Entity::Character(8745)
        }

        else if *word == into_v16("cup") {
            Entity::Character(8746)
        }

        else if *word == into_v16("therefore") {
            Entity::Character(8756)
        }

        else if *word == into_v16("because") {
            Entity::Character(8757)
        }

        else if *word == into_v16("simeq") {
            Entity::Character(8771)
        }

        else if *word == into_v16("asymp") {
            Entity::Character(8776)
        }

        else if *word == into_v16("ne") || *word == into_v16("neq") {
            Entity::Character(8800)
        }

        else if *word == into_v16("equiv") {
            Entity::Character(8801)
        }

        else if *word == into_v16("nequiv") {
            Entity::Character(8802)
        }

        else if *word == into_v16("lt") {
            Entity::Character(60)
        }

        else if *word == into_v16("gt") {
            Entity::Character(62)
        }

        else if *word == into_v16("le") || *word == into_v16("leq") {
            Entity::Character(8804)
        }

        else if *word == into_v16("ge") || *word == into_v16("geq") {
            Entity::Character(8805)
        }

        else if *word == into_v16("llt") {
            Entity::Character(8810)
        }

        else if *word == into_v16("ggt") {
            Entity::Character(8811)
        }

        else if *word == into_v16("sub") {
            Entity::Character(8834)
        }

        else if *word == into_v16("sup") {
            Entity::Character(8835)
        }

        else if *word == into_v16("nsub") {
            Entity::Character(8836)
        }

        else if *word == into_v16("nsup") {
            Entity::Character(8837)
        }

        else if *word == into_v16("sube") {
            Entity::Character(8838)
        }

        else if *word == into_v16("supe") {
            Entity::Character(8839)
        }

        else if *word == into_v16("nsube") {
            Entity::Character(8840)
        }

        else if *word == into_v16("nsupe") {
            Entity::Character(8841)
        }

        else if *word == into_v16("oplus") {
            Entity::Character(8853)
        }

        else if *word == into_v16("ominus") {
            Entity::Character(8854)
        }

        else if *word == into_v16("otimes") {
            Entity::Character(8855)
        }

        else if *word == into_v16("odiv") {
            Entity::Character(8856)
        }

        else if *word == into_v16("odot") {
            Entity::Character(8857)
        }

        else if *word == into_v16("star") {
            Entity::Character(8902)
        }

        else {
            todo!()
        }

    }

    else if ONE_ARG_FUNCTIONS.contains(word) && arguments.len() == 1 {

        if *word == into_v16("sqrt") {
            Entity::new_root(vec![], md_to_math(&arguments[0]))
        }

        else if *word == into_v16("lim") || *word == into_v16("limit") {
            Entity::new_underover(
                vec![Entity::new_identifier(into_v16("lim"))],
                md_to_math(&arguments[0]),
                vec![],
                false
            )
        }

        else {
            todo!()
        }

    }

    else if TWO_ARG_FUNCTIONS.contains(word) && arguments.len() == 2 {

        if *word == into_v16("sqrt") || *word == into_v16("root") {
            Entity::new_root(md_to_math(&arguments[0]), md_to_math(&arguments[1]))
        }

        else if *word == into_v16("frac") {
            Entity::new_fraction(
                md_to_math(&arguments[0]),
                md_to_math(&arguments[1]),
                false,
                false
            )
        }

        else if *word == into_v16("cfrac") {
            Entity::new_fraction(
                md_to_math(&arguments[0]),
                md_to_math(&arguments[1]),
                true,
                false
            )
        }

        else if *word == into_v16("bincoeff") {
            Entity::new_fraction(
                md_to_math(&arguments[0]),
                md_to_math(&arguments[1]),
                false,
                true
            )
        }

        else if *word == into_v16("sum") {
            Entity::new_underover(
                vec![Entity::new_operator(into_v16("∑"))],
                md_to_math(&arguments[0]),
                md_to_math(&arguments[1]),
                true
            )
        }

        else if *word == into_v16("prod") {
            Entity::new_underover(
                vec![Entity::new_operator(into_v16("∏"))],
                md_to_math(&arguments[0]),
                md_to_math(&arguments[1]),
                true
            )
        }

        else if *word == into_v16("sub") {
            Entity::new_script(
                md_to_math(&arguments[0]),
                vec![],
                vec![],
                vec![],
                md_to_math(&arguments[1])
            )
        }

        else if *word == into_v16("sup") {
            Entity::new_script(
                md_to_math(&arguments[0]),
                vec![],
                md_to_math(&arguments[1]),
                vec![],
                vec![]
            )
        }

        else {
            todo!()
        }

    }

    else if FIVE_ARG_FUNCTIONS.contains(word) && arguments.len() == 5 {

        if *word == into_v16("multiscript") {
            Entity::new_script(
                md_to_math(&arguments[0]),
                md_to_math(&arguments[1]),
                md_to_math(&arguments[2]),
                md_to_math(&arguments[3]),
                md_to_math(&arguments[4]),
            )
        }

        else {
            todo!()
        }

    }

    else {
        unreachable!()
    }

}

pub fn get_arguments(content: &[u16], mut index: usize) -> (Vec<Vec<u16>>, usize) {  // (Vec<argument>, end_index)

    let mut result = vec![];

    if index >= content.len() {
        return (result, index);
    }

    loop {

        while index < content.len() && content[index] == ' ' as u16 {
            index += 1;
        }

        if index < content.len() && content[index] == '{' as u16  {
            let arg_end_index = match get_curly_brace_end_index(content, index) {
                Some(end_index) => end_index,
                None => {
                    return (result, index - 1);
                }
            };

            result.push(content[(index + 1)..arg_end_index].to_vec());
            index = arg_end_index + 1;
        }

        else {
            return (result, index - 1);
        }

    }

}

#[cfg(test)]
mod testbench {
    use super::get_arguments;
    use crate::utils::into_v16;

    #[test]
    fn get_arguments_test() {
        let test_cases = vec![
            // (test_case, start_index, arguments, end_index)
            ("a{b}{c}", 1, vec!["b", "c"], 6),
            ("a {b}{c}", 1, vec!["b", "c"], 7),
            ("a {b}{c}", 2, vec!["b", "c"], 7),
            ("a {b} {c}", 1, vec!["b", "c"], 8),
            ("a {b} {c}", 2, vec!["b", "c"], 8),
            ("a {b} {c", 1, vec!["b"], 5),
            ("a{b{d}}{c}", 1, vec!["b{d}", "c"], 9),
            ("", 0, vec![], 0),
        ];

        let test_cases = test_cases.into_iter().map(
            |(test_case, start_index, arguments, end_index)|
            (
                into_v16(&test_case),
                start_index,
                arguments.iter().map(
                    |argument|
                    into_v16(argument)
                ).collect(),
                end_index
            )
        ).collect::<Vec<(Vec<u16>, usize, Vec<Vec<u16>>, usize)>>();

        for (test_case, start_index, arguments, end_index) in test_cases.into_iter() {
            assert_eq!(get_arguments(&test_case, start_index), (arguments, end_index));
        }

    }

}