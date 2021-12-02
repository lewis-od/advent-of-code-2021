use std::str::FromStr;

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "up"      => Ok(Direction::Up),
            "down"    => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _         => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub direction: Direction,
    pub amount: i32,
}

pub fn parse_command(command: &str) -> Command {
    let parts: Vec<&str> = command.split(" ").collect();

    Command {
        direction: Direction::from_str(parts[0]).unwrap(),
        amount: parts[1].parse::<i32>().unwrap(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parses_up_command() {
        let command_text = "up 7";

        let command = parse_command(command_text);

        assert_eq!(
            Command {
                direction: Direction::Up,
                amount: 7
            },
            command
        );
    }
}
