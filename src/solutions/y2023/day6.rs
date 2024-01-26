fn f(&(t, d) : &(usize, usize)) -> usize {
    // println!("{t} {d}");
    let diff = ((t*t - 4*d) as f64).sqrt();
    let l = diff / 2.0;
    let h = diff / 2.0;
    (h.floor() + l.ceil()) as usize + 1
  }

pub fn main(input: &str) -> (usize, usize) {
    let mut input = input.split("\n").map(|a| {
        a.split_ascii_whitespace()
            .filter_map(|b| b.parse::<f32>().ok())
    });

    let entries = input
        .next()
        .unwrap()
        .zip(input.next().unwrap())
        .collect::<Vec<_>>();

    let part1: usize = entries
        .iter()
        // .map(f)
        .map(|&(time, dist)| {
            let x = ((time * time) - (4.0 * dist)).sqrt();
            ((x * 0.5).ceil() + (x * 0.5).floor()) as usize
        })
        .product();
    println!("{:?}", part1);

    todo!()
}
