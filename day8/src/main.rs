use std::io;
use utils::{FileReader, INPUT_FILE_NAME, ReadLines};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let lines = reader.read_lines_as_strings()?;
    let lines = lines.iter()
        .map(|s| s.split(" | ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    part1(&lines);

    io::Result::Ok(())
}

fn part1(lines: &Vec<Vec<&str>>) {
    let actual_digits = lines.iter()
        .map(|line| line[1])
        .collect::<Vec<&str>>();
    let mut num_digits_with_unique_segments = 0;
    for digits in actual_digits {
        let displays = digits
            .split(" ")
            .map(|segments| segments.len() as u32)
            .filter(|num_segments| *num_segments == 2 || *num_segments == 4 || *num_segments == 3 || *num_segments == 7)
            .count();
        num_digits_with_unique_segments += displays
    }

    println!("Num digits with unique # segments: {}", num_digits_with_unique_segments);
}
