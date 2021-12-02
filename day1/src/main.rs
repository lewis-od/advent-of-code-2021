use day1::{calc_num_higher, calc_windowed_result};
use std::io::Result;
use utils::{FileReader, ReadLines, INPUT_FILE_NAME};

fn main() -> Result<()> {
    let file_reader = FileReader::new(INPUT_FILE_NAME);
    let depths = file_reader.read_lines_as_u32()?;

    let result = calc_num_higher(&depths);
    println!("Part 1: {}", result);

    let windowed_result = calc_windowed_result(&depths);
    println!("Part 2: {}", windowed_result);

    Ok(())
}
