use std::io;

use day3::{calc_life_support_rating, calc_power_consumption};
use utils::{FileReader, ReadLines, INPUT_FILE_NAME};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let inputs = reader.read_and_map_lines(|s| isize::from_str_radix(&s, 2).unwrap() as u32)?;

    let power_consumption = calc_power_consumption(&inputs, 12);
    println!("Power consumption: {}", power_consumption);

    let life_support_rating = calc_life_support_rating(&inputs, 12);
    println!("Life support rating: {}", life_support_rating);

    io::Result::Ok(())
}
