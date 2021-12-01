use day1::calc_result;
use day1::calc_windowed_result;
use std::io::Result;

static INPUT_FILE: &str = "input.txt";

fn main() -> Result<()> {
    let result = calc_result(INPUT_FILE)?;
    println!("Part 1: {}", result);

    let windowed_result = calc_windowed_result(INPUT_FILE)?;
    println!("Part 2: {}", windowed_result);
    Ok(())
}
