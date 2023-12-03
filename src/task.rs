use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::time::Time;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: u64,
    parent_id: Option<u64>,
    name: String,
    status: TaskStaus,
    start_time: Option<Time>,
    end_time: Option<Time>,
}

impl Task {
    pub fn new(id: u64, parent_id: Option<u64>, name: String) -> Task {
        Task {
            id,
            parent_id,
            name,
            status: TaskStaus::Incomplete,
            start_time: None,
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        self.status = TaskStaus::InProgress;
        self.start_time = Some(Time::now());
    }

    pub fn stop(&mut self) {
        self.status = TaskStaus::Complete;
        self.end_time = Some(Time::now());
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_parent_id(&self) -> Option<u64> {
        self.parent_id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_status(&self) -> &TaskStaus {
        &self.status
    }
}
