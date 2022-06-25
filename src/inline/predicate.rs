pub enum Bool {
    False,
    True(usize)
}

pub fn is_code_span(content: &[u16], index: usize) -> Bool {

    if !is_code_span_start(content, index) {
        return Bool::False;
    }

    let mut end_index = content.len() - 1;

    while end_index > index {

        if is_code_span_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index -= 1;
    }

    Bool::False
}

fn is_code_span_start(content: &[u16], index: usize) -> bool {
    todo!()
}

fn is_code_span_end(content: &[u16], index: usize) -> bool {
    todo!()
}

pub fn is_italic(content: &[u16], index: usize) -> Bool {

    if !is_italic_start(content, index) {
        return Bool::False;
    }

    let mut end_index = content.len() - 1;

    while end_index > index {

        if is_italic_end(content, end_index) {
            return Bool::True(end_index);
        }

        end_index -= 1;
    }

    Bool::False
}

fn is_italic_start(content: &[u16], index: usize) -> bool {
    todo!()
}

fn is_italic_end(content: &[u16], index: usize) -> bool {
    todo!()
}