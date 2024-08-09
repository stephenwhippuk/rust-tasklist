use crate::task::Task;
use crate::util::New;
use mockall::automock;

#[automock]
pub trait FileReader {
    fn read(&mut self, file_name : &str) -> Vec<Task>{
        println!("ignoring file {file_name} and returning preset list of tasks");
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
   fn read(&mut self, file_name : &str) -> Vec<Task> {
        println!("Reading tasks from file {file_name}");
        serde_json::from_str(&std::fs::read_to_string(file_name).unwrap()).unwrap()
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    #[test]
    fn test_read_mock_task_list() {
        // just playing with mockall here 
        let mut reader = MockFileReader::new();
        reader.expect_read()
            .times(1)
            .with(eq("tasks.json"))
            .return_const(vec![
                Task::new("Task 1".to_string()),
            ]);
        let tasks = reader.read("tasks.json");
        assert_eq!(tasks.len(), 1);

    }

    

}
