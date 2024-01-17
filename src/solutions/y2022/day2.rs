pub fn main(input: &str) -> (u32, u32) {
    let input = input
        .as_bytes()
        .chunks_exact(4)
        .map(|a| (a[0] as i8 - b'A' as i8, a[2] as i8 - b'X' as i8))
        .collect::<Vec<_>>(); // note to self, prefer chunks over '\n'

    let part1 = input
        .iter()
        .map(|a| {
            let choice_score = a.1 + 1;
            let game_score = match a.1 - a.0 {
                0 => 3,              // draw
                1 | -2 => 6,         // win
                -1 | 2 => 0,         // loss
                _ => unreachable!(), // parsing error
            };

            (choice_score + game_score) as u32
        })
        .sum();

    let part2 = input
        .iter()
        .map(|a| {

            let choice_score = match a.1 {
                0 => match a.0 - 1 {
                    -1 => 2,
                    x => x,
                }, // loss
                1 => a.0, // drew
                2 => match a.0 + 1 {
                    3 => 0,
                    x => x,
                }, // win
                _ => unreachable!(),
            }
            .clamp(0, 2)
                + 1;

            let game_score = match a.1 {
                0 => 0,              // must lose
                1 => 3,              // must draw
                2 => 6,              // must win
                _ => unreachable!(), // parsing error
            };

            (choice_score + game_score) as u32
        })
        .sum();

    (part1, part2)
}
