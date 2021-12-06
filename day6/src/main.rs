use std::io;
use day6::simulate_growth;
use utils::{FileReader, INPUT_FILE_NAME, ReadLines};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let input = reader.read_lines_as_strings()?;
    let input = input[0].split(',')
        .map(|n| n.parse::<u32>().expect("Unable to parse u32"))
        .collect();

    let num_fish = simulate_growth(&input, 80);
    println!("Num fish after 80 days: {}", num_fish);

    let num_fish = simulate_growth(&input, 256);
    println!("Num fish after 80 days: {}", num_fish);

    io::Result::Ok(())
}
