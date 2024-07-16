fn hash(a: &str) -> usize {
    a.chars()
        .map(|b| b as usize)
        .fold(0, |acc, cd| ((acc + cd) * 17) % 256)
}

pub fn main(input: &str) -> (usize, usize) {
    let part1 = input.split(",").map(hash).sum();

    let mut boxes: Vec<Vec<(&str, &str)>> = vec![vec![]; 256];
    for step in input.split(",") {
        let pos = step
            .find(|c| matches!(c, '=' | '-'))
            .expect(&format!("Uh, no inst found: {}", step));

        let label = &step[..pos];
        let ins = &step[pos..pos + 1];
        let lens = &step[pos + 1..];
        let boxn = hash(label);

        if let Some(ln) = boxes[boxn]
            .iter()
            .enumerate()
            .find_map(|(i, (lbl, ln))| (*lbl == label && ln.len() > 0).then_some(i))
        {
            boxes[boxn][ln].1 = lens;
        } else if ins == "=" {
            boxes[boxn].push((label, lens))
        }
    }

    let part2 = boxes
        .iter()
        .map(|a| {
            a.iter()
                .filter_map(|(_, ln)| ln.parse::<usize>().ok())
                .enumerate()
        })
        .enumerate()
        .fold(0, |acc, (box_n, lens_box)| {
            acc + lens_box.fold(0, |acc_in, (lens_slot, focal_length)| {
                acc_in + ((box_n + 1) * (lens_slot + 1) * focal_length)
            })
        });

    (part1, part2)
}
