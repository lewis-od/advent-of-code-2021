use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};

pub static INPUT_FILE_NAME: &str = "input.txt";

pub trait ReadLines {
    fn read_lines_as_u32(&self) -> Result<Vec<u32>>;
    fn read_lines_as_strings(&self) -> Result<Vec<String>>;
    fn read_and_map_lines<T>(&self, mapper: fn(String) -> T) -> Result<Vec<T>>;
    fn read_line_as_u32s(&self) -> Result<Vec<u32>>;
}

pub struct FileReader<'a> {
    filename: &'a str,
}

impl<'a> FileReader<'a> {
    pub fn new(filename: &str) -> FileReader {
        FileReader { filename }
    }

    fn read_lines(&self) -> Result<Lines<BufReader<File>>> {
        let file = File::open(&self.filename)?;
        let buf = BufReader::new(file);
        Ok(buf.lines())
    }
}

impl<'a> ReadLines for FileReader<'a> {
    fn read_lines_as_u32(&self) -> Result<Vec<u32>> {
        self.read_and_map_lines(|value| value.parse::<u32>().unwrap())
    }

    fn read_lines_as_strings(&self) -> Result<Vec<String>> {
        self.read_and_map_lines(|s| s)
    }

    fn read_and_map_lines<T>(&self, mapper: fn(String) -> T) -> Result<Vec<T>> {
        Ok(self
            .read_lines()?
            .map(|l| mapper(l.expect("Expected line")))
            .collect())
    }

    fn read_line_as_u32s(&self) -> Result<Vec<u32>> {
        let lines = self.read_lines_as_strings()?;
        let row = lines.get(0).expect("Expected at least 1 row in file");
        Ok(row
            .split(',')
            .map(|s| s.parse::<u32>().expect("Unable to parse u32"))
            .collect())
    }
}
