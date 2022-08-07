use super::entity::{Entity, parse_raw_data};
use super::validate::is_valid;
use super::{ZERO_ARG_FUNCTIONS, ONE_ARG_FUNCTIONS, TWO_ARG_FUNCTIONS, THREE_ARG_FUNCTIONS, FIVE_ARG_FUNCTIONS};
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
            let (arguments, _) = get_arguments(content, curr_index);

            if is_valid(curr_word, &arguments) {
                result.push(parse(curr_word, &arguments));
            }

            else {
                let string = remove_whitespaces(&content[last_index..curr_index]);

                if string.len() > 0 {

                    for entity in parse_raw_data(&string) {
                        result.push(entity);
                    }

                }

            }

        }

        else {
            let string = remove_whitespaces(&content[last_index..curr_index]);

            if string.len() > 0 {

                for entity in parse_raw_data(&string) {
                    result.push(entity);
                }

            }

        }

    }

    result
}

pub fn parse(word: &[u16], arguments: &Vec<Vec<u16>>) -> Entity {

    if is_space(word) {
        Entity::Space(word.len() - 4)
    }

    else if ZERO_ARG_FUNCTIONS.contains(word) && arguments.len() == 0 {

        if *word == into_v16("alpha") {
            Entity::new_character(945)
        }

        else if *word == into_v16("beta") {
            Entity::new_character(946)
        }

        else if *word == into_v16("gamma") {
            Entity::new_character(947)
        }

        else if *word == into_v16("delta") {
            Entity::new_character(948)
        }

        else if *word == into_v16("epsilon") {
            Entity::new_character(949)
        }

        else if *word == into_v16("zeta") {
            Entity::new_character(950)
        }

        else if *word == into_v16("eta") {
            Entity::new_character(951)
        }

        else if *word == into_v16("theta") {
            Entity::new_character(952)
        }

        else if *word == into_v16("iota") {
            Entity::new_character(953)
        }

        else if *word == into_v16("kappa") {
            Entity::new_character(954)
        }

        else if *word == into_v16("lambda") {
            Entity::new_character(955)
        }

        else if *word == into_v16("mu") {
            Entity::new_character(956)
        }

        else if *word == into_v16("nu") {
            Entity::new_character(957)
        }

        else if *word == into_v16("xi") {
            Entity::new_character(958)
        }

        else if *word == into_v16("omicron") {
            Entity::new_character(959)
        }

        else if *word == into_v16("pi") {
            Entity::new_character(960)
        }

        else if *word == into_v16("rho") {
            Entity::new_character(961)
        }

        else if *word == into_v16("sigma") {
            Entity::new_character(963)
        }

        else if *word == into_v16("tau") {
            Entity::new_character(964)
        }

        else if *word == into_v16("upsilon") {
            Entity::new_character(965)
        }

        else if *word == into_v16("phi") {
            Entity::new_character(966)
        }

        else if *word == into_v16("chi") {
            Entity::new_character(967)
        }

        else if *word == into_v16("psi") {
            Entity::new_character(968)
        }

        else if *word == into_v16("omega") {
            Entity::new_character(969)
        }

        else if *word == into_v16("Alpha") {
            Entity::new_character(913)
        }

        else if *word == into_v16("Beta") {
            Entity::new_character(914)
        }

        else if *word == into_v16("Gamma") {
            Entity::new_character(915)
        }

        else if *word == into_v16("Delta") {
            Entity::new_character(916)
        }

        else if *word == into_v16("Epsilon") {
            Entity::new_character(917)
        }

        else if *word == into_v16("Zeta") {
            Entity::new_character(918)
        }

        else if *word == into_v16("Eta") {
            Entity::new_character(919)
        }

        else if *word == into_v16("Theta") {
            Entity::new_character(920)
        }

        else if *word == into_v16("Iota") {
            Entity::new_character(921)
        }

        else if *word == into_v16("Kappa") {
            Entity::new_character(922)
        }

        else if *word == into_v16("Lambda") {
            Entity::new_character(923)
        }

        else if *word == into_v16("Mu") {
            Entity::new_character(924)
        }

        else if *word == into_v16("Nu") {
            Entity::new_character(925)
        }

        else if *word == into_v16("Xi") {
            Entity::new_character(926)
        }

        else if *word == into_v16("Omicron") {
            Entity::new_character(927)
        }

        else if *word == into_v16("Pi") {
            Entity::new_character(928)
        }

        else if *word == into_v16("Rho") {
            Entity::new_character(929)
        }

        else if *word == into_v16("Sigma") {
            Entity::new_character(931)
        }

        else if *word == into_v16("Tau") {
            Entity::new_character(932)
        }

        else if *word == into_v16("Upsilon") {
            Entity::new_character(933)
        }

        else if *word == into_v16("Phi") {
            Entity::new_character(934)
        }

        else if *word == into_v16("Chi") {
            Entity::new_character(935)
        }

        else if *word == into_v16("Psi") {
            Entity::new_character(936)
        }

        else if *word == into_v16("Omega") {
            Entity::new_character(937)
        }

        else if *word == into_v16("lcb") {  // left curly bracket
            Entity::new_character(123)
        }

        else if *word == into_v16("rcb") {  // right curly bracket
            Entity::new_character(125)
        }

        else if *word == into_v16("pm") {
            Entity::new_character(177)
        }

        else if *word == into_v16("times") {
            Entity::new_character(215)
        }

        else if *word == into_v16("leftarrow") {
            Entity::new_character(8592)
        }

        else if *word == into_v16("uparrow") {
            Entity::new_character(8593)
        }

        else if *word == into_v16("rightarrow") {
            Entity::new_character(8594)
        }

        else if *word == into_v16("downarrow") {
            Entity::new_character(8595)
        }

        else if *word == into_v16("forall") {
            Entity::new_character(8704)
        }

        else if *word == into_v16("partial") {
            Entity::new_character(8706)
        }

        else if *word == into_v16("exist") {
            Entity::new_character(8707)
        }

        else if *word == into_v16("empty") || *word == into_v16("null") {
            Entity::new_character(8709)
        }

        else if *word == into_v16("triangle") {
            Entity::new_character(8710)
        }

        else if *word == into_v16("nabla") {
            Entity::new_character(8711)
        }

        else if *word == into_v16("in") {
            Entity::new_character(8712)
        }

        else if *word == into_v16("notin") {
            Entity::new_character(8713)
        }

        else if *word == into_v16("ni") {
            Entity::new_character(8715)
        }

        else if *word == into_v16("notni") {
            Entity::new_character(8716)
        }

        else if *word == into_v16("qed") {
            Entity::new_character(8718)
        }

        else if *word == into_v16("mp") {
            Entity::new_character(8723)
        }

        else if *word == into_v16("circ") {
            Entity::new_character(8728)
        }

        else if *word == into_v16("bullet") {
            Entity::new_character(8729)
        }

        else if *word == into_v16("prop") {
            Entity::new_character(8733)
        }

        else if *word == into_v16("inf") || *word == into_v16("infty") || *word == into_v16("infin") {
            Entity::new_character(8734)
        }

        else if *word == into_v16("and") {
            Entity::new_character(8743)
        }

        else if *word == into_v16("or") {
            Entity::new_character(8744)
        }

        else if *word == into_v16("cap") {
            Entity::new_character(8745)
        }

        else if *word == into_v16("cup") {
            Entity::new_character(8746)
        }

        else if *word == into_v16("therefore") {
            Entity::new_character(8756)
        }

        else if *word == into_v16("because") {
            Entity::new_character(8757)
        }

        else if *word == into_v16("simeq") {
            Entity::new_character(8771)
        }

        else if *word == into_v16("asymp") {
            Entity::new_character(8776)
        }

        else if *word == into_v16("ne") || *word == into_v16("neq") {
            Entity::new_character(8800)
        }

        else if *word == into_v16("equiv") {
            Entity::new_character(8801)
        }

        else if *word == into_v16("nequiv") {
            Entity::new_character(8802)
        }

        else if *word == into_v16("lt") {
            Entity::new_character(60)
        }

        else if *word == into_v16("gt") {
            Entity::new_character(62)
        }

        else if *word == into_v16("le") || *word == into_v16("leq") {
            Entity::new_character(8804)
        }

        else if *word == into_v16("ge") || *word == into_v16("geq") {
            Entity::new_character(8805)
        }

        else if *word == into_v16("llt") {
            Entity::new_character(8810)
        }

        else if *word == into_v16("ggt") {
            Entity::new_character(8811)
        }

        else if *word == into_v16("sub") {
            Entity::new_character(8834)
        }

        else if *word == into_v16("sup") {
            Entity::new_character(8835)
        }

        else if *word == into_v16("nsub") {
            Entity::new_character(8836)
        }

        else if *word == into_v16("nsup") {
            Entity::new_character(8837)
        }

        else if *word == into_v16("sube") {
            Entity::new_character(8838)
        }

        else if *word == into_v16("supe") {
            Entity::new_character(8839)
        }

        else if *word == into_v16("nsube") {
            Entity::new_character(8840)
        }

        else if *word == into_v16("nsupe") {
            Entity::new_character(8841)
        }

        else if *word == into_v16("oplus") {
            Entity::new_character(8853)
        }

        else if *word == into_v16("ominus") {
            Entity::new_character(8854)
        }

        else if *word == into_v16("otimes") {
            Entity::new_character(8855)
        }

        else if *word == into_v16("odiv") {
            Entity::new_character(8856)
        }

        else if *word == into_v16("odot") {
            Entity::new_character(8857)
        }

        else if *word == into_v16("dot") {
            Entity::new_character(8901)
        }

        else if *word == into_v16("star") {
            Entity::new_character(8902)
        }

        else {
            todo!()
        }

    }

    else if ONE_ARG_FUNCTIONS.contains(word) && arguments.len() == 1 {

        if *word == into_v16("sqrt") {
            Entity::new_root(vec![], md_to_math(&arguments[0]))
        }

        else if *word == into_v16("text") {
            Entity::RawString(arguments[0].clone())
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
            let operator = if *word == into_v16("hat") {
                '^' as u16
            } else if *word == into_v16("bar") {
                '-' as u16
            } else if *word == into_v16("dot") {
                8901
            } else if *word == into_v16("tilde") {
                '~' as u16
            } else if *word == into_v16("vec") {
                8594
            } else {
                unreachable!()
            };

            Entity::new_underover(
                md_to_math(&arguments[0]),
                vec![],
                vec![Entity::new_character(operator)],
                false
            )
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
            let operator = if *word == into_v16("sum") {
                "∑"
            } else if *word == into_v16("prod") {
                "∏"
            } else if *word == into_v16("int") {
                "∫"
            } else if *word == into_v16("iint") {
                "∬"
            } else if *word == into_v16("iiint") {
                "∭"
            } else if *word == into_v16("oint") {
                "∮"
            } else {
                unreachable!()
            };

            Entity::new_underover(
                vec![Entity::new_operator(into_v16(operator))],
                md_to_math(&arguments[0]),
                md_to_math(&arguments[1]),
                true
            )
        }

    }

    else if THREE_ARG_FUNCTIONS.contains(word) && arguments.len() == 3 {

        if *word == into_v16("subsup") {
            Entity::new_script(
                md_to_math(&arguments[0]),
                vec![],
                md_to_math(&arguments[2]),
                vec![],
                md_to_math(&arguments[1]),
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

pub fn is_space(word: &[u16]) -> bool {
    word.len() > 4
    && &word[(word.len() - 5)..(word.len())] == &into_v16("space")
    && word[0..(word.len() - 5)].iter().all(|c| *c == 's' as u16)
}

#[cfg(test)]
mod testbench {
    use super::{get_arguments, is_space};
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

    #[test]
    fn space_test() {
        assert!(is_space(&into_v16("space")));
        assert!(is_space(&into_v16("sspace")));
        assert!(is_space(&into_v16("ssspace")));
        assert!(is_space(&into_v16("sssssssssssssssssspace")));

        assert!(!is_space(&into_v16("pace")));
        assert!(!is_space(&into_v16("espace")));
        assert!(!is_space(&into_v16("sespace")));
        assert!(!is_space(&into_v16("")));
    }

}