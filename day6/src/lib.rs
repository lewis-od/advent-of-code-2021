pub fn simulate_growth(initial_state: &Vec<u32>, num_days: u32) -> u64 {
    let mut timer_counts = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for age in initial_state {
        timer_counts[*age as usize] += 1
    }
    for _ in 0..num_days {
        timer_counts.rotate_left(1);
        timer_counts[6] += timer_counts[8];
    }
    timer_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example_after_2_days() {
        let initial_state = vec![3, 4, 3, 1, 2];
        let num_days = 2;

        let num_fish = simulate_growth(&initial_state, num_days);

        assert_eq!(num_fish, 6);
    }

    #[test]
    fn matches_example_after_18_days() {
        let initial_state = vec![3, 4, 3, 1, 2];
        let num_days = 18;

        let num_fish = simulate_growth(&initial_state, num_days);

        assert_eq!(num_fish, 26);
    }

    #[test]
    fn matches_example_after_80_days() {
        let initial_state = vec![3, 4, 3, 1, 2];
        let num_days = 80;

        let num_fish = simulate_growth(&initial_state, num_days);

        assert_eq!(num_fish, 5934);
    }
}
