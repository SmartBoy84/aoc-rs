use std::{
    ops::{AddAssign, Index},
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
} // (y, x)
impl Default for Coord {
    fn default() -> Self {
        Self { x: 1, y: 1 }
    }
}

struct Cave<'a> {
    cave: Vec<&'a [u8]>,
}

impl<'a> Index<Coord> for Cave<'a> {
    type Output = u8;
    fn index(&self, index: Coord) -> &Self::Output {
        &self.cave[index.y - 1][index.x - 1]
    }
}

impl AddAssign<Direction> for Coord {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        };
    }
}

impl<'a> Cave<'a> {
    fn from_input(input: &'a str) -> Self {
        Self {
            cave: input.split("\n").map(|a| a.as_bytes()).collect(),
        }
    }

    fn follow_path(&self, start: Coord, direction: Direction) -> Vec<Coord> {
        let mut energised: Vec<Coord> = Vec::new();

        let mut current_coord = start;
        let mut current_direction = direction;

        while (1..self.cave.len() + 1).contains(&current_coord.y)
            && (1..self.cave[0].len() + 1).contains(&current_coord.x)
        {
            if matches!(self[current_coord], b'|' | b'-' | b'/' | b'\\') {
                energised.push(current_coord);
            }
            current_direction = match self[current_coord] {
                b'|' => {
                    energised.extend(self.follow_path(current_coord, Direction::Up).iter());
                    Direction::Down
                } // chosen arbitrarily
                b'-' => {
                    energised.extend(self.follow_path(current_coord, Direction::Left).iter());
                    Direction::Right
                }
                b'/' => match current_direction {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    _ => unreachable!("/ encountered but moving {:?}", current_direction),
                },
                b'\\' => match current_direction {
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Right,
                    _ => unreachable!("\\ encountered but moving {:?}", current_direction),
                },
                b'.' => current_direction,
                c => unreachable!("unknown symbol: {}", c as char),
            };
            current_coord += direction;
        }
        energised
    }
}

pub fn main(input: &str) -> (usize, usize) {
    let cave = Cave::from_input(input);
    let energies = cave.follow_path(Coord::default(), Direction::Right);
    println!("{:?}", energies);
    // follow_path(&cave, Coord(0, 0), Direction::Left);
    todo!()
}
