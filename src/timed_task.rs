use std::time::{Duration, Instant};

#[derive(Default)]
pub enum TaskObjectType {
    File,
    #[default]
    KG,
}

#[derive(Default, Clone, Copy)]
pub enum TaskType {
    InferNamespaces,
    #[default]
    Normalize,
}

pub struct Task {
    pub obj_name: String,
    pub obj_type: TaskObjectType,
    pub duration: Duration,
    pub triples: usize,
    pub size: usize,
    pub task_type: TaskType,
    start: Instant,
}

impl Task {
    pub fn new(name: String, task_type: TaskType) -> Task {
        Task {
            obj_name: name,
            obj_type: TaskObjectType::KG,
            task_type,
            start: Instant::now(),
            duration: Default::default(),
            triples: Default::default(),
            size: Default::default(),
        }
    }

    pub fn file_task(&self, path: String) -> Task {
        Task {
            obj_name: path,
            obj_type: TaskObjectType::File,
            task_type: self.task_type,
            start: Instant::now(),
            duration: Default::default(),
            triples: Default::default(),
            size: Default::default(),
        }
    }
}
