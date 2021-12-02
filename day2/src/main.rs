use day2::{part1, part2};
use std::io;
use utils::read_lines_as_strings;

fn main() -> io::Result<()> {
    let lines = read_lines_as_strings("input.txt")?;
    let lines = lines.iter().map(|s| &s[..]).collect();

    part1(&lines);
    part2(&lines);

    io::Result::Ok(())
}

fn part1(commands: &Vec<&str>) {
    let mut sub = part1::Submarine::new();
    sub.process_commands(commands);
    let result = sub.position() * sub.depth();
    println!("Part 1: {}", result);
}

fn part2(commands: &Vec<&str>) {
    let mut sub = part2::Submarine::new();
    sub.process_commands(commands);
    let result = sub.position() * sub.depth();
    println!("Part 2: {}", result);
}
