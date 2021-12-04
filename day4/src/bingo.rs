pub mod cell;
pub mod grid;

use std::cell::RefCell;
use std::rc::Rc;
pub use cell::Cell;
pub use grid::Grid;

pub fn find_winner(mut grids: RefCell<Vec<Grid>>, numbers: &Vec<u32>) -> Option<Grid> {
    for number in numbers.iter() {
        for grid in grids.borrow_mut().iter_mut() {
            grid.mark_cells(*number);
            if grid.has_won() {
                return Some(grid.clone())
            }
        }
    }
    None
}

pub fn call_number(grids: &mut Vec<Grid>, number: u32) -> Option<&Grid> {
    for grid in grids.iter_mut() {
        grid.mark_cells(number);
        if grid.has_won() {
            return Some(grid)
        }
    }
    None
}
