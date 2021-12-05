pub mod geometry;
pub mod parsing;

use geometry::{Grid, LineSegment};
use parsing::parse_row_to_line_segment;

pub fn part1(rows: &Vec<String>, grid_size: usize) -> u32 {
    let mut grid = Grid::new(grid_size);
    let line_segments = rows
        .iter()
        .map(|row| parse_row_to_line_segment(row))
        .filter(|line_segment| line_segment.is_straight())
        .collect::<Vec<LineSegment>>();

    for line_segment in line_segments.iter() {
        grid.mark_line_segment(line_segment);
    }

    grid.num_dangerous_points()
}

pub fn part2(rows: &Vec<String>, grid_size: usize) -> u32 {
    let mut grid = Grid::new(grid_size);
    let line_segments = rows
        .iter()
        .map(|row| parse_row_to_line_segment(row))
        .collect::<Vec<LineSegment>>();

    for line_segment in line_segments.iter() {
        grid.mark_line_segment(line_segment);
    }

    grid.num_dangerous_points()
}
