use crate::kbdin::KeyboardInput;

pub enum Command {
    Add(String),
    Complete(usize),
    List,
    Exit
}

pub struct CommandParser
{
    reader: Box<dyn KeyboardInput>
}

impl CommandParser {
    pub fn new(reader: Box<dyn KeyboardInput>) -> CommandParser {
        CommandParser {
            reader: reader
        }
    }

    pub fn parse(&mut self) -> Result<Command, &str > {
        println!("Enter Command: ");
        let input = self.reader.get_input();
        match input.as_str() {
            "exit" => Ok(Command::Exit),
            "list" => Ok(Command::List),
            _ => Err("Invalid Command")
        }
    }
}