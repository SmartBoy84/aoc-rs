use crate::utils::parse_int;
use std::cmp::Ordering;

fn card_idx(c: u8, p2: bool) -> usize {
    match c {
        b'J' if p2 => 0,
        x => {
            (match x {
                b'2'..=b'9' => x - b'2',
                b'T' => 8,
                b'J' => 9,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                _ => unreachable!(),
            }) as usize
                + p2 as usize
        } /* spooky; casting bool to number */
    }
}

fn sort_by_rank(cards: &mut [(i32, &str, usize)], p2: bool) {
    cards.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => {
            a.1.as_bytes()
                .iter()
                .zip(b.1.as_bytes().iter())
                .find_map(|(&c1, &c2)| match card_idx(c1, p2).cmp(&card_idx(c2, p2)) {
                    Ordering::Equal => None,
                    x => Some(x),
                })
                .expect("Equal hand not possible") // I trust no hand will be equivalent
        }
        s => s, // greater than or equal to
    });
}

fn deck_value(cards: &[(i32, &str, usize)]) -> usize {
    cards
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &(_, _, bid))| acc + ((i + 1) * bid))
}

pub fn main(input: &str) -> (usize, usize) {
    let mut cards = input
        .split("\n")
        .map(|a| a.split_ascii_whitespace())
        .map(|mut a| (a.next().unwrap(), parse_int(a.next().unwrap())))
        .map(|(hand, bid)| {
            let (mut freq, ..) = hand.as_bytes().iter().fold(
                // this convoluted ass shizzle actually shaves off 100us
                ([0u8; 5], Vec::with_capacity(5), 0),
                |(mut store, mut lookup_table, joker_count), &x| {
                    let i = lookup_table
                        .iter()
                        .position(|&c| c == x)
                        .unwrap_or_else(|| {
                            lookup_table.push(x);
                            lookup_table.len() - 1
                        });
                    store[i] += 1;
                    (store, lookup_table, joker_count)
                },
            );
            freq.sort_unstable();
            let value = match &freq {
                &[.., 5] => 7,             // 7: five of a kind
                &[.., 1, 4] => 6,          // 6: four of a kind
                &[.., 2, 3] => 5,          // 5: full house
                &[.., 1, 1, 3] => 4,       // 4: three of a kind
                &[.., 1, 2, 2] => 3,       // 3: two pair
                &[.., 1, 1, 1, 2] => 2,    // 2: one pair
                &[.., 1, 1, 1, 1, 1] => 1, // 1: high card
                _ => unreachable!("{:?}", freq),
            };
            (value, hand, bid)
        })
        .collect::<Vec<_>>();

    sort_by_rank(&mut cards, false);
    let part1 = deck_value(&cards);

    // promote rank due to joker inclusion
    for (rank, name, _) in &mut cards {
        let jokers = name.chars().filter(|&a| a == 'J').count();
        if jokers > 0 {
            *rank = match *rank {
                6 => 7,
                5 => 7,
                4 => 6,
                3 => match jokers {
                    // ay yo, special case took forever to figure out
                    2 => 6,
                    _ => 5,
                },
                2 => 4,
                1 => 2,
                x => x,
            }
        }
    }
    sort_by_rank(&mut cards, true);
    let part2 = deck_value(&cards);

    (part1, part2)
}
