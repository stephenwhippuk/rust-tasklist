// Main Application Module
use crate::task::Task;
use crate::reader::FileReader;

pub struct App{
    tasks: Vec<Task>,
    reader:  Box< dyn FileReader>
}

impl App {
    pub fn new(reader : Box<dyn FileReader>) -> App {
        let data = reader.read("taskList.dat");
        App{
            reader: reader,
            tasks : data   
        }
    }

    pub fn run(&self) {
        println!("Welcome to the Task Manager!");
        for task in &self.tasks {
            let status = if task.completed { "Completed" } else { "Pending" };
            println!("{} - {}", task.name, status);
        }
    }
}