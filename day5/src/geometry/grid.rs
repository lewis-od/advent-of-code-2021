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
        if line_segment.is_straight() {
            self.mark_straight_line(line_segment)
        } else {
            self.mark_diagonal_line(line_segment)
        }
    }

    fn mark_straight_line(&mut self, line_segment: &LineSegment) {
        let (min_x, max_x) = line_segment.x_range();
        let (min_y, max_y) = line_segment.y_range();
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                self.values[x as usize][y as usize] += 1
            }
        }
    }

    fn mark_diagonal_line(&mut self, line_segment: &LineSegment) {
        let dx: i32 = if line_segment.start().x() < line_segment.end().x() {
            1
        } else {
            -1
        };
        let dy: i32 = if line_segment.start().y() < line_segment.end().y() {
            1
        } else {
            -1
        };
        let mut x = line_segment.start().x();
        let mut y = line_segment.start().y();
        while x != (line_segment.end().x() as i32 + dx) as u32 {
            self.values[x as usize][y as usize] += 1;
            x = (x as i32 + dx) as u32;
            y = (y as i32 + dy) as u32;
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

    #[test]
    fn marks_positive_increasing_diagonal_on_grid() {
        let start = Point::new(0, 5);
        let end = Point::new(5, 0);
        let segment = LineSegment::new(start, end);
        let mut grid = Grid::new(10);

        grid.mark_line_segment(&segment);

        assert_eq!(1, grid.value(0, 5));
        assert_eq!(1, grid.value(1, 4));
        assert_eq!(1, grid.value(2, 3));
        assert_eq!(1, grid.value(3, 2));
        assert_eq!(1, grid.value(4, 1));
        assert_eq!(1, grid.value(5, 0));
    }

    #[test]
    fn marks_negative_increasing_diagonal_on_grid() {
        let start = Point::new(0, 0);
        let end = Point::new(5, 5);
        let segment = LineSegment::new(start, end);
        let mut grid = Grid::new(10);

        grid.mark_line_segment(&segment);

        assert_eq!(1, grid.value(0, 0));
        assert_eq!(1, grid.value(1, 1));
        assert_eq!(1, grid.value(2, 2));
        assert_eq!(1, grid.value(3, 3));
        assert_eq!(1, grid.value(4, 4));
        assert_eq!(1, grid.value(5, 5));
    }

    #[test]
    fn marks_positive_decreasing_diagonal_on_grid() {
        let start = Point::new(5, 0);
        let end = Point::new(0, 5);
        let segment = LineSegment::new(start, end);
        let mut grid = Grid::new(10);

        grid.mark_line_segment(&segment);

        assert_eq!(1, grid.value(0, 5));
        assert_eq!(1, grid.value(1, 4));
        assert_eq!(1, grid.value(2, 3));
        assert_eq!(1, grid.value(3, 2));
        assert_eq!(1, grid.value(4, 1));
        assert_eq!(1, grid.value(5, 0));
    }

    #[test]
    fn marks_negative_decreasing_diagonal_on_grid() {
        let start = Point::new(5, 5);
        let end = Point::new(0, 0);
        let segment = LineSegment::new(start, end);
        let mut grid = Grid::new(10);

        grid.mark_line_segment(&segment);

        assert_eq!(1, grid.value(0, 0));
        assert_eq!(1, grid.value(1, 1));
        assert_eq!(1, grid.value(2, 2));
        assert_eq!(1, grid.value(3, 3));
        assert_eq!(1, grid.value(4, 4));
        assert_eq!(1, grid.value(5, 5));
    }
}
