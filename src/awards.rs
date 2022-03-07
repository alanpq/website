
use std::{path::{Path, PathBuf}, fs::File, io::Read};

use log::error;
use serde::Serialize;
use yaml_rust::{YamlLoader, Yaml, yaml};

use crate::util::yaml::yaml_err;

#[derive(Serialize, Clone)]
pub struct Link {
  pub href: String,
  pub name: String,
}

impl Link {
  pub fn from_yaml(yml: &yaml::Hash) -> Option<Link>{
    Some(Link {
      href: yml[&Yaml::String(format!("href"))].as_str()?.to_string(),
      name: yml[&Yaml::String(format!("name"))].as_str()?.to_string(),
    })
  }
}

#[derive(Serialize, Clone)]
pub struct Award {
  pub id: String,
  pub title: String,
  pub date: Option<String>,
  pub body: String,
  pub links: Option<Vec<Link>>,
}

impl Award {
  pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Award, Box<dyn std::error::Error>> {
    let path = path.as_ref();

    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let yml = &YamlLoader::load_from_str(&buf)?[0];

    let id = yaml_err(yml["id"].as_str(), "award needs id!")?;
    let title = yaml_err(yml["title"].as_str(), "award needs title!")?;
    let body = yaml_err(yml["body"].as_str(), "award needs body!")?;

    let date = match yml["date"].as_str() {
      Some(str) => Some(str.to_string()),
      None => None
    };

    let links: Option<Vec<Link>> = match yml["links"].as_vec() {
      Some(vec) => Some(vec.iter().filter_map(|link| Link::from_yaml(link.as_hash()?)).collect()),
      _ => None
    };


    Ok(Award {
        id: id.to_string(),
        title: title.to_string(),
        date,
        body: body.to_string(),
        links,
    })
  }
}

pub type Awards = Vec<Award>;

pub fn fetch_awards<P: AsRef<Path>>(path: P) -> Option<Awards> {
  match path.as_ref().read_dir() {
    Ok(res) => {
      let mut paths: Vec<PathBuf> = res.into_iter().filter_map(|f| Some(f.ok()?.path())).collect();
      paths.sort_unstable_by(|a,b| b.cmp(a));
      return Some(paths.iter().filter_map(|p| Some(Award::from_file(p).ok()?)).collect());
    },
    Err(_) => error!("'{}' not found!", path.as_ref().display()),
  }
  None
}