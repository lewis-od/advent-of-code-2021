use std::cell::RefCell;
use day4::parsing::{parse_grid, parse_numbers};
use std::io;
use std::rc::Rc;
use day4::bingo::{Cell, find_winner, Grid};
use utils::{FileReader, INPUT_FILE_NAME, ReadLines};

fn main() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let lines = reader.read_lines_as_strings()?;
    let mut line_iter = lines.iter();

    let called_numbers = line_iter.next().expect("First line missing");
    let called_numbers = parse_numbers(&called_numbers);

    // Skip first blank row
    line_iter.next();

    let mut grids = parse_grids(Box::new(line_iter));
    let winner = find_winner(RefCell::new(grids), &called_numbers);

    match winner {
        Some(grid) => println!("{}", grid.score()),
        None => println!("No winner"),
    }

    io::Result::Ok(())
}

fn parse_grids<'a, I>(mut line_iter: I) -> Vec<Grid>
where
    I: Iterator<Item = &'a String>,
{
    let mut grids: Vec<Grid> = vec![];
    loop {
        let grid: Vec<&String> = line_iter
            .by_ref()
            .take_while(|line| line.as_str() != "")
            .collect();
        if grid.len() == 0 {
            break;
        }
        let grid = parse_grid(&grid);
        grids.push(Grid::new(grid));
    }
    grids
}
