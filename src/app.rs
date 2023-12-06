use std::collections::HashMap;

use crate::{
    task::{Task, TaskStaus},
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    tasks: Vec<Task>,
    running_id: u64,
}

impl App {
    pub fn new() -> App {
        App {
            tasks: Vec::new(),
            running_id: 0,
        }
    }

    pub async fn save(&self) -> Result<()> {
        let serialized = serde_json::to_string_pretty(self)?;
        async_fs::write("data.json", serialized).await?;
        Ok(())
    }

    pub async fn load() -> Result<App> {
        let serialized = async_fs::read_to_string("data.json").await?;
        let app = serde_json::from_str(&serialized)?;
        Ok(app)
    }

    pub fn add_task(&mut self, name: String) -> Result<u64> {
        let task = Task::new(self.running_id, None, name);
        self.tasks.push(task);
        self.running_id += 1;
        Ok(self.running_id - 1)
    }

    pub fn add_subtask(&mut self, parent_id: u64, name: String) -> Result<u64> {
        let task = Task::new(self.running_id, Some(parent_id), name);
        self.tasks.push(task);
        self.running_id += 1;
        Ok(self.running_id - 1)
    }

    pub fn start_task(&mut self, id: u64) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.get_id() == id)
            .ok_or("Task not found")?;
        task.start();
        if task.get_parent_id().is_some() {
            let parent_id = task.get_parent_id().unwrap();
            let parent = self
                .tasks
                .iter_mut()
                .find(|task| task.get_id() == parent_id)
                .ok_or("Task not found")?;
            if parent.get_status() == &TaskStaus::Incomplete {
                self.start_task(parent_id)?;
            }
        }
        Ok(())
    }

    pub async fn stop_task(
        &mut self,
        id: u64,
        summary: Option<String>,
    ) -> Result<Option<HashMap<String, String>>> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.get_id() == id)
            .ok_or("Task not found")?;
        let images = task.stop(summary).await;
        Ok(images)
    }

    pub fn rename_task(&mut self, id: u64, name: String) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.get_id() == id)
            .ok_or("Task not found")?;
        task.rename(name);
        Ok(())
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
