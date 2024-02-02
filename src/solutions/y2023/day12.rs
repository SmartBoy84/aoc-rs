use crate::utils::parse_int;

// I'm wary that recursion is rarely ideal but it's ok here since my inputs don't run the risk of filling up stack mem
fn find_permutations(map: &[u8], groups: &[usize]) -> usize {
    let my_group = groups[0];
    let remaining = groups.iter().sum::<usize>() + groups.len() - 1;
    let mut ways = 0;

    // adapters I find to be better than if (...) {continue;}
    for i in (0..map.len())
        .filter(|i| map[*i] != b'.') // we can skip over '.' chunks as they can't comprise a group
        .take_while(|i| map.len() - *i >= remaining)
    // remaining bit must be able to fit total remaining chunks + enough space for the smallest gap between them (a single space)
    {
        println!("{:?} {:?}", &map[i..], groups);
        let end = i + my_group; // +1 to ensure that EOL error causes break here rather than panic!
        if matches!(map.get(end), Some(b'.' | b'?') | None) // cursor is on potentially start of next group
            && map[i..]
                .iter()
                .take_while(|c| matches!(**c, b'#' | b'?'))
                .count()
                >= my_group
        {
            ways += if groups.len() == 1 {
                println!("Here");
                1
            } else {
                find_permutations(&map[end..], &groups[1..])
            }
        }

        if map[i] == b'#' {
            break; // since this '#' must comprise this group
        }
    }
    ways
}

pub fn main(input: &str) -> (usize, usize) {
    let input = input
        .split("\n")
        .map(|a| a.split_once(" ").unwrap())
        .map(|(map, alt)| {
            (
                map.as_bytes(),
                alt.split(",").map(parse_int).collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let p1 = input
        .iter()
        .map(|(map, alt)| find_permutations(&map, &alt))
        .collect::<Vec<_>>();

    println!("{:?}", p1);
    todo!()
}