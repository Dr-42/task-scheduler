mod duration;
mod task;
mod time;

use task::Task;

struct App {
    tasks: Vec<Task>,
    running_id: u64,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl App {
    fn new() -> App {
        App {
            tasks: Vec::new(),
            running_id: 1,
        }
    }

    fn get_task<'a, 'b: 'a>(&'a self, parent: Option<&'b Task>, id: u64) -> Option<&Task> {
        if let Some(result) = parent {
            for subtask in result.get_subtasks() {
                if subtask.get_id() == id {
                    return Some(subtask);
                }
                if let Some(subtask_result) = self.get_task(Some(subtask), id) {
                    return Some(subtask_result);
                }
            }
            None
        } else {
            for task in self.tasks.iter() {
                if task.get_id() == id {
                    return Some(task);
                }
                if let Some(task_result) = self.get_task(Some(task), id) {
                    return Some(task_result);
                }
            }
            None
        }
    }

    fn add_subtask(&mut self, parent_id: u64, name: String) -> Result<()> {
        let parent = self
            .tasks
            .iter_mut()
            .find(|t| t.get_id() == parent_id)
            .ok_or("No task found with that id")?;
        let id = self.running_id;
        self.running_id += 1;
        parent.add_subtask(id, name);
        Ok(())
    }

    fn add_task(&mut self, parent_id: u64, name: String) -> Result<()> {
        if parent_id != 0 {
            let parent = self
                .tasks
                .iter_mut()
                .find(|t| t.get_id() == parent_id)
                .ok_or("No task found with that id")?;
            let id = self.running_id;
            self.running_id += 1;
            parent.add_subtask(id, name);
        } else {
            let id = self.running_id;
            self.running_id += 1;
            self.tasks.push(Task::new(id, name));
        }
        Ok(())
    }

    fn start_task(&mut self, id: u64) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.get_id() == id)
            .ok_or("No task found with that id")?;
        task.start();
        Ok(())
    }

    fn finish_task(&mut self, id: u64) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.get_id() == id)
            .ok_or("No task found with that id")?;
        task.finish();
        Ok(())
    }
}

fn main() {}
