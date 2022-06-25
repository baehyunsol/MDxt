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
    pub fn is_code_fence(&self) -> bool {
        self.indent == 0 && self.content.len() > 2 && &self.content[0..3] == &into_v16("```")
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
}