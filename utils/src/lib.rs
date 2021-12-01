use std::io::Lines;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

pub fn read_lines_as_u32(filename: &str) -> Result<Vec<u32>> {
    read_and_map_lines(filename, |value| value.parse::<u32>().unwrap())
}

fn read_and_map_lines<T>(filename: &str, mapper: fn(String) -> T) -> Result<Vec<T>> {
    Ok(read_lines(filename)?
        .map(|l| mapper(l.expect("Expected line")))
        .collect())
}

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    Ok(buf.lines())
}
