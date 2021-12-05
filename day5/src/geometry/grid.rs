use crate::geometry::line_segment::LineSegment;

pub struct Grid {
    size: usize,
    values: Vec<Vec<u32>>,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid { size, values: create_zero_grid(size) }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn value(&self, x: u32, y: u32) -> u32 {
        self.values[x as usize][y as usize]
    }

    pub fn mark_line_segment(&mut self, line_segment: &LineSegment) {
        let (min_x, max_x) = line_segment.x_range();
        let (min_y, max_y) = line_segment.y_range();
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                self.values[x as usize][y as usize] += 1
            }
        }
    }
}

fn create_zero_grid(size: usize) -> Vec<Vec<u32>> {
    let mut rows = Vec::with_capacity(size);
    for _ in 0..size {
        rows.push(create_zeros(size));
    }
    rows
}

fn create_zeros(size:usize) -> Vec<u32> {
    let mut zeros = Vec::with_capacity(size);
    for _ in 0..size {
        zeros.push(0);
    }
    zeros
}

#[cfg(test)]
mod tests {
    use crate::geometry::Point;
    use super::*;

    #[test]
    fn initialises_with_square_grid_of_zeros() {
        let grid = Grid::new(10);

        assert_eq!(10, grid.size());
        for x in 0..10 {
            for y in 0..10 {
                assert_eq!(0, grid.value(x, y))
            }
        }
    }

    #[test]
    fn marks_horizontal_segment_on_grid() {
        let mut grid = Grid::new(10);

        let start = Point::new(1, 1);
        let end = Point::new(5, 1);
        let line_segment = LineSegment::new(start, end);

        grid.mark_line_segment(&line_segment);

        for x in 1..6 {
            assert_eq!(1, grid.value(x, 1));
        }
    }

    #[test]
    fn marks_decreasing_horizontal_segment_on_grid() {
        let mut grid = Grid::new(10);

        let start = Point::new(5, 1);
        let end = Point::new(1, 1);
        let line_segment = LineSegment::new(start, end);

        grid.mark_line_segment(&line_segment);

        for x in 1..6 {
            assert_eq!(1, grid.value(x, 1));
        }
    }

    #[test]
    fn marks_vertical_segment_on_grid() {
        let mut grid = Grid::new(10);

        let start = Point::new(5, 1);
        let end = Point::new(5, 5);
        let line_segment = LineSegment::new(start, end);

        grid.mark_line_segment(&line_segment);

        for y in 1..6 {
            assert_eq!(1, grid.value(5, y));
        }
    }

    #[test]
    fn marks_decreasing_vertical_segment_on_grid() {
        let mut grid = Grid::new(10);

        let start = Point::new(5, 5);
        let end = Point::new(5, 1);
        let line_segment = LineSegment::new(start, end);

        grid.mark_line_segment(&line_segment);

        for y in 1..6 {
            assert_eq!(1, grid.value(5, y));
        }
    }
}
