use crate::footnote::predicate::is_valid_footnote_label;
use crate::link::predicate::is_valid_link_label;
use crate::ast::line::Line;
use crate::utils::*;

impl Line {

    #[inline]
    pub fn is_header(&self) -> bool {
        self.indent == 0
        && {
            let (pre, post) = take_and_drop_while(&self.content, '#' as u16);

            pre.len() > 0 && pre.len() < 7 && post[0] == ' ' as u16 && drop_while(&post, ' ' as u16).len() > 0
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.len() == 0
    }

    #[inline]
    pub fn is_code_fence_begin(&self) -> bool {
        // TODO check the following string
        self.indent == 0 && self.content.len() > 2 && &self.content[0..3] == &into_v16("```")
    }

    #[inline]
    pub fn is_code_fence_end(&self) -> bool {
        // TODO
        self.is_code_fence_begin()
    }

    #[inline]
    pub fn is_table_row(&self) -> bool {
        self.indent < 4 && self.content.len() > 1 && self.content[0] == '|' as u16 && self.content[self.content.len() - 1] == '|' as u16
    }

    #[inline]
    pub fn is_table_delimiter(&self) -> bool {

        self.is_table_row() && self.content.iter().all(
            |c|
            *c == '|' as u16 || *c == '-' as u16 || *c == ':' as u16 || *c == ' ' as u16
        ) && {
            // the first and the last element of `cells` is empty, because the line has trailing and leading pipes.
            // the empty elements should be eliminated
            let cells = self.content.split(|c| *c == '|' as u16).collect::<Vec<&[u16]>>();

            cells[1..cells.len() - 1].iter().all(
                |delim| {
                    let stripped = strip_whitespaces(delim);

                    stripped.len() > 0 && (
                        (stripped.len() == 1 && stripped[0] == '-' as u16) || (
                            stripped[1..stripped.len() - 1].iter().all(|c| *c == '-' as u16)
                            && (stripped[0] == '-' as u16 || stripped[0] == ':' as u16)
                            && (stripped[stripped.len() - 1] == '-' as u16 || stripped[stripped.len() - 1] == ':' as u16)
                        )
                    )
                }
            )
        }
    }

    #[inline]
    pub fn is_thematic_break(&self) -> bool {
        self.indent < 4 && self.content.len() > 2 && (
            self.content[0] == '*' as u16 ||
            self.content[0] == '-' as u16 ||
            self.content[0] == '_' as u16
        ) && {
            let (pre, post) = take_and_drop_while(&self.content, self.content[0]);
    
            pre.len() > 2 && post.iter().filter(|c| **c != ' ' as u16 && **c != '\t' as u16).collect::<Vec<&u16>>().len() == 0
        }
    }

    #[inline]
    pub fn is_unordered_list(&self) -> bool {
        self.content.len() > 2 && (self.content[0] == '-' as u16 || self.content[0] == '*' as u16) && self.content[1] == ' ' as u16
    }

    #[inline]
    pub fn is_link_or_footnote_reference_definition(&self) -> bool {
        self.indent < 4 && self.content.len() > 4 && self.content[0] == '[' as u16
        && match get_bracket_end_index(&self.content, 0) {
            None => false,
            Some(bracket_end_index) => self.content.len() > bracket_end_index + 2
            && self.content[bracket_end_index + 1] == ':' as u16
            && (is_valid_link_label(&self.content[1..bracket_end_index]) || is_valid_footnote_label(&self.content[1..bracket_end_index]))
        }

    }
}