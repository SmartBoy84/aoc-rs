fn extract_num(input: &str, end_c: u8) -> Option<(usize, usize)> {
    let mut num = 0;
    for (i, c) in input.as_bytes().iter().enumerate() {
        if c == &end_c {
            return (i > 0).then_some((i, num)); // first char can't just be ,
        }
        if !matches!(c, b'0'..=b'9') {
            return None; // must end at ','
        }
        num = (num * 10) + (c - b'0') as usize;
    }
    return None; // end of string - didn't see end_c
}

pub fn get_nums(input: &str) -> Option<((usize, usize), usize)> {
    if let Some((offset_1, digit_1)) = extract_num(input, b',')
        && let Some((offset_2, digit_2)) = extract_num(&input[offset_1 + 1..], b')')
    {
        Some(((digit_1, digit_2), offset_1 + offset_2))
    } else {
        None
    }
}

pub fn my_sol(input: &str) -> (usize, usize) {
    let i_bytes = input.as_bytes();
    let mut i = 0;

    let mut part1 = 0;
    let mut part2 = 0;
    let mut enabled = true; // start enabled

    while i < i_bytes.len() {
        if i_bytes[i..].starts_with(b"mul(") {
            i += 4;
            if let Some(((d1, d2), offset)) = get_nums(&input[i..]) {
                let p = d1 * d2;
                part1 += p;
                if enabled {
                    part2 += p;
                }
                i += offset;
            }
        } else if i_bytes[i..].starts_with(b"do()") {
            enabled = true;
        } else if i_bytes[i..].starts_with(b"don't()") {
            enabled = false;
        }
        i += 1;
    }
    (part1, part2)
}

pub fn main(input: &str) -> (usize, usize) {
    // ape_sol(input) // p1: 183788984, p2: 62098619
    my_sol(input)
}
