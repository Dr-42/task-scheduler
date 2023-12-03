use crate::{task::Task, Result};

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

    pub fn save(&self) -> Result<()> {
        let serialized = serde_json::to_string(&self.tasks)?;
        std::fs::write("tasks.json", serialized)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        let serialized = std::fs::read_to_string("tasks.json")?;
        self.tasks = serde_json::from_str(&serialized)?;
        Ok(())
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
        Ok(())
    }

    pub fn stop_task(&mut self, id: u64) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.get_id() == id)
            .ok_or("Task not found")?;
        task.stop();
        Ok(())
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
