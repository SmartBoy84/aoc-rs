use crate::utils::parse_int;
use hashbrown::HashMap;

// I'm wary that recursion is rarely ideal but it's ok here since my inputs don't run the risk of filling up stack mem
fn find_permutations(
    map: &[u8],
    groups: &[usize],
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    // this caching system is not my idea and I would never have thought to do it because I didn't think the puzzle input would require such memoization
    // like it'd so absurd that this is used as much as it is; for it to be so, the input must have segments which are **exactly** identical!
    // I hate that I'm too retarded to come up with anything better than this - 70ms is crazy
    let key = (
        unsafe { String::from_utf8_unchecked(map.to_vec()) },
        groups.to_vec(),
    );
    if let Some(&ways) = cache.get(&key) {
        return ways;
    }

    let my_group = groups[0];
    let remaining = groups.iter().sum::<usize>() + groups.len() - 1;
    let mut ways = 0;

    // adapters I find to be better than if (...) {continue;}
    for i in (0..map.len())
        .filter(|i| map[*i] != b'.') // we can skip over '.' chunks as they can't comprise a group
        .take_while(|i| map.len() - *i >= remaining)
    // remaining section must be able to fit total remaining chunks + enough space for the smallest gap between them (a single space)
    {
        let end = i + my_group; // +1 to ensure that EOL error causes break here rather than panic!
        if matches!(map.get(end), Some(b'.' | b'?') | None) // cursor is on potentially start of next group
            && map[i..]
                .iter()
                .take_while(|c| matches!(**c, b'#' | b'?'))
                .count()
                >= my_group
        {
            if groups.len() == 1 {
                // must ensure that if this is the last group then we only consider it's search section as valid if no # linger beyond it
                if !map[end..].iter().any(|c| *c == b'#') {
                    ways += 1
                }
            } else {
                ways += find_permutations(&map[end + 1..], &groups[1..], cache)
            }
        }

        // once # crossed, iteration must stop as this point HAS to be the start at this stage
        if map[i] == b'#' {
            break; // since this '#' must comprise this group
        }
    }

    // def unique else function would've returned at the start (no threading happening)
    cache.insert_unique_unchecked(key, ways);
    ways
}

pub fn main(input: &str) -> (usize, usize) {
    let mut input = input
        .split("\n")
        .map(|a| a.split_once(" ").unwrap())
        .map(|(map, alt)| {
            (
                map.as_bytes().to_vec(),
                alt.split(",").map(parse_int).collect::<Vec<_>>(),
            )
        })
        .map(|mut x| {
            x.0.push(b'.');
            x
        })
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();

    let p1 = input
        .iter()
        // .map(|(map, alt)| {
        //     let ways = find_permutations(&map, &alt, &mut cache);
        //     println!("{:?} {:?} => {ways} ways", std::str::from_utf8(&map), alt);
        //     ways
        // })
        .map(|(map, alt)| find_permutations(&map, &alt, &mut cache))
        .sum();

    // bruh, I've given up at this point - the implementation is correct but it doesn't finish executing in under half an hour
    let p2 = input
        .iter_mut()
        .map(|x| {
            *x.0.last_mut().unwrap() = b'?';
            x
        })
        .map(|(map, alt)| {
            (
                Vec::from_iter(map.iter().copied().cycle().take((map.len() * 5) - 1)),
                Vec::from_iter(alt.iter().copied().cycle().take(alt.len() * 5)),
            )
        })
        .map(|(map, alt)| find_permutations(&map, &alt, &mut cache))
        .sum();

    (p1, p2)
}
