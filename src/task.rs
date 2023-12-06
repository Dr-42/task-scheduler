use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, path::Path};

use crate::time::Time;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    id: u64,
    parent_id: Option<u64>,
    name: String,
    status: TaskStaus,
    start_time: Option<Time>,
    end_time: Option<Time>,
    summary: Option<String>,
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
            summary: None,
        }
    }

    pub fn start(&mut self) {
        self.status = TaskStaus::InProgress;
        self.start_time = Some(Time::now());
    }

    async fn get_images(&self, summary: &str) -> HashMap<String, String> {
        println!("Getting images");
        let mut images = HashMap::new();
        for line in summary.lines() {
            if line.contains("![") {
                let mut split = line.split("](");
                let image = split.next().unwrap().split('[').nth(1).unwrap();
                let path = split.next().unwrap().split(')').next().unwrap();
                if path.starts_with("http") {
                    continue;
                }
                images.insert(image.to_string(), path.to_string());
                println!("{} {}", image, path);
            }
        }
        images
    }

    pub async fn stop(&mut self, summary: Option<String>) -> Option<HashMap<String, String>> {
        self.status = TaskStaus::Complete;
        self.end_time = Some(Time::now());
        if let Some(summary) = summary {
            if !Path::exists(Path::new("summaries")) {
                async_fs::create_dir("summaries").await.unwrap();
            }

            let summary_images = self.get_images(&summary).await;
            if !summary_images.is_empty() {
                if !Path::exists(Path::new("temp")) {
                    async_fs::create_dir("temp").await.unwrap();
                }
                let temp_md = format!("temp/{}.md", self.id);
                async_fs::write(&temp_md, summary).await.unwrap();
                return Some(summary_images);
            }

            let summary_text = markdown::to_html(&summary);
            async_fs::write(format!("summaries/{}.html", self.id), summary_text)
                .await
                .unwrap();
            self.summary = Some(format!("summaries/{}.html", self.id));
            return None;
        }
        None
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    #[allow(dead_code)]
    pub fn get_parent_id(&self) -> Option<u64> {
        self.parent_id
    }

    #[allow(dead_code)]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[allow(dead_code)]
    pub fn get_status(&self) -> &TaskStaus {
        &self.status
    }
}
