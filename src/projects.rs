use log::error;
use notify::{watcher, DebouncedEvent, RecommendedWatcher};
use yaml_rust::YamlLoader;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use crate::project::Project;
use crate::project::ProjectFlags;

macro_rules! getValue {
  ($doc:ident, $key:literal, $as:tt, $default:literal) => {
      $doc[$key].$as().unwrap_or($default)
  }
}

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
      return Projects {
          rx: rx,
          watcher: watcher,
          value: Arc::new(Mutex::new(HashMap::new())),
          by_category: Arc::new(Mutex::new(HashMap::new())),
      };
  }

  pub fn process(&self, path: std::path::PathBuf) {
      let file = File::open(path.clone());
      if file.is_err() {
          error!("File '{}' not found!", path.to_str().unwrap());
          return;
      }
      let mut file = file.unwrap();
      let mut s = String::new();
      file.read_to_string(&mut s).unwrap();
      let docs = YamlLoader::load_from_str(&s);
      if docs.is_err() {
          return;
      }

      let doc = &docs.unwrap()[0];
      match doc["id"].as_str() {
          Some(id) => {
              let category = String::from(getValue!(doc, "category", as_str, "other"));

              let mut projects = self.value.lock().unwrap();
              let project = Project {
                  id: id.to_string(),
                  category: category.clone(),
                  thumbnail: String::from(getValue!(doc, "thumbnail", as_str, "")),
                  title: String::from(getValue!(doc, "title", as_str, "default")),
                  body: String::from(getValue!(doc, "body", as_str, "default")),
                  description: String::from(getValue!(doc, "description", as_str, "default")),
                  url: String::from(getValue!(doc, "url", as_str, "")),
                  github: String::from(getValue!(doc, "github", as_str, "")),
                  stars: getValue!(doc, "stars", as_i64, 0),
                  forks: getValue!(doc, "forks", as_i64, 0),
                  flags: ProjectFlags::from(doc)
              };
              
              match projects.insert(id.to_string(),project) {
                  Some(_) => {},
                  None => {
                      let mut by_category = self.by_category.lock().unwrap();
                      if !by_category.contains_key(&category) {
                          by_category.insert(category.clone(), Vec::new());
                      }
                      by_category.get_mut(&category).unwrap().push(id.to_string());
                  }
              }
          }
          None => {
              error!(
                  "'{}' - Project ID not found!",
                  path.into_os_string()
                      .into_string()
                      .unwrap_or(String::from("???"))
              );
          }
      }
      
      // println!("{:?}", doc);
  }

  pub fn value(&self) -> Arc<Mutex<HashMap<String, Project>>> {
      self.rx.try_iter().for_each(|e: DebouncedEvent| match e {
          DebouncedEvent::NoticeWrite(p)  => self.process(p),
          DebouncedEvent::NoticeRemove(p) => self.process(p),
          DebouncedEvent::Create(p)       => self.process(p),
          DebouncedEvent::Write(p)        => self.process(p),
          DebouncedEvent::Chmod(p)        => self.process(p),
          DebouncedEvent::Remove(p)       => {
              self.value.lock().unwrap().remove(&String::from(p.file_name().unwrap().to_str().unwrap()));
          },
          DebouncedEvent::Rename(_, p)    => self.process(p),
          _ => {}
      });
      return self.value.clone();
  }

  pub fn categories(&self) -> Arc<Mutex<HashMap<String, Vec<String>>>> {
      return self.by_category.clone();
  }
}
