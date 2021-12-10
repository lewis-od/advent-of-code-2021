use std::io;
use day9::TerrainScanner;
use utils::{FileReader, INPUT_FILE_NAME, ReadLines};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let rows = reader.read_lines_as_strings()?;
    let terrain = rows.iter()
        .map(|row| row.chars().map(|char| char.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    part1(&terrain);

    io::Result::Ok(())
}

fn part1(terrain: &Vec<Vec<u32>>) {
    let scanner = TerrainScanner::new(terrain);
    let risk_level = scanner.find_risk_level();
    println!("Risk level is: {}", risk_level);
}
