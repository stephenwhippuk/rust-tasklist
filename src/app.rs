// Main Application Module
use crate::task::Task;
use crate::reader::FileReader;
use crate::writer::FileWriter;

const FILE_NAME : &str = ".\\target\\debug\\data\\tasklist.json";

pub struct App{
    tasks: Vec<Task>,
    reader:  Box< dyn FileReader>,
    writer: Box<dyn FileWriter>
}

impl App {
    pub fn new(reader : Box<dyn FileReader>, writer : Box<dyn FileWriter>) -> App {
        let data = reader.read(FILE_NAME);
        App{
            writer: writer,
            reader: reader,
            tasks : data   
        }
    }

    pub fn reload_file(&mut self) {
        self.tasks = self.reader.read(FILE_NAME);
    }

    pub fn run(&mut self) {
        println!("Welcome to the Task Manager!");
        for task in &self.tasks {
            let status = if task.completed { "Completed" } else { "Pending" };
            println!("{} - {}", task.name, status);
        }
        self.tasks[0].complete();
        self.writer.write(FILE_NAME, self.tasks.clone());
    }
}