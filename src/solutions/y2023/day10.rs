// p1: 7145
// p2: 445
#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl std::ops::Not for Direction {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

fn get_dir(input: &[u8], idx: usize, from: Direction) -> Option<(usize, Direction)> {
    let k = match input[idx] {
        b'|' => [Direction::Up, Direction::Down],
        b'-' => [Direction::Left, Direction::Right],
        b'L' => [Direction::Up, Direction::Right],
        b'J' => [Direction::Left, Direction::Up],
        b'7' => [Direction::Left, Direction::Down],
        b'F' => [Direction::Right, Direction::Down],
        _ => unreachable!(),
    };

    k.iter().position(|&d| d == !from).map(|i| (idx, k[1 - i])) // d == !from -> if the two directions are complements (left/right, up/down)
}

fn get_around(
    input: &[u8],
    idx: usize,
    line_len: usize,
) -> impl Iterator<Item = (usize, Direction)> + '_ {
    [
        (idx.wrapping_sub(1), Direction::Left), // I don't bother to check this against usize::MAX as I won't have a puzzle input of len() >~ usize::MAX
        (idx + 1, Direction::Right),
        (idx.wrapping_sub(line_len), Direction::Up),
        (idx + line_len, Direction::Down), // these are directions that the point from which the indexes are computed should have a slot for
    ]
    .into_iter()
    .filter(move |&(x, _)| matches!(input.get(x), Some(&x) if !matches!(x, b'\n' | b'.' | b'S')))
}

fn find_next(
    (idx, target_dir): (usize, Direction),
    input: &[u8],
    line_len: usize,
) -> Option<(usize, Direction)> {
    get_around(input, idx, line_len)
        .filter(|&(_, dir)| dir == target_dir) // first ensure that the "look" direction matches the required direction
        .find_map(|(idx, _)| get_dir(input, idx, target_dir)) // ensure the item in this ideal direction has the correct slot for us
}

fn print_path(input: &[u8], path: &Vec<(usize, usize)>) {
    println!(
        "{}",
        input
            .iter()
            .map(|&c| c as char)
            .enumerate()
            .map(|(i, c)| {
                if c == '\n' || path.iter().any(|(idx, _)| *idx == i) {
                    c
                } else {
                    '.'
                }
            })
            .collect::<String>()
    );
}

pub fn main(input: &str) -> (usize, usize) {
    let bytes = input.as_bytes();
    let start = bytes.iter().position(|&c| c == b'S').expect("no 'S'??");
    let line_len = bytes.iter().take_while(|&&c| c != b'\n').count() + 1;

    let branches = get_around(bytes, start, line_len)
        .filter_map(|(i, dir)| get_dir(bytes, i, dir))
        .collect::<Vec<_>>();

    let mut path_map = vec![(start % line_len, start / line_len)]; // integer division always truncating (simply pop of the shizzle after decimal so round down)
    let mut seen_ends = vec![];

    for path in branches {
        let mut current = path;
        if seen_ends.contains(&current.0) {
            break;
        }
        path_map.truncate(1);

        loop {
            path_map.push((current.0 % line_len, current.0 / line_len));
            if let Some(point) = find_next(current, bytes, line_len) {
                current = point;
            } else {
                break;
            }
        }
        seen_ends.push(current.0);
    }

    // print_path(bytes, &path_map);
    // path_map.reverse();
    // let p2 = path_map
    //     .iter()
    //     .map(|a| (a.0 as isize, a.1 as isize))
    //     .collect::<Vec<_>>()
    //     .windows(2)
    //     .fold(0, |acc, matrix| {
    //         acc + (matrix[0].0 * matrix[1].1) - (matrix[1].0 * matrix[0].1)
    //     })
    //     / 2;
    // println!("{}", p2.abs() - (path_map.len() / 2) as isize);

    let p1 = path_map.len() / 2;
    todo!("can't figure out part 2 :(");
    (p1, 0)
}
