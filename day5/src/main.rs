use day5::{part1, part2};
use std::io;
use utils::{FileReader, ReadLines, INPUT_FILE_NAME};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let rows = reader.read_lines_as_strings()?;

    let num_points = part1(&rows, 999);
    println!("Part 1: {}", num_points);

    let num_points = part2(&rows, 999);
    println!("Part 2: {}", num_points);

    io::Result::Ok(())
}
