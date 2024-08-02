use crate::task::Task;
use crate::util::New;

pub trait FileReader {
    fn read(&self, file_name : &str) -> Vec<Task>{
        vec![
            Task::new("Task 1".to_string()),
            Task::new("Task 2".to_string()),
            Task::new("Task 3".to_string()),
        ]
    }
}

pub struct TaskListReader
{
}

impl New for TaskListReader
{
    fn new() -> TaskListReader
    {
        TaskListReader{}
    }
}

impl FileReader for TaskListReader
{
   
}