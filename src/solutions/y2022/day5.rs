pub fn main(input: &str) -> (usize, usize) {
    let (stack, instructions) = input.split_once("\n\n").unwrap();

    let mut stack = stack.lines().rev();

    // extract crate count by getting bottom right number
    // zowhee mama! I could cheat and replace this with 9 but eh
    let len = stack
        .by_ref()
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .rev()
        .find(|a| a.is_ascii_digit())
        .unwrap()
        - b'0';

    // extract initial stack state
    let stack = stack.fold(vec![Vec::new(); len as usize], |mut acc, x| {
        for (i, a) in x.as_bytes().chunks(4).enumerate() {
            if a[1].is_ascii_alphabetic() {
                acc[i].push(a[1] as char);
            }
        }
        acc
    });

    // parse instructions
    let instructions = instructions.split("\n").map(|a| {

    });

    (stack[0][0] as usize, stack[0][1] as usize)
}
