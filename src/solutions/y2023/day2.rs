use super::*;

// ["red", "green", "blue"]; => reference
// readability?? Pshaw!

pub fn main() -> Result<()> {
    let input: Vec<_> = get_input(2, 2023)?
        .trim()
        .split("\n")
        .map(|line| {
            line.split(": ")
                .last() // get the game
                .unwrap()
                .split("; ") // split games
                .map(|subset| subset.split(", ").map(|color| color.split(" "))) // split colours
                .fold(vec![Vec::new(); 3], |mut acc, subset| {
                    for mut side in subset {
                        let val: u32 = str::parse(side.next().unwrap()).unwrap();
                        let colour = side.next().unwrap();
                        match colour {
                            "red" => acc[0].push(val),
                            "green" => acc[1].push(val),
                            "blue" => acc[2].push(val),
                            _ => println!("Uh..."),
                        }
                    }
                    acc
                })
        })
        .collect();

    let part1 = input
        .iter()
        .map(|game| {
            game.iter()
                .map(|colour| colour.iter().max().unwrap_or(&0))
                .collect::<Vec<&u32>>()
        })
        .enumerate()
        .filter_map(|(id, game)| {
            if *game[0] <= 12 && *game[1] <= 13 && *game[2] <= 14 {
                Some(id + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    let part2 = input
        .iter()
        .map(|game| {
            game.iter()
                .map(|colour| colour.iter().max().unwrap_or(&1))
                .fold(1, |acc, x| acc * x)
        })
        .sum::<u32>();

    println!("{:?}", part1);
    println!("{:?}", part2);

    Ok(())
}
