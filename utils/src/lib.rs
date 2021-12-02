use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;

pub static INPUT_FILE_NAME: &str = "input.txt";

pub trait ReadLines {
    fn read_lines_as_u32(&self) -> Result<Vec<u32>>;
    fn read_lines_as_strings(&self) -> Result<Vec<String>>;
}

pub struct FileReader<'a> {
    filename: &'a str,
}

impl<'a> FileReader<'a> {
    pub fn new(filename: &str) -> FileReader {
        FileReader { filename }
    }

    fn read_and_map_lines<T>(&self, mapper: fn(String) -> T) -> Result<Vec<T>> {
        Ok(self
            .read_lines()?
            .map(|l| mapper(l.expect("Expected line")))
            .collect())
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
}
