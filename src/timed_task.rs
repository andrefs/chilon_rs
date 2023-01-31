use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

use log::info;
use serde::Serialize;

#[derive(Default, Serialize, Debug)]
pub enum TaskObjectType {
    File,
    #[default]
    KG,
}

#[derive(Default, Clone, Copy, Serialize, Debug)]
pub enum TaskType {
    Execution,
    InferNamespaces,
    Visualization,

    #[default]
    Normalize,
}

#[derive(Serialize, Debug)]
pub struct Task {
    pub obj_name: String,
    pub obj_type: TaskObjectType,
    pub duration: Duration,
    pub triples: usize,
    pub size: usize,
    pub task_type: TaskType,
    pub parent: Option<String>,

    #[serde(skip)]
    pub file: PathBuf,

    #[serde(skip)]
    start: Instant,
}

impl Task {
    pub fn new(name: String, task_type: TaskType, file: PathBuf) -> Task {
        Task {
            obj_name: name,
            obj_type: TaskObjectType::KG,
            task_type,
            start: Instant::now(),
            duration: Default::default(),
            triples: Default::default(),
            file,
            parent: None,
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
            parent: Some(self.obj_name.clone()),
            file: self.file.clone(),
            triples: Default::default(),
            size: Default::default(),
        }
    }

    pub fn finish(&mut self, msg: &str) {
        self.duration = self.start.elapsed();

        info!("{msg} ({:?})", self.duration);

        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(self.file.clone())
            .unwrap();

        writeln!(fd, "{}", serde_json::to_string(self).unwrap()).unwrap();
    }
}
