use day2::{part1, part2};
use std::io;
use utils::{FileReader, ReadLines, INPUT_FILE_NAME};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let lines = reader.read_lines_as_strings()?;

    part1(&lines);
    part2(&lines);

    io::Result::Ok(())
}

fn part1(commands: &Vec<String>) {
    let mut sub = part1::Submarine::new();
    sub.process_commands(commands);
    let result = sub.position() * sub.depth();
    println!("Part 1: {}", result);
}

fn part2(commands: &Vec<String>) {
    let mut sub = part2::Submarine::new();
    sub.process_commands(commands);
    let result = sub.position() * sub.depth();
    println!("Part 2: {}", result);
}
