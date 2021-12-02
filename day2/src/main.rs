use day2::Submarine;
use utils::read_lines_as_strings;
use std::io;

fn main() -> io::Result<()> {
    let lines = read_lines_as_strings("input.txt")?;
    let lines = lines.iter()
        .map(|s| &s[..])
        .collect();

    let mut submarine = Submarine::new();
    submarine.process_commands(lines);

    let result = submarine.position() * submarine.depth();
    println!("Part 1: {}", result);

    io::Result::Ok(())
}
