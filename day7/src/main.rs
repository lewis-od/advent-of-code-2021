use std::io;
use day7::calculate_min_fuel_cost;
use utils::{FileReader, INPUT_FILE_NAME, ReadLines};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let positions = reader.read_line_as_u32s()?;

    let min_cost = calculate_min_fuel_cost(&positions);
    println!("Min cost: {}", min_cost);

    io::Result::Ok(())
}
