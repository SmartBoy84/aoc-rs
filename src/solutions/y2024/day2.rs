use crate::{util::parse::ParseOps, utils};

use super::scratch::ape_sol;

pub fn check(arr: &[i32], cur_ord: &mut Option<i32>) -> bool {
    let nex_ord = arr[1] - arr[0];
    if let Some(cur_ord) = cur_ord
        && *cur_ord != 0
    {
        if cur_ord != &nex_ord.signum() {
            // println!("{:?}", arr);
            return false;
        }
    } else {
        *cur_ord = Some(nex_ord.signum());
    }
    if !(1..=3).contains(&nex_ord.abs()) {
        // println!("{:?} as", arr);
        return false;
    }
    true
}

pub fn main(input: &str) -> (usize, usize) {
    let mut collector = Vec::new();

    let mut part1 = 0;
    let mut part2 = 0;

    for report in input
        .split('\n')
        // .map(|l| l.split(' ').map(|a| utils::parse_int(a) as i32))
    {
        collector.extend(report.iter_signed::<i32>());

        let mut cur_ord = None;

        let mut p1_s = 1;
        let mut p2_s = 1;
        let mut p2_1up = false; // has 1-up been used?

        let mut i = 0;

        while i < collector.len() - 1 {
            let mut t_ord = cur_ord;
            // println!("{:?}", &collector[i..=i + 1]);
            if check(&collector[i..=i + 1], &mut t_ord) {
                cur_ord = t_ord;
                i += 1;
                continue; // calm!
            }

            p1_s = 0; // no hope for part 1

            // but potential vindication for part 2
            if p2_1up {
                p2_s = 0;
                break;
            }
            p2_1up = true;

            // now for edge case galore!!
            if i > collector.len() - 3 || check(&[collector[i], collector[i + 2]], &mut cur_ord) {
                i += 2; // skip next one
            } else if i == 0 {
                // 1 - index = 1
                i = 1;
            } else if i == 1 {
                // step 1 - get actual order by looking at last 2 vals
                let mut t_ord = None;
                if !check(&collector[collector.len() - 2..], &mut t_ord) || t_ord == None {
                    // we're going to end up failing regardless, so quit now
                    p2_s = 0;
                    break;
                }

                // if order of second pair matches inherent order (would mean first does as well), skip third (issue with spacing)
                if t_ord == cur_ord {
                    if check(&[collector[1], collector[3]], &mut t_ord) {
                        i = 3;
                    }
                }
                // only other option is skipping second
                else {
                    cur_ord = t_ord;
                    if check(&[collector[1], collector[2]], &mut t_ord)
                        || check(&[collector[0], collector[2]], &mut t_ord)
                    {
                        i = 2;
                    }
                }
            }
        }

        part1 += p1_s;
        part2 += p2_s;

        collector.clear();
    }

    (part1, part2)
}
