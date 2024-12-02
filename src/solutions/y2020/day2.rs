pub fn main(input: &str) -> (usize, usize) {
    let input = input.split("\n").map(|a| {
        let pass = a.as_bytes();
        let pass_it = 0..pass.len();
        let start = pass_it.take_while(|&i| pass[i] != b'-').collect::<&str>();
    }) 
    todo!()
}