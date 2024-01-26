use crate::utils::parse_int;

pub fn main(input: &str) -> (usize, usize) {
    let mut input = input.split("\n");
    let seeds = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(parse_int)
        .collect::<Vec<_>>();

    let mut maps = vec![];
    loop {
        let map = input
            .by_ref()
            .skip_while(|a| !matches!(a.as_bytes().get(0), Some(b'0'..=b'9')))
            .take_while(|a| a.len() > 0)
            .map(|a| a.split_ascii_whitespace().map(parse_int))
            .map(|mut range| {
                (
                    // it's like jquery with js, I refuse to cave and use itertools
                    range.next().unwrap(),
                    range.next().unwrap(),
                    range.next().unwrap(),
                )
            })
            .map(|(dst, src, len)| (dst, src, src + len))
            .collect::<Vec<_>>();
        if map.len() == 0 {
            break;
        }
        maps.push(map)
    }

    let part1 = seeds
        .iter()
        .map(|a| {
            maps.iter().fold(*a, |acc, x| {
                x.iter()
                    .find(|&&(_, src, src_end)| (src..src_end).contains(&acc))
                    .map_or(acc, |&(dst, src, _)| dst + acc - src)
            })
        })
        .min()
        .unwrap();

    let seed_ranges = seeds
        .chunks_exact(2)
        .map(|a| (a[0], a[1] + a[0]))
        .collect::<Vec<_>>();

    let y = maps.iter().fold(seed_ranges, |seed_ranges, map_group| {
        seed_ranges
            .iter()
            .flat_map(|&range| {
                let mut mapped = vec![];
                let mut unmapped = vec![range];
                let mut new_unmapped: Vec<(usize, usize)> = vec![];
                for &(dst, src, src_end) in map_group {
                    for (seed_start, seed_end) in unmapped.drain(..) {
                        let left_region = (seed_start, seed_end.min(src)); // the second part ensures we get the correct leftward dangling bit even if the interval overlaps to the right
                        let included_region = (seed_start.max(src), seed_end.min(src_end)); // max, min just ensures that even if the start or end exceed the bounds we get a valid internal interval
                        let right_region = (seed_start.max(src_end), seed_end); // max with the end ensures that even if seed_start is contained in interval we can get a valid dangling right bit

                        // think about it, the first check would only be invalid if the range is fully within - clever! Kill two birds with one stone
                        if left_region.0 < left_region.1 {
                            new_unmapped.push(left_region)
                        }
                        if right_region.0 < right_region.1 {
                            new_unmapped.push(right_region)
                        }
                        if included_region.0 < included_region.1 {
                            mapped.push((
                                dst + included_region.0 - src,
                                dst + included_region.1 - src,
                            ))
                        }
                    }
                    unmapped.extend(new_unmapped.drain(..)); // doing this helps avoid uncessary allocations (drain reduces len but not cap)
                }
                mapped.extend(unmapped); // woah, rust implicitly calls .drain() for unmapped!
                mapped
            })
            .collect::<Vec<_>>()
    });

    let part2 = y.iter().map(|r| r.0).min().unwrap();

    (part1, part2)
}
