/*
 the wording in this is absolute shit
He says each unoccupied row/column expands to "twice as big"
I mistakenly thought this meant they expanded to twice their length, rather this means the unoccupied sections just "double"
So you add another row/column adjacent to them
*/
pub fn bresenham(((x1, y1), (x2, y2)): ((usize, usize), (usize, usize))) -> usize {
    // Not needed, what the hell?!
    // To get from point A->B, we can calculate distance by simply summing difference of x and y coords
    // think about it, to get from one to the other, you can to cover a fixed number of pixel regardless of the route
    // idk why I was about to implement bresenham!!
    todo!()
}

pub fn find_dists(
    galaxies: &Vec<(usize, usize)>,
    col: &Vec<usize>,
    row: &Vec<usize>,
    expansion: usize,
) -> usize {
    let mut galaxies = galaxies.clone(); // I'm not proud of this and I could do a better way but that unecessarily convolute as solution that is already good and fast (~200us overall) as it is
    let expansion = expansion - 1;

    for (x, y) in galaxies.iter_mut() {
        *x += col[..*x].iter().filter(|k| **k == 1).count() * expansion;
        *y += row[..*y].iter().filter(|k| **k == 1).count() * expansion;
    }

    galaxies
        .iter()
        .enumerate()
        .map(|(idx, &p1)| galaxies[idx + 1..].iter().map(move |&p2| (p1, p2)))
        .flatten()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum()
}

pub fn main(input: &str) -> (usize, usize) {
    let row_len = input.lines().next().unwrap().len() + 1; // include \n, col vec will have additional entry but doesn't matter
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

    let p1 = find_dists(&galaxies, &col, &row, 2);
    let p2 = find_dists(&galaxies, &col, &row, 1000000);

    (p1, p2)
}
