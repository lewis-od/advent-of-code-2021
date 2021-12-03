use std::io;

use day3::{find_most_common, calc_power_consumption};
use utils::{FileReader, INPUT_FILE_NAME, ReadLines};


fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let inputs = reader.read_and_map_lines(|s| isize::from_str_radix(&s, 2).unwrap() as u32)?;

    let power_consumption = calc_power_consumption(&inputs, 12);
    println!("Power consumption: {}", power_consumption);

    io::Result::Ok(())
}
