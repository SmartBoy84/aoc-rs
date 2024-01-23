// the solution in use was *heavily inspired (aka copied) from the one on ManEatingApe's repo
fn find_index_packet(input: &str, marker: usize) -> usize {
    let mut window_start = 0;
    let mut seen = [0; 26];

    for (i, c) in input.as_bytes().iter().enumerate() {
        let index = (c - b'a') as usize;
        let first_duplicated = seen[index];
        seen[index] = i;

        // previous is out because we've encountered a duplicate, if previous falls within the window range (in front of the current start value) then reset start
        if first_duplicated > window_start {
            window_start = first_duplicated;
        }
        if i - window_start == marker {
            return i + 1;
        }
    }

    unreachable!()

    // let mut seen = [0; 26];
    // let mut dup = 0;
    // let mut chunk = std::collections::VecDeque::with_capacity(marker);
    // for (i, char) in input.as_bytes().iter().map(|&a| (a - b'a') as usize).enumerate() {
    //     if chunk.len() == chunk.capacity() {
    //         let last = chunk.pop_front().unwrap();
    //         seen[last] -= 1;
    //         if seen[last] == 1 {
    //             dup -= 1;
    //         }
    //     }

    //     chunk.push_back(char);
    //     seen[char] += 1;
    //     if seen[char] == 2 {
    //         dup += 1;
    //     }

    //     if dup == 0 && chunk.len() == marker {
    //         return i + 1;
    //     }
    // }
    // unreachable!()

    // input
    //     .as_bytes()
    //     .iter()
    //     .enumerate()
    //     .find_map(|(i, _)| {
    //         input
    //             .get(i - len..i)
    //             .is_some_and(|packet| {
    //                 packet.as_bytes().iter().enumerate().all(|(cur, a)| {
    //                     for (test, letter) in packet.as_bytes().iter().enumerate() {
    //                         if test == cur {
    //                             continue;
    //                         }
    //                         if letter == a {
    //                             return false;
    //                         }
    //                     }
    //                     true
    //                 })
    //             })
    //             .then_some(i)
    //     })
    //     .unwrap()
}

pub fn main(input: &str) -> (usize, usize) {
    (find_index_packet(input, 4), find_index_packet(input, 14))
}
