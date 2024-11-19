mod position;
mod direction;
mod action;

use position::Position;
use direction::Direction;
use action::Action;

pub struct Rover {
    pub position: Position,
    pub direction: Direction,
    pub action: Box<dyn Action>, // Utilisation d'un pointeur pour un trait dynamique
}

impl Rover {
    pub fn execute(&self) -> i32 {
        self.action.execute(&self.direction, &self.position) // Appel du trait
    }
}

pub fn print_rover() {
    println!("Hello, Rover!");
}
