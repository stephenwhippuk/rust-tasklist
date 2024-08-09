
use crate::console::ConsoleWriter;
use crate::kbdin::KeyboardInput;
// Main Application Module
use crate::task::Task;
use crate::reader::FileReader;
use crate::writer::FileWriter;
use crate::cmdparse::CommandParser;
use crate::cmdparse::Command;

const FILE_NAME : &str = ".\\target\\debug\\data\\tasklist.json";

pub struct App{
    tasks: Vec<Task>,
    reader: Box< dyn FileReader>,
    writer: Box<dyn FileWriter>,
    console: Box<dyn ConsoleWriter>,
    commmand_parser: CommandParser
}

impl App {
    pub fn new(  mut reader : Box<dyn FileReader>,
                 writer : Box<dyn FileWriter>, 
                 kbd : Box<dyn KeyboardInput>, 
                 console : Box<dyn ConsoleWriter>) -> App {
        let data = reader.read(FILE_NAME);
        App{
            writer: writer,
            reader: reader,
            tasks : data,
            commmand_parser: CommandParser::new(kbd),
            console: console
        }
    }

    pub fn reload_file(&mut self) {
        self.tasks = self.reader.read(FILE_NAME);
    }

    pub fn run(&mut self) {
        self.tasks = self.reader.read(FILE_NAME);
        loop {
            match self.commmand_parser.parse() {
                Ok(command) => {
                    match command {
                        Command::Add(task) => {
                            self.tasks.push(Task::new(task));
                            self.writer.write(FILE_NAME, &self.tasks);
                        },
                        Command::Complete(index) => {
                            self.tasks.remove(index);
                            self.writer.write(FILE_NAME, &self.tasks);
                        },
                        Command::List => {
                            for (index, task) in self.tasks.iter().enumerate() {
                                self.console.write(&format!("{}: {}", index, task.name));
                            }
                        },
                        Command::Exit => {
                            self.writer.write(FILE_NAME, &self.tasks);
                            break;
                        }
                    }
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console;
    use crate::reader::MockFileReader;
    use crate::writer::MockFileWriter;
    use crate::kbdin::MockKeyboardInput;
    use mockall::predicate::*;
    use mockall::Sequence;

    // This test simulates the user entering the command "list" and then "exit"
    #[test]
    fn test_list_exit() {

        // this is the input that would come from the json file database

        let mut reader = MockFileReader::new();
        reader.expect_read()
            .times(2)
            .with(eq(FILE_NAME))
            .return_const(vec![
                Task::new("Task 1".to_string()),
                Task::new("Task 2".to_string()),
                Task::new("Task 3".to_string()),
            ]);

        // this is the output that would be written to the json file database

        let mut writer = MockFileWriter::new();

        writer.expect_write()
            .times(1)
            .with(eq(FILE_NAME), eq(vec![
                Task::new("Task 1".to_string()),
                Task::new("Task 2".to_string()),
                Task::new("Task 3".to_string()),
            ]))
            .return_const(());

        // This is the input the system would receive from the console if it were running

        let mut kbdin = MockKeyboardInput::new();
        let mut seq = Sequence::new();

        kbdin.expect_get_input()
            .times(1)
            .in_sequence(&mut seq)
            .return_const("list".to_string());

        kbdin.expect_get_input()
            .times(1)
            .in_sequence(&mut seq)
            .return_const("exit".to_string());

        // This is the output the system would send to the console if it were running

        let mut console = console::MockConsoleWriter::new();
        let mut seq = Sequence::new();
        console.expect_write()
            .times(1)
            .with(eq("0: Task 1"))
            .in_sequence(&mut seq)
            .return_const(());
                
        console.expect_write()
            .times(1)
            .with(eq("1: Task 2"))
            .in_sequence(&mut seq)
            .return_const(());
                
        console.expect_write()
            .times(1)
            .with(eq("2: Task 3"))
            .in_sequence(&mut seq)
            .return_const(());


        // perform the actual test
        
        let mut app = App::new(Box::new(reader), Box::new(writer), Box::new(kbdin), Box::new(console));
        app.run();

    }
}