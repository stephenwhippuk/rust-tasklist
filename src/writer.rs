pub trait FileWriter {
    fn write(&self, file_name : &str, tasks: Vec<Task>);
}

pub struct TaskListWriter
{
}

impl New for TaskListWriter
{
    fn new() -> TaskListWriter
    {
        TaskListWriter{}
    }
}

impl FileWriter for TaskListWriter
{
    fn write(&self, file_name : &str, tasks: Vec<Task>) {
        println!("Writing tasks to file {file_name}");
        std::fs::write(file_name, serde_json::to_string(&tasks).unwrap()).unwrap();
    }
}