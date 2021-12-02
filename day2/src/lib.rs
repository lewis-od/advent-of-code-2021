pub mod part1;
pub mod part2;

pub trait Execute {
    fn execute(&self, submarine: &mut Submarine);
}

pub struct Submarine {
    position: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn process_commands(&mut self, commands: &Vec<Box<dyn Execute>>) {
        for command in commands {
            command.execute(self)
        }
    }

    pub fn position(&self) -> i32 {
        self.position
    }

    pub fn depth(&self) -> i32 {
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StubCommand {}

    impl Clone for StubCommand {
        fn clone(&self) -> Self {
            StubCommand {}
        }
    }

    impl Execute for StubCommand {
        fn execute(&self, submarine: &mut Submarine) {
            submarine.position += 1;
            submarine.depth += 2;
            submarine.aim += 3;
        }
    }

    #[test]
    fn executes_all_commands() {
        let commands: Vec<Box<dyn Execute>> =
            vec![Box::new(StubCommand {}), Box::new(StubCommand {})];

        let mut submarine = Submarine::new();
        submarine.process_commands(&commands);

        assert_eq!(2, submarine.position);
        assert_eq!(4, submarine.depth);
        assert_eq!(6, submarine.aim);
    }
}
