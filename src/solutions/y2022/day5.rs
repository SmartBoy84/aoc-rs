type Stack = Vec<Vec<char>>;

fn extract_id(stack: Stack) -> String {
    stack.iter().map(|a| a.last().unwrap()).collect::<String>()
}

pub fn main(input: &str) -> (String, String) {
    let (stack, instructions) = input.split_once("\n\n").unwrap();

    let mut stack = stack.lines();
    stack.by_ref().next(); // hack
    let mut stack = stack.rev();

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
    let pattern = [5, 5, 3]; // "move ", "from ", " to"
    let instructions = instructions
        .split("\n")
        .map(|a| {
            let mut iter = a.as_bytes().iter();
            let mut nums = [0; 3];
            let mut i = 0;
            for x in pattern {
                for _ in 0..x {
                    iter.next().unwrap();
                }
                nums[i] = iter
                    .by_ref()
                    .take_while(|x| x.is_ascii_digit())
                    .fold(0, |acc, x| (acc * 10) + (x - b'0')) as usize;
                i += 1;
            }
            nums
        })
        .collect::<Vec<_>>();

    let mut part1 = stack.clone();

    let mut part2 = stack;
    let part2_mut: &mut Vec<Vec<char>> = unsafe { std::mem::transmute(&mut part2 as *mut _) }; // desperate times, desperate measures

    // the above is not only unsafe in the literal sense but also in that it's not intended, so it may be "fixed" at any time? (apparently)

    for [n, from, to] in instructions {
        for _ in 0..n {
            let val = part1[from - 1].pop().unwrap();
            part1[to - 1].push(val);
        }

        let start = part2[from - 1].len() - n;
        part2[to - 1].extend(part2_mut[from - 1].drain(start..));
    }

    (extract_id(part1), extract_id(part2))
}
