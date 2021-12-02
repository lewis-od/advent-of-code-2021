use day1::{calc_windowed_result, calc_num_higher};
use utils::{FileReader, INPUT_FILE_NAME, ReadLines};
use std::io::Result;

fn main() -> Result<()> {
    let file_reader = FileReader::new(INPUT_FILE_NAME);
    let depths = file_reader.read_lines_as_u32()?;

    let result = calc_num_higher(&depths);
    println!("Part 1: {}", result);

    let windowed_result = calc_windowed_result(&depths);
    println!("Part 2: {}", windowed_result);

    Ok(())
}
