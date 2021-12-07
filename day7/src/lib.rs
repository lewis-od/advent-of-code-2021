pub fn calculate_min_fuel_cost(positions: &Vec<u32>) -> u32 {
    let start = *positions.iter().min().unwrap();
    let end = *positions.iter().max().unwrap();
    let mut min_cost = calculate_cumulative_cost(positions, start);
    for position in (start + 1)..=end {
        let cost = calculate_cumulative_cost(positions, position);
        if cost < min_cost {
            min_cost = cost;
        }
    }
    min_cost
}

fn calculate_cumulative_cost(positions: &Vec<u32>, target_position: u32) -> u32 {
    positions
        .iter()
        .map(|position| calculate_individual_cost(*position, target_position))
        .sum()
}

fn calculate_individual_cost(from: u32, to: u32) -> u32 {
    (from as i64 - to as i64).abs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcs_min_cost_for_example() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let min_cost = calculate_min_fuel_cost(&positions);

        assert_eq!(37, min_cost);
    }
}
