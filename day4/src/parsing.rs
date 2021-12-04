use crate::bingo::Cell;

pub fn parse_numbers(row: &str) -> Vec<u32> {
    parse_row_to_u32(row, ",")
}

pub fn parse_grid(input: &Vec<&String>) -> Vec<Vec<Cell>> {
    input
        .iter()
        .map(|row| parse_row_to_u32(row, " "))
        .map(|row| map_row_to_cells(row))
        .collect()
}

fn map_row_to_cells(row: Vec<u32>) -> Vec<Cell> {
    row.iter().map(|value| Cell::new(*value)).collect()
}

fn parse_row_to_u32(row: &str, separator: &str) -> Vec<u32> {
    row.split(separator)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().expect("Unable to parse u32"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_string_of_csv_numbers() {
        let input = "7,4,9,5,11,17,23,2,0";

        let output = parse_numbers(input);
        assert_eq!(vec![7, 4, 9, 5, 11, 17, 23, 2, 0], output);
    }

    #[test]
    fn parses_grid_of_numbers() {
        let row1 = "22 13 17 11  0".to_string();
        let row2 = " 8  2 23  4 24".to_string();
        let row3 = "21  9 14 16  7".to_string();
        let row4 = " 6 10  3 18  5".to_string();
        let row5 = " 1 12 20 15 19".to_string();
        let input = vec![&row1, &row2, &row3, &row4, &row5];

        let output = parse_grid(&input);

        let expected = vec![
            vec![
                Cell::new(22),
                Cell::new(13),
                Cell::new(17),
                Cell::new(11),
                Cell::new(0),
            ],
            vec![
                Cell::new(8),
                Cell::new(2),
                Cell::new(23),
                Cell::new(4),
                Cell::new(24),
            ],
            vec![
                Cell::new(21),
                Cell::new(9),
                Cell::new(14),
                Cell::new(16),
                Cell::new(7),
            ],
            vec![
                Cell::new(6),
                Cell::new(10),
                Cell::new(3),
                Cell::new(18),
                Cell::new(5),
            ],
            vec![
                Cell::new(1),
                Cell::new(12),
                Cell::new(20),
                Cell::new(15),
                Cell::new(19),
            ],
        ];
        assert_eq!(expected, output);
    }
}
