use super::Cell;

#[derive(Debug)]
pub struct Grid {
    rows: Vec<Vec<Cell>>,
    has_won: bool,
    score: u32,
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            rows: self.rows.clone(),
            has_won: self.has_won,
            score: self.score,
        }
    }
}

impl Grid {
    pub fn new(cells: Vec<Vec<Cell>>) -> Grid {
        Grid {
            rows: cells,
            has_won: false,
            score: 0,
        }
    }

    pub fn cell(&self, row: usize, col: usize) -> &Cell {
        &self.rows[row][col]
    }

    pub fn has_won(&self) -> bool {
        self.has_won
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn mark_cells(&mut self, called_number: u32) {
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut() {
                cell.mark(called_number);
            }
        }

        if !self.has_won {
            self.check_if_won(called_number)
        }
    }

    fn check_if_won(&mut self, called_number: u32) {
        if self.has_winning_column() {
            self.has_won = true;
        } else if self.has_winning_row() {
            self.has_won = true;
        }

        if self.has_won {
            self.calculate_score(called_number);
        }
    }

    fn has_winning_column(&self) -> bool {
        for column in 0..self.rows[0].len() {
            let first_cell = self.cell(0, column);
            if first_cell.is_marked() {
                let mut column_all_marked = true;
                for row in 1..self.rows.len() {
                    let col_cell = self.cell(row, column);
                    if !col_cell.is_marked() {
                        column_all_marked = false;
                        break;
                    }
                }
                if column_all_marked {
                    return true;
                }
            }
        }
        false
    }

    fn has_winning_row(&self) -> bool {
        for row in 0..self.rows.len() {
            let first_cell = self.cell(row, 0);
            if first_cell.is_marked() {
                let mut row_all_marked = true;
                for row_cell in self.rows[row].iter() {
                    if !row_cell.is_marked() {
                        row_all_marked = false;
                        break;
                    }
                }
                if row_all_marked {
                    return true;
                }
            }
        }
        false
    }

    fn calculate_score(&mut self, winning_number: u32) {
        let num_rows = self.rows.len();
        let num_cols = self.rows[0].len();
        let mut score = 0;
        for row in 0..num_rows {
            for col in 0..num_cols {
                let cell = self.cell(row, col);
                if !cell.is_marked() {
                    score += cell.value()
                }
            }
        }
        self.score = score * winning_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_grid() -> Grid {
        let values = vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19],
        ];
        grid_from_values(values)
    }

    fn grid_from_values(values: Vec<Vec<u32>>) -> Grid {
        let cells = values
            .iter()
            .map(|row| row.iter().map(|value| Cell::new(*value)).collect())
            .collect();
        Grid::new(cells)
    }

    #[test]
    fn marks_all_cells() {
        let cells = vec![vec![Cell::new(1), Cell::new(2)]];
        let mut grid = Grid::new(cells);

        grid.mark_cells(2);

        assert_eq!(true, grid.cell(0, 1).is_marked());
    }

    #[test]
    fn detects_winning_column() {
        let mut grid = test_grid();

        grid.mark_cells(17);
        grid.mark_cells(23);
        grid.mark_cells(14);
        grid.mark_cells(3);
        grid.mark_cells(20);

        let has_won = grid.has_winning_column();
        assert_eq!(true, has_won);
    }

    #[test]
    fn doesnt_detect_losing_column() {
        let mut grid = test_grid();

        grid.mark_cells(17);
        grid.mark_cells(23);
        grid.mark_cells(9);
        grid.mark_cells(22);
        grid.mark_cells(0);

        let has_won = grid.has_winning_column();
        assert_eq!(false, has_won);
    }

    #[test]
    fn detects_winning_row() {
        let mut grid = test_grid();

        grid.mark_cells(21);
        grid.mark_cells(9);
        grid.mark_cells(14);
        grid.mark_cells(16);
        grid.mark_cells(7);

        let has_won = grid.has_winning_row();
        assert_eq!(true, has_won);
    }

    #[test]
    fn doesnt_detect_losing_row() {
        let mut grid = test_grid();

        grid.mark_cells(21);
        grid.mark_cells(9);
        grid.mark_cells(14);
        grid.mark_cells(1);
        grid.mark_cells(12);

        let has_won = grid.has_winning_row();
        assert_eq!(false, has_won);
    }

    #[test]
    fn calculates_score() {
        let mut grid = test_grid();

        // vec![],
        // vec![2, 23,  4, 24],
        // vec![14, 16,  7],
        // vec![ 6, 10,  3, 18,  5],
        // vec![ 1, 12, 20, 15, 19],

        grid.mark_cells(22);
        grid.mark_cells(8);
        grid.mark_cells(21);
        grid.mark_cells(9);
        grid.mark_cells(13);
        grid.mark_cells(17);
        grid.mark_cells(0);
        grid.mark_cells(11);

        let score = grid.score();
        assert_eq!(2189, score);
    }
}
