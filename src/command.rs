pub trait command {
    fn execute(&self, rover:Rover) -> i32;
}