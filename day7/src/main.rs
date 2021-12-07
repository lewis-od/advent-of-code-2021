use std::io;

use day7::{part1, part2};
use utils::{FileReader, ReadLines, INPUT_FILE_NAME};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let positions = reader.read_line_as_u32s()?;

    let min_cost = part1(&positions);
    println!("Min cost (part 1): {}", min_cost);

    let min_cost = part2(&positions);
    println!("Min cost (part 2): {}", min_cost);

    io::Result::Ok(())
}
