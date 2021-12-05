#[derive(PartialEq, Debug)]
pub struct Point(u32, u32);

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point(x, y)
    }

    pub fn x(&self) -> u32 {
        self.0
    }

    pub fn y(&self) -> u32 {
        self.1
    }
}
