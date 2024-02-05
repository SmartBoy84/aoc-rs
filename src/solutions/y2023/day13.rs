/*
Part 1: 30575
Part 2: 37478
Time taken: 234.148µs
*/

fn find_reflection(groups: &Vec<(usize, String)>) -> Option<(usize, usize)> {
    let mut true_end = 0;

    let mut needle = 0;
    let mut start = 0;
    let mut max_end = None::<usize>;

    while let Some((_, group)) = groups.get(needle) {
        if let Some(right_end) = groups[needle + 1..]
            .iter()
            .find_map(|(check_idx, check_group)| (check_group == group).then_some(*check_idx))
        {
            if max_end.is_none() || !matches!(max_end.unwrap().checked_sub(right_end), Some(1)) {
                true_end = right_end;
                start = needle;
            }
            max_end = Some(right_end); // doesn't matter if this is to the left (ideal) OR to the right
        } else {
            max_end = None; // due to a violating group, this entire reflection must be negated
        }
        needle += 1;

        // this occurs when the scope has been narrowed without finding a violating group such that we are at the centre [pair]
        // *relies on the assumption that all reflections are of n-groups where n is even
        if matches!(max_end, Some(x) if x == needle) {
            return Some((start, true_end));
        }
    }
    return None; // this means we've gone up step-by-step and reached the end without finding ANY reflection
}

pub fn main(input: &str) -> (usize, usize) {
    let line_len = input.lines().next().unwrap().len();

    let hoz_lines = input
        .split("\n\n")
        .map(|a| {
            a.split("\n")
                .map(|b| b.to_string())
                .enumerate()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let vert_lines = hoz_lines
        .iter()
        .map(|group| {
            group.iter().fold(
                (0..line_len)
                    .map(|idx| (idx, String::new()))
                    .collect::<Vec<_>>(),
                |mut acc, (_, x)| {
                    for (idx, c) in x.char_indices() {
                        acc[idx].1.push(c)
                    }
                    acc
                },
            )
        })
        .collect::<Vec<_>>();

println!("{:?}", vert_lines);

    let horizontal = hoz_lines.iter().map(find_reflection).collect::<Vec<_>>();
    let vertical = vert_lines.iter().map(find_reflection).collect::<Vec<_>>();

    println!("Hoz: {:?} \n\nVert: {:?}", horizontal, vertical);

    todo!()
}
