use super::Rover;
use crate::rover::{Direction, Position};

pub trait Action {
    fn execute(&mut self, rover: &mut Rover);
}

pub struct MoveForward;

impl Action for MoveForward {
    fn execute(&mut self, rover: &mut Rover) {
        rover.position = match rover.direction {
            Direction::North => Position::new(rover.position.x, rover.position.y + 1),
            Direction::East => Position::new(rover.position.x + 1, rover.position.y),
            Direction::South => Position::new(rover.position.x, rover.position.y - 1),
            Direction::West => Position::new(rover.position.x - 1, rover.position.y),
        };
    }
}

pub struct MoveBackward;

impl Action for MoveBackward {
    fn execute(&mut self, rover: &mut Rover) {
        rover.position = match rover.direction {
            Direction::North => Position::new(rover.position.x, rover.position.y - 1),
            Direction::East => Position::new(rover.position.x - 1, rover.position.y),
            Direction::South => Position::new(rover.position.x, rover.position.y + 1),
            Direction::West => Position::new(rover.position.x + 1, rover.position.y),
        };
    }
}

pub struct TurnLeft;

impl Action for TurnLeft {
    fn execute(&mut self, rover: &mut Rover) {
        rover.direction.turn_left();
    }
}

pub struct TurnRight;

impl Action for TurnRight {
    fn execute(&mut self, rover: &mut Rover) {
        rover.direction.turn_right();
    }
}