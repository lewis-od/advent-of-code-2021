pub mod cell;
pub mod grid;

pub use cell::Cell;
pub use grid::Grid;
use std::cell::RefCell;

pub fn find_first_winner_score(grids: RefCell<Vec<Grid>>, numbers: &Vec<u32>) -> u32 {
    for number in numbers.iter() {
        for grid in grids.borrow_mut().iter_mut() {
            grid.mark_cells(*number);
            if grid.has_won() {
                return grid.score();
            }
        }
    }
    0
}

pub fn find_last_winner_score(grids: RefCell<Vec<Grid>>, numbers: &Vec<u32>) -> u32 {
    let mut last_grid = Grid::new(vec![vec![]]);
    for number in numbers.iter() {
        let mut num_won = 0;
        for grid in grids.borrow_mut().iter_mut() {
            grid.mark_cells(*number);
            if grid.has_won() {
                num_won += 1;
            }
        }
        if num_won == grids.borrow().len() - 1 {
            for grid in grids.borrow_mut().iter_mut() {
                if !grid.has_won() {
                    last_grid = grid.clone();
                    break;
                }
            }
            break;
        }
    }

    for number in numbers.iter() {
        last_grid.mark_cells(*number);
        if last_grid.has_won() {
            return last_grid.score();
        }
    }
    0
}
