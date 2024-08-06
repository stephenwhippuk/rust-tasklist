// Main Application Module
use crate::task::Task;
use crate::reader::FileReader;
const FILE_NAME : &str = ".\\target\\debug\\data\\tasklist.json";
pub struct App{
    tasks: Vec<Task>,
    reader:  Box< dyn FileReader>
}

impl App {
    pub fn new(reader : Box<dyn FileReader>) -> App {
        let data = reader.read(FILE_NAME);
        App{
            reader: reader,
            tasks : data   
        }
    }

    pub fn reload_file(&mut self) {
        self.tasks = self.reader.read(FILE_NAME);
    }

    pub fn run(&self) {
        println!("Welcome to the Task Manager!");
        for task in &self.tasks {
            let status = if task.completed { "Completed" } else { "Pending" };
            println!("{} - {}", task.name, status);
        }
    }
}