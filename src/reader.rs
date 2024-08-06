use crate::task::Task;
use crate::util::New;
pub trait FileReader {
    fn read(&self, file_name : &str) -> Vec<Task>{
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
   fn read(&self, file_name : &str) -> Vec<Task> {
        println!("Reading tasks from file {file_name}");
        serde_json::from_str(&std::fs::read_to_string(file_name).unwrap()).unwrap()
   }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_mock_task_list() {
        let reader = MockFileReader{};
        let tasks = reader.read("tasks.txt");
        assert_eq!(tasks.len(), 3);
    }

    struct MockFileReader {
    }

    impl FileReader for MockFileReader {
        fn read(&self, _: &str) -> Vec<Task> {
            // Return a mock list of tasks
            vec![
                Task::new("Mock Task 1".to_string()),
                Task::new("Mock Task 2".to_string()),
                Task::new("Mock Task 3".to_string()),
            ]
        }
    }

}
