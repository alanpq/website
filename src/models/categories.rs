use log::{info, debug};
use serde::Serialize;
use std::{
	fs::File,
	io::Read,
	path::Path,
};

use crate::util::yaml::yaml_err;

use yaml_rust::{Yaml, YamlLoader, yaml};

#[derive(Serialize, Clone)]
#[derive(Debug)]
pub struct Category {
	pub name: String,
	pub blurb: Option<String>,
	pub hidden: bool,
}

impl Category {
	pub fn from<S: AsRef<str>>(str: S) -> Category {
		Category {
			name: str.as_ref().to_string(),
			blurb: None,
			hidden: false,
		}
	}
	pub fn from_yaml(hash: &yaml::Hash) -> Option<Category> {
		let blurb = match hash.get(&Yaml::String(format!("blurb"))) {
			Some(b) => Some(b.as_str()?.to_string()),
			None => None
		};

		let hidden = match hash.get(&Yaml::String(format!("hidden"))) {
			Some(b) => b.as_bool()?,
			None => false
		};

		Some(Category{
			name: hash.get(&Yaml::String(format!("name")))?.as_str()?.to_string(),
			blurb: blurb,
			hidden: hidden,
		})
	}
}

pub type Categories = Vec<Category>;

pub fn get_categories<P: AsRef<Path>>(path: P) -> Result<Categories, Box<dyn std::error::Error>> {
	let path = path.as_ref().to_owned();
	debug!("loading categories from file '{}'", path.display());
	let mut file = File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	let docs = YamlLoader::load_from_str(&s)?;
	info!("{} docs", docs.len());

	let categories = yaml_err(
		yaml_err(
			yaml_err(docs[0].as_hash(), "could not parse as hash")?
				.get(&Yaml::String("categories".to_string())),
			"no categories key",
		)?
		.as_vec(),
		"categories is not a list",
	)?;

	Ok(categories
		.iter()
		.filter_map(|e| {
			return match e {
				Yaml::Hash(e) => Category::from_yaml(e),
				Yaml::String(str) => Some(Category::from(str)),
				_ => None
			}
		})
		.collect()
	)
}