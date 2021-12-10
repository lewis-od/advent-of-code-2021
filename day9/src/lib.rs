pub struct TerrainScanner<'a> {
    terrain: &'a Vec<Vec<u32>>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl<'a> TerrainScanner<'a> {
    pub fn new(terrain: &'a Vec<Vec<u32>>) -> TerrainScanner {
        let (min_y, max_y) = (0, terrain.len() - 1);
        let (min_x, max_x) = (0, terrain[0].len() - 1);
        TerrainScanner {
            terrain,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn find_risk_level(&self) -> u32 {
        let mut risk_level: u32 = 0;
        for (y, row) in self.terrain.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if self.is_low_point(x, y) {
                    risk_level += value + 1;
                }
            }
        }
        risk_level
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        self.less_than_left(x, y) && self.less_than_above(x, y) && self.less_than_right(x, y) && self.less_than_below(x, y)
    }

    fn less_than_above(&self, x: usize, y: usize) -> bool {
        if y == self.min_y {
            return true;
        }
        self.terrain[y][x] < self.terrain[y - 1][x]
    }

    fn less_than_left(&self, x: usize, y: usize) -> bool {
        if x == self.min_x {
            return true;
        }
        self.terrain[y][x] < self.terrain[y][x - 1]
    }

    fn less_than_below(&self, x: usize, y: usize) -> bool {
        if y == self.max_y {
            return true;
        }
        self.terrain[y][x] < self.terrain[y + 1][x]
    }

    fn less_than_right(&self, x: usize, y: usize) -> bool {
        if x == self.max_x {
            return true;
        }
        self.terrain[y][x] < self.terrain[y][x + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example() {
        let inputs = vec![
            vec![2,1,9,9,9,4,3,2,1,0],
            vec![3,9,8,7,8,9,4,9,2,1],
            vec![9,8,5,6,7,8,9,8,9,2],
            vec![8,7,6,7,8,9,6,7,8,9],
            vec![9,8,9,9,9,6,5,6,7,8],
        ];
        let scanner = TerrainScanner::new(&inputs);

        let risk_level = scanner.find_risk_level();

        assert_eq!(15, risk_level);
    }
}
