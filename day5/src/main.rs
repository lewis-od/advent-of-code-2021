use std::io;
use day5::part1;
use utils::{FileReader, INPUT_FILE_NAME, ReadLines};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let rows = reader.read_lines_as_strings()?;
    let num_points = part1(&rows, 999);
    println!("Part 1: {}", num_points);
    io::Result::Ok(())
}
