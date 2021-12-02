use day2::{part1, part2, Submarine};
use std::io;
use utils::{FileReader, ReadLines, INPUT_FILE_NAME};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let lines = reader.read_lines_as_strings()?;
    let lines = lines.iter().map(String::as_str).collect();

    part1(&lines);
    part2(&lines);

    io::Result::Ok(())
}

fn part1(commands: &Vec<&str>) {
    let commands = part1::parse_commands(commands);
    let mut sub = Submarine::new();
    sub.process_commands(&commands);
    let result = sub.position() * sub.depth();
    println!("Part 1: {}", result);
}

fn part2(commands: &Vec<&str>) {
    let commands = part2::parse_commands(commands);
    let mut sub = Submarine::new();
    sub.process_commands(&commands);
    let result = sub.position() * sub.depth();
    println!("Part 2: {}", result);
}
