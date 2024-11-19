mod position;
mod direction;

use position::Position;
use direction::Direction;

struct Rover {
    position: Position,
    direction: Direction,
    command: Command,
}

impl Rover {
    fn execute() {
        let value = command::execute();
    }
}

pub fn print_rover() {
    println!("Hello, Rover!");
}

