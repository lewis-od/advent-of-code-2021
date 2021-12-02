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
            self.process_command(command);
        }
    }

    fn process_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split(" ").collect();
        let direction = parts[0];
        let amount = parts[1].parse::<i32>().unwrap();
        if direction == "forward" {
            self.position += amount;
            self.depth += amount * self.aim;
        } else if direction == "down" {
            self.aim += amount;
        } else if direction == "up" {
            self.aim -= amount;
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

        let sub = Submarine::new();

        assert_eq!(15, sub.position());
        assert_eq!(60, sub.depth());
    }
}