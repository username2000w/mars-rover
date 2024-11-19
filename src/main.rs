mod rover;
use rover::Rover;

fn main() {
    let mut rover = Rover::new(rover::direction::Direction::North, rover::position::Position::new(0, 0));

    let command = "ffrff";

    for c in command.chars() {
        match c {
            'f' => {println!("moveForward"); rover.move_forward();}, // Appel de la fonction moveForward
            'b' => {println!("moveBackward"); rover.move_backward();}, // Appel de la fonction moveBackward
            'l' => {println!("moveleft"); rover.turn_left();}, // Appel de la fonction moveleft
            'r' => {println!("moveRight"); rover.turn_right();}, // Appel de la fonction moveRight
            _ => println!("Invalid command"), // Affichage d'un message d'erreur si la commande est invalide
        }
    }

    println!("Position: ({}, {})", rover.position.x, rover.position.y);
}
