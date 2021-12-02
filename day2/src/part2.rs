use crate::{Execute, Submarine};

#[derive(Debug, PartialEq)]
struct UpCommand {
    amount: i32,
}

impl Execute for UpCommand {
    fn execute(&self, submarine: &mut Submarine) {
        submarine.aim -= self.amount;
    }
}

#[derive(Debug, PartialEq)]
struct DownCommand {
    amount: i32,
}

impl Execute for DownCommand {
    fn execute(&self, submarine: &mut Submarine) {
        submarine.aim += self.amount;
    }
}

#[derive(Debug, PartialEq)]
struct ForwardCommand {
    amount: i32,
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
    fn processes_commands_correctly() {
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

    #[test]
    fn up_command_decreases_aim() {
        let amount = 5;
        let command = Box::new(UpCommand { amount });

        let mut submarine = Submarine::new();
        submarine.process_commands(&vec![command]);

        assert_eq!(-amount, submarine.aim);
    }

    #[test]
    fn down_command_increases_aim() {
        let amount = 5;
        let command = Box::new(DownCommand { amount });

        let mut submarine = Submarine::new();
        submarine.process_commands(&vec![command]);

        assert_eq!(amount, submarine.aim);
    }

    #[test]
    fn fprward_command_increases_position_and_depth() {
        let amount = 5;
        let aim = 2;
        let command = Box::new(ForwardCommand { amount });

        let mut submarine = Submarine::new();
        submarine.aim = aim;
        submarine.process_commands(&vec![command]);

        assert_eq!(amount, submarine.position());
        assert_eq!(amount * aim, submarine.depth());
    }
}
