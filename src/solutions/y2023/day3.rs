/* INCOMPLETE FOR NOW */

use super::*;

fn part1(input: &str, line_length: usize) -> u32 {
    let mut popper: (Option<String>, bool) = (None, false);
    let mut collector = vec![];

    for (index, point) in input.char_indices() {
        if point.is_digit(10) {
            if let Some(val) = &mut popper.0 {
                val.push(point);
            } else {
                popper.0 = Some(String::from(point));
            }

            for y in -1..=1 {
                if input
                    .get(
                        (index as i32 - (line_length as i32 * y) - 1)
                            .try_into()
                            .unwrap_or(0)..,
                    )
                    .and_then(|x| {
                        x.chars()
                            .take(3)
                            .take_while(|char| char == &('\n' as char))
                            .find(|x| !matches!(x, '0'..='9' | '.'))
                    })
                    .is_some()
                {
                    popper.1 = true;
                }
            }
        } else if let Some(_) = popper.0 {
            let num = popper.0.take().unwrap();
            if popper.1 {
                collector.push(num);
            }
            popper.1 = false;
        }
    }
    println!("{:?}", collector);

    collector
        .iter()
        .map(|x| str::parse::<u32>(x).unwrap())
        .sum::<u32>()
}

pub fn main(input: &str) -> Result<()> {
    let input = get_input(3, 2023)?;
    let line_length = input.split("\n").next().unwrap().len();

    println!("{}", part1(&input, line_length));
    panic!("Part 2 not done yet");
    // part1(&input, line_length);
    Ok(())
}
