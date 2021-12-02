
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

    pub fn process_commands(&mut self, commands: Vec<&str>) {
        for command in commands {
            self.process_command(command);
        }
    }

    fn process_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split(" ").collect();
        let direction = parts[0];
        let amount = parts[1].parse::<i32>().unwrap();
        if direction == "forward" {
            self.position += amount
        } else if direction == "down" {
            self.depth += amount
        } else if direction == "up" {
            self.depth -= amount
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
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        let mut sub = Submarine::new();
        sub.process_commands(inputs);

        assert_eq!(15, sub.position());
        assert_eq!(10, sub.depth());
    }
}
