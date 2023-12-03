use std::fmt::Display;

use crate::time::Time;

pub enum TaskStaus {
    Incomplete,
    InProgress,
    Complete,
}

impl Display for TaskStaus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStaus::Incomplete => write!(f, "Incomplete"),
            TaskStaus::InProgress => write!(f, "In Progress"),
            TaskStaus::Complete => write!(f, "Complete"),
        }
    }
}

pub struct Task {
    id: u64,
    description: String,
    status: TaskStaus,
    start_time: Time,
    end_time: Option<Time>,
    subtasks: Vec<Task>,
}

impl Task {
    pub fn new(id: u64, description: String) -> Task {
        Task {
            id,
            description,
            status: TaskStaus::Incomplete,
            subtasks: Vec::new(),
            start_time: Time::now(),
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        self.status = TaskStaus::InProgress;
        self.start_time = Time::now();
    }

    pub fn finish(&mut self) {
        self.status = TaskStaus::Complete;
        self.end_time = Some(Time::now());
    }

    pub fn add_subtask(&mut self, id: u64, name: String) {
        self.subtasks.push(Task::new(id, name));
    }

    pub fn get_name(&self) -> &str {
        &self.description
    }

    pub fn get_status(&self) -> &TaskStaus {
        &self.status
    }

    pub fn get_subtasks(&self) -> &Vec<Task> {
        &self.subtasks
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}
