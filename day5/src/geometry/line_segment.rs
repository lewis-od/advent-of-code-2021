use super::Point;

#[derive(PartialEq, Debug)]
pub struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> LineSegment {
        LineSegment { start, end }
    }

    pub fn start(&self) -> &Point {
        &self.start
    }

    pub fn end(&self) -> &Point {
        &self.end
    }

    pub fn x_range(&self) -> (u32, u32) {
        if self.start.x() > self.end.x() {
            (self.end.x(), self.start.x())
        } else {
            (self.start.x(), self.end().x())
        }
    }

    pub fn y_range(&self) -> (u32, u32) {
        if self.start.y() > self.end.y() {
            (self.end.y(), self.start.y())
        } else {
            (self.start.y(), self.end().y())
        }
    }

    pub fn is_straight(&self) -> bool {
        self.start.x() == self.end.x() || self.start.y() == self.end.y()
    }
}
