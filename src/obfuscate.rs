// WIP

pub fn hash(content: &[u16]) -> u128 {

    let mut result = 0;
    let mut sum = 0;
    let limit: u128 = 1 << 120;

    for (i, c) in content.iter().enumerate() {

        result *= 139;
        result += *c as u128 % 128;
        result += i as u128 % 11;

        sum += *c as u128;

        if i > 14 {
            result %= limit;
        }

    }

    // it adds 1 so that it never becomes 0
    result + sum + 1
}

pub fn num2char(num: u128) -> u16 {

    #[cfg(test)]
    assert!(num < 62);

    if num < 10 {
        '0' as u16 + num as u16
    }

    else if num < 36 {
        'A' as u16 + num as u16 - 10
    }

    else {
        'a' as u16 + num as u16 - 36
    }

}

pub fn obfuscate(content: &[u16]) -> Vec<u16> {

    let mut hash = hash(content);
    let mut result = vec![];

    while hash > 0 {
        result.push(num2char(hash % 62));
        hash /= 62;
    }

    result
}
