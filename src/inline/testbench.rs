mod tests {

    #[test]
    fn inline_node_test() {
        use crate::inline::InlineNode;
        use crate::utils::into_v16;

        let test_cases = vec![
            "*italic* **bold** ~_underline_~ ~subscript~ ^superscript^ `codespan` ~~deletion~~",
            "`*italic in a codespan, which is not rendered*` ***italic_and_bold*** ~_~~del_and_underline~~_~",
            "`*`*`*`, *`*`*`*",
            "****abcde****, ~~deletion?~~~, ~~~deletion?~~",
            "[[red]]This text is red and **bold**.[[/red]] [[center]] Some whitespaces  [[/center]]",
            "[[red]][[center]] Broken Macros! [[/cetner]]"
        ];

        let test_cases: Vec<Vec<u16>> = test_cases.into_iter().map(into_v16).collect();

        for test_case in test_cases.iter() {

            // it's not always equal
            // I cherrypicked cases that make these two identical
            assert_eq!(&InlineNode::to_md(&InlineNode::from_md(test_case)), test_case);
        }

    }

}