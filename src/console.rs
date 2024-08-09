use crate::util::New;
use mockall::automock;

#[automock]
pub trait ConsoleWriter {
    fn write(&self, message: &str){
        println!("{}", message);
    }
}

pub struct ConsoleOutput;

impl ConsoleWriter for ConsoleOutput {}

impl New for ConsoleOutput {
    fn new() -> ConsoleOutput {
        ConsoleOutput{}
    }
}