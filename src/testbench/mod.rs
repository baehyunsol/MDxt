pub fn random(mut seed: usize) -> usize {

    for _ in 0..3 {
        seed = ((seed % 21 + seed % 23 + seed % 25) * 821 + (seed % 27 + seed % 29 + seed % 31) * 823 + (seed % 33 + seed % 35 + seed % 37) * 827 + (seed % 39 + seed % 41 + seed % 43) * 829) % 65536;
    }

    seed
}

#[test]
fn renderer() {
    use crate::render_reference;
    use std::fs::File;
    use std::io::Write;

    let mut f = File::create("reference.html").unwrap();
    f.write_all(render_reference().as_bytes()).unwrap();
}
