mod samples;
mod html;

pub fn random(mut seed: usize) -> usize {

    for _ in 0..3 {
        seed = ((seed % 21 + seed % 23 + seed % 25) * 821 + (seed % 27 + seed % 29 + seed % 31) * 823 + (seed % 33 + seed % 35 + seed % 37) * 827 + (seed % 39 + seed % 41 + seed % 43) * 829) % 65536;
    }

    seed
}

pub fn remove_whitespaces(line: &[u16]) -> Vec<u16> {
    line.iter().filter(
        |c| **c != ' ' as u16 && **c != '\n' as u16 && **c != '\t' as u16
    ).map(|c| *c).collect()
}