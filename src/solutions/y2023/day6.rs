fn way_finder(&(t, d): &(f64, f64)) -> usize {
    let det = f64::sqrt(t.powi(2) - (4.0 * d));

    let sol1 = (0.5 * (t - det)).clamp(0.0, f64::MAX);
    let sol1 = if sol1.fract() == 0.0 {
        sol1 + 1.0
    } else {
        sol1.ceil()
    };

    let sol2 = (t + det) * 0.5;
    let sol2 = if sol2.fract() == 0.0 {
        sol2 - 1.0
    } else {
        sol2.floor()
    };

    (sol2 - sol1 + 1.0) as usize
}

pub fn main(input: &str) -> (usize, usize) {
    let mut iter = input.split("\n").map(|a| {
        a.split_ascii_whitespace()
            .filter_map(|b| b.parse::<f64>().ok())
    });

    let entries = iter
        .next()
        .unwrap()
        .zip(iter.next().unwrap())
        .collect::<Vec<_>>();

    let p1 = entries.iter().map(way_finder).product();

    let mut iter = input.split("\n").map(|a| {
        let b = a
            .chars()
            .filter(|a| matches!(a, '0'..='9'))
            .collect::<String>()
            .parse::<f64>()
            .unwrap();
        b
    });
    let p2 = way_finder(&(iter.next().unwrap(), iter.next().unwrap()));

    (p1, p2)
}
