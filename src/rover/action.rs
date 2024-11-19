use crate::rover::position::Position;
use crate::rover::direction::Direction;
pub trait Action {
    fn execute(&self, direction: &Direction, position: &Position) -> i32;
}
