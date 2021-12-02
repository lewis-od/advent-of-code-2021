use crate::Command;
use crate::Direction;
use crate::parse_command;

pub struct Submarine {
    position: i32,
    depth: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            position: 0,
            depth: 0,
        }
    }

    pub fn process_commands(&mut self, commands: &Vec<String>) {
        for command in commands {
            let command = parse_command(command);
            self.process_command(&command);
        }
    }

    fn process_command(&mut self, command: &Command) {
        if command.direction == Direction::Forward {
            self.position += command.amount
        } else if command.direction == Direction::Down {
            self.depth += command.amount
        } else if command.direction == Direction::Up {
            self.depth -= command.amount
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
    fn processes_commands_correctly() {
        let inputs = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];

        let mut sub = Submarine::new();
        sub.process_commands(&inputs);

        assert_eq!(15, sub.position());
        assert_eq!(10, sub.depth());
    }
}
