use std::fs::File;
use std::io::{Read, Write};
use crate::{render_to_html, RenderOption, RenderResult};

// hxml is not thread-safe
pub static mut HXML_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn render_reference_test() {
    let lock = unsafe {
        HXML_LOCK.lock().unwrap()
    };

    let mut reference = String::new();
    let mut f = File::open("./reference.md").unwrap();
    f.read_to_string(&mut reference).unwrap();

    let RenderResult {
        content: reference,
        has_collapsible_table,
        has_tooltip,
        has_sidebar,
        ..
    } = render_to_html(
        &reference,
        RenderOption {
            xml: true,
            ..RenderOption::default()
        }
    );

    hxml::into_dom(reference.clone()).unwrap();

    assert!(has_collapsible_table);
    assert!(has_tooltip);
    assert!(has_sidebar);

    // TODO: more queries
    // TODO: same stuff for math_test.html, icon_test.html, and character_test.html

    let mut f = File::create("./reference.html").unwrap();
    f.write_all(reference.as_bytes()).unwrap();

    drop(lock);
}
