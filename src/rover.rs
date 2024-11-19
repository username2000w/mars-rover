pub mod position;
pub mod direction;
pub mod action;

use position::Position;
use direction::Direction;
use action::Action;

pub struct Rover {
    pub position: Position,
    pub direction: Direction,
}

impl Rover {
    pub fn new(direction: Direction, position: Position) -> Rover {
        Rover { direction, position }
    }

    pub fn execute(&mut self, command: &mut dyn Action) -> i32 {
        command.execute(self);

        return 0;
    }
}
