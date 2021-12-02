use crate::Command;
use crate::Direction;
use crate::parse_command;

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

    pub fn process_commands(&mut self, commands: &Vec<&str>) {
        for command in commands {
            let command = parse_command(command);
            self.process_command(&command);
        }
    }

    fn process_command(&mut self, command: &Command) {
        if command.direction == Direction::Forward {
            self.position += command.amount;
            self.depth += command.amount * self.aim;
        } else if command.direction == Direction::Down {
            self.aim += command.amount;
        } else if command.direction == Direction::Up {
            self.aim -= command.amount;
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

    #[test]
    fn matches_example() {
        let inputs = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        let mut sub = Submarine::new();
        sub.process_commands(&inputs);

        assert_eq!(15, sub.position());
        assert_eq!(60, sub.depth());
    }
}
