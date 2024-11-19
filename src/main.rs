mod rover;

use rover::{action::{Action, MoveBackward, MoveForward, TurnLeft, TurnRight}, Rover};

fn main() {
    let mut rover = Rover::new(rover::direction::Direction::North, rover::position::Position::new(0, 0));

    let command = "ffrff";

    for c in command.chars() {
        let mut action: Box<dyn Action> = match c {
            'f' => Box::new(MoveForward{}),
            'b' => Box::new(MoveBackward{}),
            'l' => Box::new(TurnLeft{}),
            'r' => Box::new(TurnRight{}),
            _ => Box::new(MoveForward{}),
        };

        rover.execute(action.as_mut());
    }

    println!("Position: ({}, {})", rover.position.x, rover.position.y);
}
