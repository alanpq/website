use super::project::Project;
use crate::util::parse;
use log::debug;
use notify::{watcher, DebouncedEvent, RecommendedWatcher};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver},
        Arc, Mutex,
    },
    time::Duration,
};

pub struct Projects {
    pub rx: Receiver<notify::DebouncedEvent>,
    pub watcher: RecommendedWatcher,
    pub value: Arc<Mutex<HashMap<String, Project>>>,
    pub by_category: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Projects {
    pub fn new() -> Projects {
        let (tx, rx) = channel();
        let watcher = watcher(tx, Duration::from_secs(2)).unwrap();
        Projects {
            rx,
            watcher,
            value: Arc::new(Mutex::new(HashMap::new())),
            by_category: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn process(&self, path: std::path::PathBuf) {
        if let Some(path) = path.file_name() {
            if path.to_str().unwrap().contains("categories.yml") {
                return;
            }
        }

        let (mut project, body) = parse::read_file::<Project>(path).unwrap();
        project.body = body;

        let mut projects = self.value.lock().unwrap();
        let id = project.id.clone();
        let category = project.category.clone();

        match projects.insert(project.id.clone(), project) {
            Some(_) => {}
            None => {
                let mut by_category = self.by_category.lock().unwrap();
                by_category.entry(category).or_default().push(id);
            }
        }

        // println!("{:?}", doc);
    }

    pub fn value(&self) -> Arc<Mutex<HashMap<String, Project>>> {
        self.rx.try_iter().for_each(|e: DebouncedEvent| {
            debug!("fs watch triggered!");
            match e {
                DebouncedEvent::NoticeWrite(p) => self.process(p),
                DebouncedEvent::NoticeRemove(p) => self.process(p),
                DebouncedEvent::Create(p) => self.process(p),
                DebouncedEvent::Write(p) => self.process(p),
                DebouncedEvent::Chmod(p) => self.process(p),
                DebouncedEvent::Remove(p) => {
                    self.value
                        .lock()
                        .unwrap()
                        .remove(&String::from(p.file_name().unwrap().to_str().unwrap()));
                }
                DebouncedEvent::Rename(_, p) => self.process(p),
                _ => {}
            }
        });
        self.value.clone()
    }

    pub fn categories(&self) -> Arc<Mutex<HashMap<String, Vec<String>>>> {
        self.by_category.clone()
    }
}
