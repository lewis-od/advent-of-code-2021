pub fn part1(positions: &Vec<u32>) -> u32 {
    calculate_min_fuel_cost(positions, calculate_constant_cost)
}

pub fn part2(positions: &Vec<u32>) -> u32 {
    calculate_min_fuel_cost(positions, calculate_increasing_cost)
}

fn calculate_min_fuel_cost(positions: &Vec<u32>, cost_fn: fn(u32, u32) -> u32) -> u32 {
    let start = *positions.iter().min().unwrap();
    let end = *positions.iter().max().unwrap();
    let mut min_cost = calculate_cumulative_cost(positions, start, cost_fn);
    for position in (start + 1)..=end {
        let cost = calculate_cumulative_cost(positions, position, cost_fn);
        if cost < min_cost {
            min_cost = cost;
        }
    }
    min_cost
}

fn calculate_cumulative_cost(
    positions: &Vec<u32>,
    target_position: u32,
    cost_fn: fn(u32, u32) -> u32,
) -> u32 {
    positions
        .iter()
        .map(|position| cost_fn(*position, target_position))
        .sum()
}

fn calculate_constant_cost(from: u32, to: u32) -> u32 {
    (from as i64 - to as i64).abs() as u32
}

fn calculate_increasing_cost(from: u32, to: u32) -> u32 {
    let num_steps = calculate_constant_cost(from, to);
    (num_steps * (num_steps + 1)) / 2 // sum_{k=0}^{n} k = n(n+1)/2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcs_min_constant_cost_for_example() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let min_cost = part1(&positions);

        assert_eq!(37, min_cost);
    }

    #[test]
    fn calcs_min_increasing_cost_for_example() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let min_cost = part2(&positions);

        assert_eq!(168, min_cost);
    }
}
