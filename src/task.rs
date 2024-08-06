use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub completed: bool,
}

impl Task {
    pub fn new(name: String) -> Task {
        Task {
            name,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}