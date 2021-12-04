use day4::bingo::{find_first_winner, find_last_winner_score, Cell, Grid};
use day4::parsing::{parse_grid, parse_numbers};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use utils::{FileReader, ReadLines, INPUT_FILE_NAME};

fn main() -> io::Result<()> {
    part1()?;
    part2()?;
    io::Result::Ok(())
}

fn part1() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let lines = reader.read_lines_as_strings()?;
    let mut line_iter = lines.iter();

    let called_numbers = line_iter.next().expect("First line missing");
    let called_numbers = parse_numbers(&called_numbers);

    // Skip first blank row
    line_iter.next();

    let mut grids = parse_grids(Box::new(line_iter));
    let score = find_first_winner(RefCell::new(grids), &called_numbers);
    println!("Score of first winner: {}", score);

    io::Result::Ok(())
}

fn part2() -> io::Result<()> {
    let reader = FileReader::new(INPUT_FILE_NAME);
    let lines = reader.read_lines_as_strings()?;
    let mut line_iter = lines.iter();

    let called_numbers = line_iter.next().expect("First line missing");
    let called_numbers = parse_numbers(&called_numbers);

    // Skip first blank row
    line_iter.next();

    let mut grids = parse_grids(Box::new(line_iter));
    let score = find_last_winner_score(RefCell::new(grids), &called_numbers);
    println!("Score of last winner: {}", score);

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
