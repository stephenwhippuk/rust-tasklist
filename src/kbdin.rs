use crate::util::New;
use mockall::automock;

#[automock]
pub trait KeyboardInput {
    fn get_input(&mut self) -> String{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

pub struct ConsoleInput;

impl New for ConsoleInput {
    fn new() -> ConsoleInput {
        ConsoleInput{}
    }
    
}

impl KeyboardInput for ConsoleInput {}

