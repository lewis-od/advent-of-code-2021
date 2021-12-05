use crate::geometry::{LineSegment, Point};

pub fn parse_row_to_line_segment(row: &str) -> LineSegment {
    let points = row.split(" -> ").collect::<Vec<&str>>();
    let start = parse_coordinates_to_point(points[0]);
    let end = parse_coordinates_to_point(points[1]);

    LineSegment::new(start, end)
}

fn parse_coordinates_to_point(coordinates: &str) -> Point {
    let parts = coordinates.split(",").collect::<Vec<&str>>();
    let x = parts[0]
        .parse::<u32>()
        .expect("Couldn't parse x to integer");
    let y = parts[1]
        .parse::<u32>()
        .expect("Couldn't parse y to integer");
    Point::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_row_to_line_segment() {
        let row = "955,125 -> 151,929";

        let parsed_segment = parse_row_to_line_segment(row);

        let start = Point::new(955, 125);
        let end = Point::new(151, 929);
        let expected_segment = LineSegment::new(start, end);

        assert_eq!(expected_segment, parsed_segment);
    }
}
