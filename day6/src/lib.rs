use lantern_fish::LanternFish;

pub mod lantern_fish;

pub fn part1(initial_state: &Vec<u32>, num_days: u32) -> u64 {
    let mut fish = initial_state.iter()
        .map(|state| LanternFish::new(*state))
        .collect::<Vec<LanternFish>>();
    for _ in 0..num_days {
        let mut babies = fish.iter_mut()
            .map(|fish| fish.tick())
            .filter(|possible_baby| possible_baby.is_some())
            .map(|baby| baby.unwrap())
            .collect::<Vec<LanternFish>>();
        fish.append(&mut babies);
    }
    fish.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_example_after_2_days() {
        let initial_state = vec![3, 4, 3, 1, 2];
        let num_days = 2;

        let num_fish = part1(&initial_state, num_days);

        assert_eq!(num_fish, 6);
    }

    #[test]
    fn matches_example_after_18_days() {
        let initial_state = vec![3, 4, 3, 1, 2];
        let num_days = 18;

        let num_fish = part1(&initial_state, num_days);

        assert_eq!(num_fish, 26);
    }

    #[test]
    fn matches_example_after_80_days() {
        let initial_state = vec![3, 4, 3, 1, 2];
        let num_days = 80;

        let num_fish = part1(&initial_state, num_days);

        assert_eq!(num_fish, 5934);
    }
}
