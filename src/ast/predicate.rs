use crate::ast::line::Line;
use crate::container::codefence::predicate::is_valid_info_string;
use crate::escape::HTML_ESCAPE_OFFSET;
use crate::inline::{
    macros::predicate::read_macro,
    link::predicate::is_valid_link_label,
    footnote::predicate::is_valid_footnote_label
};
use crate::utils::*;

impl Line {

    #[inline]
    pub fn is_header(&self) -> bool {
        self.indent == 0
        && {
            let (pre, post) = take_and_drop_while(&self.content, '#' as u32);

            pre.len() > 0 && pre.len() < 7 && (post.len() == 0 || post[0] == ' ' as u32)
        }
    }

    // indented empty lines must be considered an empty line -> regarding indented code blocks
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.len() == 0
    }

    #[inline]
    pub fn is_code_fence_begin(&self) -> bool {
        self.is_code_fence() && is_valid_info_string(&drop_while(&self.content[3..self.content.len()], self.content[0]))
    }

    #[inline]
    pub fn is_code_fence_end(&self) -> bool {
        self.is_code_fence() && {
            let mut index = 3;

            while index < self.content.len() {

                if self.content[index] != self.content[0] {
                    return false;
                }

                index += 1;
            }

            true
        }
    }

    #[inline]
    fn is_code_fence(&self) -> bool {
        self.indent < 4 && self.content.len() > 2
        && (self.content[0] == '`' as u32 || self.content[0] == '~' as u32)
        && self.content[0] == self.content[1] && self.content[1] == self.content[2]
    }

    #[inline]
    pub fn is_table_row(&self) -> bool {
        self.indent < 4 && self.content.len() > 1 && self.content[0] == '|' as u32 && {
            let mut last_index = self.content.len() - 1;

            while self.content[last_index] == ' ' as u32 {
                last_index -= 1;
            }

            self.content[last_index] == '|' as u32
        }
    }

    #[inline]
    pub fn is_table_delimiter(&self) -> bool {

        self.is_table_row() && self.content.iter().all(
            |c|
            *c == '|' as u32 || *c == '-' as u32 || *c == ':' as u32 || *c == ' ' as u32
        ) && {
            // the first and the last element of `cells` is empty, because the line has trailing and leading pipes.
            // the empty elements should be eliminated
            let cells = self.content.split(|c| *c == '|' as u32).collect::<Vec<&[u32]>>();

            cells[1..cells.len() - 1].iter().all(
                |delim| {
                    let stripped = strip_whitespaces(delim);

                    stripped.len() > 0 && (
                        (stripped.len() == 1 && stripped[0] == '-' as u32) || (
                            stripped[1..stripped.len() - 1].iter().all(|c| *c == '-' as u32)
                            && (stripped[0] == '-' as u32 || stripped[0] == ':' as u32)
                            && (stripped[stripped.len() - 1] == '-' as u32 || stripped[stripped.len() - 1] == ':' as u32)
                        )
                    )
                }
            )
        }
    }

    #[inline]
    pub fn is_thematic_break(&self) -> bool {
        self.indent < 4 && self.content.len() > 2 && (
            self.content[0] == '*' as u32 ||
            self.content[0] == '-' as u32 ||
            self.content[0] == '_' as u32
        ) && {
            let break_ = remove_whitespaces(&self.content);

            break_.iter().all(|c| *c == self.content[0])
        }
    }

    #[inline]
    pub fn is_blockquote(&self) -> bool {
        self.indent < 4 && self.content.len() > 0
        && self.content[0] == '>' as u32 + HTML_ESCAPE_OFFSET
    }

    #[inline]
    pub fn is_unordered_list(&self) -> bool {
        self.content.len() > 1 && !self.is_thematic_break()
        && (self.content[0] == '-' as u32 || self.content[0] == '*' as u32)
        && self.content[1] == ' ' as u32 || (
            self.content.len() == 1 && self.content[0] == '-' as u32
        )
    }

    #[inline]
    pub fn is_ordered_list(&self) -> bool {

        // `1. ` ~ `999999999. `, `a. `, `A. `, `i. `, `I. `
        match self.content.iter().position(|c| *c == '.' as u32) {
            Some(ind) if ind + 1 < self.content.len() && self.content[ind + 1] == ' ' as u32 => {
                let marker = &self.content[0..ind];

                marker.iter().all(is_numeric) && match to_int(marker) {
                    Some(n) if n < 1_000_000_000 => true,
                    _ => false
                }
                || marker == &[97] || marker == &[65]
                || marker == &[105] || marker == &[73]
            },
            _ => false,
        }
    }

    #[inline]
    pub fn is_link_or_footnote_reference_definition(&self) -> bool {
        self.indent < 4 && self.content.len() > 4
        && self.content[0] == '[' as u32
        && match get_bracket_end_index(&self.content, 0) {
            None => false,
            Some(bracket_end_index) => self.content.len() > bracket_end_index + 2
            && self.content[bracket_end_index + 1] == ':' as u32
            && drop_while(&self.content[1..bracket_end_index], ' ' as u32).len() > 0 && (
                is_valid_link_label(&self.content[1..bracket_end_index]) ||
                is_valid_footnote_label(&self.content[1..bracket_end_index])
            )
        }

    }

    #[inline]
    pub fn is_multiline_macro(&self) -> bool {
        self.indent == 0 && read_macro(&self.content, 0).is_some() && {

            // trailing whitespaces are okay, but the other characters are not allowed
            let mut curr_index = get_bracket_end_index(&self.content, 0).unwrap() + 1;

            while curr_index < self.content.len() {

                if self.content[curr_index] != ' ' as u32 {
                    return false;
                }

                curr_index += 1;
            }

            true
        }

    }

}