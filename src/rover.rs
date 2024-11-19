pub mod position;
pub mod direction;

use position::Position;
use direction::Direction;

pub struct Rover {
    pub position: Position,
    pub direction: Direction,
    // pub action: Box<dyn Action>, // Utilisation d'un pointeur pour un trait dynamique
}

impl Rover {
    pub fn new(direction: Direction, position: Position) -> Rover {
        Rover { direction, position }
    }

    // pub fn execute(&self) -> i32 {
    //     self.action.execute(&self.direction, &self.position) // Appel du trait
    // }
    pub fn move_forward(&mut self) -> i32 {
        self.position = match self.direction {
            Direction::North => Position::new(self.position.x, self.position.y + 1),
            Direction::East => Position::new(self.position.x + 1, self.position.y),
            Direction::South => Position::new(self.position.x, self.position.y - 1),
            Direction::West => Position::new(self.position.x - 1, self.position.y),
        };

        return 0;
    }

    pub fn move_backward(&mut self) -> i32 {
        self.position = match self.direction {
            Direction::North => Position::new(self.position.x, self.position.y - 1),
            Direction::East => Position::new(self.position.x - 1, self.position.y),
            Direction::South => Position::new(self.position.x, self.position.y + 1),
            Direction::West => Position::new(self.position.x + 1, self.position.y),
        };

        return 0;
    }

    pub fn turn_left(&mut self) -> i32 {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North
        };

        return 0;
    }

    pub fn turn_right(&mut self) -> i32 {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        };

        return 0;
    }
}
