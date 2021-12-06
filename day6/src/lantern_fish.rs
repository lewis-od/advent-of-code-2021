#[derive(Debug, PartialEq)]
pub struct LanternFish {
    days_until_spawn: u32,
}

impl LanternFish {
    pub fn new(days_until_spawn: u32) -> LanternFish {
        LanternFish { days_until_spawn }
    }

    pub fn tick(&mut self) -> Option<LanternFish> {
        if self.days_until_spawn == 0 {
            self.days_until_spawn = 6;
            return Some(LanternFish { days_until_spawn: 8 })
        }
        self.days_until_spawn -= 1;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawns_fish_on_day_0() {
        let mut mother = LanternFish::new(0);

        let baby = mother.tick();

        let baby = baby.expect("Expected baby to be spawned");
        assert_eq!(LanternFish { days_until_spawn: 8 }, baby);
        assert_eq!(mother.days_until_spawn, 6);
    }

    #[test]
    fn decrements_timer_if_not_day_0() {
        let mut mother = LanternFish::new(4);

        let baby = mother.tick();

        assert_eq!(None, baby);
        assert_eq!(mother.days_until_spawn, 3);
    }
}
