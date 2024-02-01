/*
 the wording in this is absolute shit
He says each unoccupied row/column expands to "twice as big"
I mistakenly thought this meant they expanded to twice their length, rather this means the unoccupied sections just "double"
So you add another row/column adjacent to them
*/
pub fn bresenham(((x1, y1), (x2, y2)): ((usize, usize), (usize, usize))) -> usize {
    todo!()
}

pub fn find_dists(
    galaxies: &Vec<(usize, usize)>,
    expansion: usize,
    col: &Vec<usize>,
    row: &Vec<usize>,
) -> usize {
    let mut galaxies = galaxies.clone();
    let expansion = expansion - 1;

    for (x, y) in &mut galaxies {
        *x += col[..*x].iter().map(|k| k * expansion).sum::<usize>();
        *y += row[..*y].iter().map(|k| k * expansion).sum::<usize>();
    }
    println!("{:?}", galaxies);
    galaxies
        .iter()
        .enumerate()
        .map(|(idx, &g1)| galaxies[idx + 1..].iter().map(move |&g2| (g1, g2)))
        .flatten()
        .map(bresenham)
        .sum()
}

pub fn main(input: &str) -> (usize, usize) {
    let row_len = input.as_bytes().iter().take_while(|c| **c != b'\n').count();
    let col_len = input.lines().count();

    let galaxies = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (*c == b'#').then_some(i))
        .map(|idx| ((idx % row_len), idx / row_len)) // convert to x/y coordinates
        .collect::<Vec<_>>();

    let mut col = vec![1; row_len];
    let mut row = vec![1; col_len];
    for (y, r) in input.split("\n").map(|a| a.as_bytes()).enumerate() {
        for x in r
            .iter()
            .enumerate()
            .filter_map(|(x, c)| (*c != b'.').then_some(x))
        {
            col[x] = 0; // if there are multiple galaxies in this row, this will run multiple times but there's minimal overhead so no need for ugly flagging
            row[y] = 0;
        }
    }

    let p1 = find_dists(&galaxies, 2, &col, &row);
    let p2 = find_dists(&galaxies, 1000000, &col, &row);

    todo!();
}
