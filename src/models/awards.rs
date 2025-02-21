use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use log::error;
use markdown::{mdast::Node, to_mdast, Constructs};
use serde::{Deserialize, Serialize};

use crate::util::parse;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    pub href: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Award {
    pub id: String,
    pub title: String,
    pub date: Option<String>,
    #[serde(skip_deserializing)]
    pub body: String,
    pub links: Option<Vec<Link>>,
}

impl Award {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Award, Box<dyn std::error::Error>> {
        let (mut award, body) = parse::read_file::<Award>(path)?;
        award.body = body;

        Ok(award)
    }
}

pub type Awards = Vec<Award>;

pub fn fetch_awards<P: AsRef<Path>>(path: P) -> Option<Awards> {
    match path.as_ref().read_dir() {
        Ok(res) => {
            let mut paths: Vec<PathBuf> = res
                .into_iter()
                .filter_map(|f| Some(f.ok()?.path()))
                .collect();
            paths.sort_unstable_by(|a, b| b.cmp(a));
            return Some(
                paths
                    .iter()
                    .filter_map(|p| Award::from_file(p).ok())
                    .collect(),
            );
        }
        Err(_) => error!("'{}' not found!", path.as_ref().display()),
    }
    None
}
