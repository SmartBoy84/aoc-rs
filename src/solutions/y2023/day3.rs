use super::*;

pub fn main() -> Result<()> {
    let input = get_input(3, 2023)?;
    let line_length = input.split("\n").next().unwrap().len();
let input = input.replace("\n", "");

    println!("{}, length: {line_length}", input);

    Ok(())
}
