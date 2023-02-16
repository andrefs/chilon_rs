use std::{
    collections::BTreeMap,
    fs::{metadata, File},
    io::Write,
    ops::Add,
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
    Maintenance,

    #[default]
    Normalize,
}

#[derive(Serialize, Debug)]
pub struct Task {
    pub name: String,
    pub duration: Duration,
    pub size: usize,
    pub task_type: TaskType,

    pub triples: usize,
    pub blanks: usize,
    pub iris: usize,
    pub literals: usize,
    pub unknowns: usize,

    #[serde(skip)]
    start: Instant,
}

impl Task {
    pub fn new(name: String, task_type: TaskType) -> Task {
        Task {
            name,
            duration: Default::default(),
            triples: 0,
            iris: 0,
            blanks: 0,
            unknowns: 0,
            literals: 0,
            size: 0,
            task_type,
            start: Instant::now(),
        }
    }

    pub fn finish(&mut self, msg: &str) {
        self.duration = self.start.elapsed();

        info!("{msg} ({:?})", self.duration);
    }
}

#[derive(Serialize, Debug)]
pub struct MetaInfoInference {
    pub triples: usize,
    pub iris: usize,
    pub literals: usize,
    pub blanks: usize,

    pub duration: Duration,
    pub size: usize,

    pub housekeeping: InferHK, // housekeeping performed on the iri trie

    pub tasks: BTreeMap<String, Task>,

    #[serde(skip)]
    start: Instant,
}

impl MetaInfoInference {
    pub fn add_tasks(&mut self, tasks: BTreeMap<String, Task>) {
        for (_, task) in tasks {
            self.triples += task.triples;
            self.blanks += task.blanks;
            self.literals += task.literals;
            self.iris += task.iris;

            self.size += task.size;

            self.tasks.insert(task.name.clone(), task);
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct InferHK {
    pub rounds: usize, // number of rounds of housekeeping performed on the iri trie
    pub duration: Duration, // total duration of housekeeping (maintenance) performed on the iri trie
    pub discarded_ns: usize, // number of discarded iris due to low frequency
    pub inferred_ns: usize, // number of namespaces inferred from iris
    pub added_ns: usize,    // number of inferred iris actually new and added to the trie
}

#[derive(Debug, Clone, Copy)]
pub struct InferHKTask {
    pub rounds: usize,
    pub duration: Duration,
    pub start: Instant,
    pub discarded_ns: usize,
    pub inferred_ns: usize,
    pub added_ns: usize,
}

impl InferHK {
    pub fn add(&mut self, task: InferHKTask) {
        self.rounds += task.rounds;
        self.duration += task.duration;
        self.discarded_ns += task.discarded_ns;
        self.added_ns += task.added_ns;
        self.inferred_ns += task.inferred_ns;
    }

    pub fn new() -> InferHK {
        InferHK {
            rounds: 0,
            duration: Default::default(),
            discarded_ns: 0,
            added_ns: 0,
            inferred_ns: 0,
        }
    }
}

impl InferHKTask {
    pub fn new() -> InferHKTask {
        InferHKTask {
            rounds: 0,
            duration: Default::default(),
            start: Instant::now(),
            discarded_ns: 0,
            added_ns: 0,
            inferred_ns: 0,
        }
    }

    pub fn finish(&mut self) {
        self.duration = self.start.elapsed();
    }
}

#[derive(Serialize, Debug)]
pub struct MetaInfoNormalization {
    pub triples: usize,
    pub iris: usize,
    pub literals: usize,
    pub blanks: usize,
    pub unknowns: usize,

    #[serde(skip)]
    start: Instant,
    pub size: usize,
    pub duration: Duration,
    pub namespaces: usize,
    pub tasks: BTreeMap<String, Task>,
}

impl MetaInfoNormalization {
    pub fn add_tasks(&mut self, tasks: BTreeMap<String, Task>) {
        for (_, task) in tasks {
            self.triples += task.triples;
            self.literals += task.literals;
            self.blanks += task.blanks;
            self.iris += task.iris;
            self.unknowns = task.unknowns;

            self.size += task.size;

            self.tasks.insert(task.name.clone(), task);
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MetaInfoVisualization {
    pub duration: Duration,
    pub size: usize,
    #[serde(skip)]
    start: Instant,
}

impl MetaInfoVisualization {
    pub fn new() -> MetaInfoVisualization {
        MetaInfoVisualization {
            duration: Default::default(),
            size: 0,
            start: Instant::now(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MetaInfoFull {
    pub duration: Duration,
    #[serde(skip)]
    start: Instant,
}

#[derive(Serialize, Debug)]
pub struct MetaInfo {
    pub file_path: PathBuf,

    pub inference: Option<MetaInfoInference>,
    pub normalization: Option<MetaInfoNormalization>,
    pub visualization: Option<MetaInfoVisualization>,
    pub full: MetaInfoFull,
}

impl MetaInfo {
    pub fn new(file_path: PathBuf) -> MetaInfo {
        MetaInfo {
            file_path,
            inference: None,
            normalization: None,
            visualization: None,
            full: MetaInfoFull {
                duration: Default::default(),
                start: Instant::now(),
            },
        }
    }

    pub fn save(&mut self) {
        let mut fd = File::create(self.file_path.clone()).unwrap();
        self.full.duration = self.full.start.elapsed();
        writeln!(fd, "{}", serde_json::to_string_pretty(self).unwrap()).unwrap();
    }
}

pub trait FileTask {
    fn file_task(&mut self, name: String);
}

pub trait HasTasks {
    fn inc_size(&mut self, size: usize);
    fn task_type(&self) -> TaskType;
    fn tasks(&self) -> &BTreeMap<String, Task>;
    fn tasks_mut(&mut self) -> &mut BTreeMap<String, Task>;
}

pub trait StageTask {
    fn new() -> Self;
    fn finish(&mut self, msg: &str);
}

impl<T> FileTask for T
where
    T: HasTasks,
{
    fn file_task(&mut self, path: String) {
        let file_size = metadata(path.clone()).unwrap().len() as usize;
        self.inc_size(file_size);
        let t = Task {
            name: path.clone(),
            task_type: self.task_type(),
            start: Instant::now(),
            duration: Default::default(),
            size: file_size,

            iris: 0,
            triples: 0,
            blanks: 0,
            literals: 0,
            unknowns: 0,
        };
        self.tasks_mut().insert(path, t);
    }
}

impl StageTask for MetaInfoInference {
    fn new() -> MetaInfoInference {
        MetaInfoInference {
            triples: 0,
            iris: 0,
            literals: 0,
            blanks: 0,
            duration: Default::default(),
            size: 0,
            housekeeping: InferHK::new(),
            tasks: Default::default(),
            start: Instant::now(),
        }
    }

    fn finish(&mut self, msg: &str) {
        self.duration = self.start.elapsed();
        info!("{msg} ({:?})", self.duration);
    }
}

impl StageTask for MetaInfoNormalization {
    fn new() -> MetaInfoNormalization {
        MetaInfoNormalization {
            triples: 0,
            iris: 0,
            unknowns: 0,
            literals: 0,
            namespaces: 0,
            duration: Default::default(),
            size: 0,
            blanks: 0,
            tasks: Default::default(),
            start: Instant::now(),
        }
    }

    fn finish(&mut self, msg: &str) {
        self.duration = self.start.elapsed();
        info!("{msg} ({:?})", self.duration);
    }
}

impl StageTask for MetaInfoVisualization {
    fn new() -> MetaInfoVisualization {
        MetaInfoVisualization {
            duration: Default::default(),
            size: 0,
            start: Instant::now(),
        }
    }

    fn finish(&mut self, msg: &str) {
        self.duration = self.start.elapsed();
        info!("{msg} ({:?})", self.duration);
    }
}

impl HasTasks for MetaInfoInference {
    fn inc_size(&mut self, size: usize) {
        self.size += size;
    }
    fn task_type(&self) -> TaskType {
        TaskType::InferNamespaces
    }
    fn tasks(&self) -> &BTreeMap<String, Task> {
        &self.tasks
    }
    fn tasks_mut(&mut self) -> &mut BTreeMap<String, Task> {
        &mut self.tasks
    }
}

impl HasTasks for MetaInfoNormalization {
    fn inc_size(&mut self, size: usize) {
        self.size += size;
    }
    fn task_type(&self) -> TaskType {
        TaskType::Normalize
    }
    fn tasks(&self) -> &BTreeMap<String, Task> {
        &self.tasks
    }
    fn tasks_mut(&mut self) -> &mut BTreeMap<String, Task> {
        &mut self.tasks
    }
}
