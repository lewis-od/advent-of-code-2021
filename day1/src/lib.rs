use std::io::Result;
use utils::read_lines_as_u32;

pub fn calc_result(filename: &str) -> Result<u32> {
    let depths = read_lines_as_u32(filename)?;
    Ok(calc_num_higher(depths))
}

pub fn calc_windowed_result(filename: &str) -> Result<u32> {
    let depths = read_lines_as_u32(filename)?;
    let windowed = calc_windows(depths);
    Ok(calc_num_higher(windowed))
}

fn calc_num_higher(depths: Vec<u32>) -> u32 {
    let mut depths_iter = depths.iter();
    let mut prev_depth: u32 = *depths_iter.next().unwrap();
    let mut num_higher: u32 = 0;
    for depth in depths_iter {
        if *depth > prev_depth {
            num_higher += 1;
        }
        prev_depth = *depth;
    }
    num_higher
}

fn calc_windows(depths: Vec<u32>) -> Vec<u32> {
    let mut windowed = vec![];
    for i in 0..(depths.len() - 2) {
        let sum = depths[i] + depths[i + 1] + depths[i + 2];
        windowed.push(sum);
    }
    windowed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_counts_num_higher_consecutive() {
        let inputs = vec![1, 2, 3, 4];
        let result = calc_num_higher(inputs);
        assert_eq!(result, 3);
    }

    #[test]
    fn passes_part_1_example() {
        let inputs: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let results = calc_num_higher(inputs);
        assert_eq!(results, 7);
    }

    #[test]
    fn calculates_windows_vector() {
        let inputs: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let windowed_inputs = vec![607, 618, 618, 617, 647, 716, 769, 792];

        let result = calc_windows(inputs);

        assert_eq!(windowed_inputs, result);
    }

    #[test]
    fn calculates_windowed_result() {
        let inputs: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = calc_num_higher(calc_windows(inputs));
        assert_eq!(result, 5);
    }
}
