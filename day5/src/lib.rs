pub mod geometry;
pub mod parsing;

use geometry::Grid;
use crate::geometry::LineSegment;
use crate::parsing::parse_row_to_line_segment;

pub fn part1(rows: &Vec<String>, grid_size: usize) -> u32 {
    let mut grid = Grid::new(grid_size);
    let line_segments = rows.iter()
        .map(|row| parse_row_to_line_segment(row))
        .filter(|line_segment| line_segment.is_straight())
        .collect::<Vec<LineSegment>>();

    for line_segment in line_segments.iter() {
        grid.mark_line_segment(line_segment);
    }

    let mut num_intersections = 0;
    for x in 0..grid.size() {
        for y in 0..grid.size() {
            if grid.value(x as u32, y as u32) >= 2 {
                num_intersections += 1;
            }
        }
    }

    num_intersections
}

pub fn part2(rows: &Vec<String>, grid_size: usize) -> u32 {
    let mut grid = Grid::new(grid_size);
    let line_segments = rows.iter()
        .map(|row| parse_row_to_line_segment(row))
        .collect::<Vec<LineSegment>>();

    for line_segment in line_segments.iter() {
        grid.mark_line_segment(line_segment);
    }

    let mut num_intersections = 0;
    for x in 0..grid.size() {
        for y in 0..grid.size() {
            if grid.value(x as u32, y as u32) >= 2 {
                num_intersections += 1;
            }
        }
    }

    num_intersections
}
