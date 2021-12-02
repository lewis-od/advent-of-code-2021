pub mod part1;
pub mod part2;

pub trait Execute {
    fn execute(&self, submarine: &mut Submarine);
}

pub struct Submarine {
    pub position: i32,
    pub depth: i32,
    pub aim: i32,
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
