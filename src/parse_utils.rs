pub fn extract_digits_n(a: &str, n: usize) -> Vec<u32> {
    let mut x = Vec::with_capacity(n);
    let mut i = 0;
    let mut a = a.as_bytes().iter();
    loop {
        if let Some(c) = a.next() {
            if !c.is_ascii_digit() {
                continue;
            }
            let mut num = c - b'0';
            for c in a.by_ref() {
                if !c.is_ascii_digit() {
                    break;
                }
                num = (num * 10) + (c - b'0');
            }
            x[i] = num as u32;
            i += 1;
        } else {
            break;
        }
    }
    x
}