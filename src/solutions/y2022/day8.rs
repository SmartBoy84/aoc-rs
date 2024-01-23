// this solutions relies on a square plot
pub fn main(input: &str) -> (usize, u8) {
    let input = input
        .split("\n")
        .map(|a| a.bytes().map(|b| b - b'0').collect())
        .collect::<Vec<Vec<_>>>();

    let map_width = input.len();
    let level_n = (map_width as f32 / 2.0).ceil() as usize;

    let mut visible_trees = (map_width - 1) * 4; // # trees on edge (always visible)
    let mut max_edges = vec![vec![0; map_width]; 4]; //[top, bottom, left, right]

    // for x in 0..map_width {
    //     max_edges[0].push(input[0][x]);
    //     max_edges[1].push(input[map_width - 1][x]);
    //     max_edges[2].push(input[x][0]);
    //     max_edges[3].push(input[x][map_width - 1]);
    // }

    for level in 0..level_n {
        let end = map_width - level - 1;
        for point in level..=end {
            let mut visible = false;
            // upper edge
            if max_edges[0][point] < input[level][point] {
                max_edges[0][point] = input[level][point];
                if end - level == 0 {
                    visible = true;
                } else {
                    visible_trees += 1;
                }
            }

            // bottom edge
            if max_edges[1][point] < input[end][point] {
                max_edges[1][point] = input[end][point];
                if end - level == 0 {
                    visible = true;
                } else {
                    visible_trees += 1;
                }
            }

            // left edge
            if max_edges[2][point] < input[point][level] {
                max_edges[2][point] = input[point][level];
                if end - level == 0 {
                    visible = true;
                } else {
                    visible_trees += 1;
                }
            }

            // right edge
            if max_edges[3][point] < input[point][end] {
                max_edges[3][point] = input[point][end];
                if end - level == 0 {
                    visible = true;
                } else {
                    visible_trees += 1;
                }
            }

            if visible {
                visible_trees += 1;
            }
        }
        // println!("{:?}", max_edges);
    }

    (visible_trees, max_edges[0][0])
}
