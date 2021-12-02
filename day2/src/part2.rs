use crate::{Execute, Submarine};

#[derive(Debug, PartialEq)]
pub struct UpCommand {
    pub amount: i32,
}

impl Execute for UpCommand {
    fn execute(&self, submarine: &mut Submarine) {
        submarine.aim -= self.amount;
    }
}

#[derive(Debug, PartialEq)]
pub struct DownCommand {
    pub amount: i32,
}

impl Execute for DownCommand {
    fn execute(&self, submarine: &mut Submarine) {
        submarine.aim += self.amount;
    }
}

#[derive(Debug, PartialEq)]
pub struct ForwardCommand {
    pub amount: i32,
}

impl Execute for ForwardCommand {
    fn execute(&self, submarine: &mut Submarine) {
        submarine.position += self.amount;
        submarine.depth += self.amount * submarine.aim;
    }
}

pub fn parse_commands(commands: &Vec<&str>) -> Vec<Box<dyn Execute>> {
    commands
        .iter()
        .map(|command| parse_command(command))
        .collect()
}

fn parse_command(command: &str) -> Box<dyn Execute> {
    let parts: Vec<&str> = command.split(" ").collect();

    let amount = parts[1].parse::<i32>().unwrap();
    match parts[0] {
        "up" => Box::new(UpCommand { amount }),
        "down" => Box::new(DownCommand { amount }),
        "forward" => Box::new(ForwardCommand { amount }),
        _ => panic!("Unknown direction"),
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

        let commands = parse_commands(&inputs);

        let mut sub = Submarine::new();
        sub.process_commands(&commands);

        assert_eq!(15, sub.position());
        assert_eq!(60, sub.depth());
    }
}
