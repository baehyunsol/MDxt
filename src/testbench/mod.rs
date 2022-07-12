pub fn random(mut seed: usize) -> usize {

    for _ in 0..3 {
        seed = ((seed % 21 + seed % 23 + seed % 25) * 821 + (seed % 27 + seed % 29 + seed % 31) * 823 + (seed % 33 + seed % 35 + seed % 37) * 827 + (seed % 39 + seed % 41 + seed % 43) * 829) % 65536;
    }

    seed
}

#[test]
fn renderer() {
    use crate::render_to_html_with_default_options;
    use std::fs::File;
    use std::io::{Write, Read};

    let mut s = String::new();

    let md = match File::open("showcase.md") {
        Err(_) => Err(()),
        Ok(mut f) => match f.read_to_string(&mut s) {
            Err(_) => Err(()),
            Ok(_) => Ok(s)
        }
    };

    let html = render_to_html_with_default_options(&md.unwrap());

    let result = match File::create("showcase.html") {
        Err(_) => Err(()),
        Ok(mut f) => match f.write_all(html.as_bytes()) {
            Err(_) => Err(()),
            Ok(_) => Ok(())
        }
    };

    result.unwrap();
}
