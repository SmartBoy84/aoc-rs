/*
Part 1: 30575
Part 2: 37478
Time taken: 234.148µs
*/

#[derive(Debug)]
struct Region {
    lines: Vec<String>,
}

impl Region {
    fn new_hoz(region: &str) -> Self {
        Self {
            lines: region.split("\n").map(|a| a.to_string()).collect(),
        }
    }

    fn new_vert(region: &str) -> Self {
        let mut cols = vec![];
        for line in region.split("\n") {
            if cols.len() == 0 {
                cols.extend(std::iter::once(String::new()).cycle().take(line.len()))
            }
            for (n, c) in line.char_indices() {
                cols[n].push(c);
            }
        }
        Self { lines: cols }
    }

    fn find_sym_lin(&self) -> (Option<usize>, Option<usize>) {
        self.find_symmetry(true, self.find_symmetry(false, (None, None)))
    }

    fn find_symmetry(
        &self,
        rev: bool,
        (mut perfect, mut smudge): (Option<usize>, Option<usize>),
    ) -> (Option<usize>, Option<usize>) {
        let max = self.lines.len() / 2;

        for i in 1..max + 1 {
            if let (Some(_), Some(_)) = (perfect, smudge) {
                break;
            }

            let first_half = &self.lines[if rev {
                self.lines.len() - i..self.lines.len()
            } else {
                0..i
            }];
            let second_half = &self.lines[if rev {
                self.lines.len() - (i * 2)..self.lines.len() - i
            } else {
                i..i * 2
            }];

            let diff = first_half
                .iter()
                .zip(second_half.iter().rev())
                .map(|(l1, l2)| {
                    l1.chars()
                        .zip(l2.chars())
                        .filter(|(c1, c2)| c1 != c2)
                        .take(3) // don't really care about any more discrepancies
                        .count()
                })
                .try_fold(0, |acc, x| match acc + x {
                    2.. => None, // >= 2
                    x => Some(x),
                });

            let index = if rev { self.lines.len() - i } else { i };
            match diff {
                Some(0) => perfect = Some(index),
                Some(1) => smudge = Some(index), // a single char was different between the two divisons around the line of symmetry
                _ => (),                         // more than two => don't care, move on
            };
        }
        (perfect, smudge)
    }
}

pub fn main(input: &str) -> (usize, usize) {
    input
        .split("\n\n")
        .map(|region| {
            (
                Region::new_hoz(region).find_sym_lin(),
                Region::new_vert(region).find_sym_lin(),
            )
        })
        .fold((0, 0), |(p1, p2), ((hp, hs), (vp, vs))| {
            (
                p1 + (hp.unwrap_or(0) * 100) + vp.unwrap_or(0),
                p2 + (hs.unwrap_or(0) * 100) + vs.unwrap_or(0),
            )
        })
}
