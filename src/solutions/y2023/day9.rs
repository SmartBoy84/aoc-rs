pub fn main(input: &str) -> (isize, isize) {
    input
        .split("\n")
        .map(|a| {
            a.split(" ")
                .map(|a| a.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|mut a| {
            let mut back = Vec::with_capacity(a.len());
            back.push(a[0]);
            for i in (0..a.len() - 1).rev() {
                for x in 0..=i {
                    a[x] = a[x + 1] - a[x];
                }
                back.push(a[0]);
            }
            back.reverse(); // I don't even care at this point, screw speed
            for x in 0..a.len() - 1 {
                a[x + 1] += a[x];
                back[x + 1] -= back[x];
            }
            (*a.last().unwrap(), *back.last().unwrap())
        })
        .fold((0, 0), |mut acc, x| {
            acc.0 += x.0;
            acc.1 += x.1;
            acc
        })
}
