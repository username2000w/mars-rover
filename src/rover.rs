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
    // pub fn execute(&self) -> i32 {
    //     self.action.execute(&self.direction, &self.position) // Appel du trait
    // }
    pub fn moveForward(&mut self, position:Position , direction:Direction) -> i32 {
        self.position = match direction {
            Direction::North => Position::new(position.x, position.y + 1),
            Direction::East => Position::new(position.x + 1, position.y),
            Direction::South => Position::new(position.x, position.y - 1),
            Direction::West => Position::new(position.x - 1, position.y),
        };

        return 0;

    }

    pub fn moveBackward(&mut self, position:Position , direction:Direction) -> i32 {
        self.position = match direction {
            Direction::North => Position::new(position.x, position.y - 1),
            Direction::East => Position::new(position.x - 1, position.y),
            Direction::South => Position::new(position.x, position.y + 1),
            Direction::West => Position::new(position.x + 1, position.y),
        };

        return 0;

    }

    pub fn moveleft(&mut self, position:Position , direction:Direction) -> i32 {
        self.position = match direction {
            Direction::North => Position::new(position.x - 1, position.y),
            Direction::East => Position::new(position.x, position.y + 1),
            Direction::South => Position::new(position.x + 1, position.y),
            Direction::West => Position::new(position.x, position.y - 1),
        };

        return 0;

    }

    pub fn moveRight(&mut self, position:Position , direction:Direction) -> i32 {
        self.position = match direction {
            Direction::North => Position::new(position.x + 1, position.y),
            Direction::East => Position::new(position.x, position.y - 1),
            Direction::South => Position::new(position.x - 1, position.y),
            Direction::West => Position::new(position.x, position.y + 1),
        };

        return 0;

    }
}

pub fn print_rover() {
    println!("Hello, Rover!");
}
