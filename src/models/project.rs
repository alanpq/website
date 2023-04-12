use log::debug;
use serde::Serialize;
use yaml_rust::Yaml;

#[derive(Serialize, Clone)]
pub struct ProjectFlags {
	pub readme_thumbnail: bool, // is thumbnail taken from the README
}

impl ProjectFlags {
	pub fn from(doc: &Yaml) -> ProjectFlags {
		debug!("{:?}", doc["flags"]);
		match doc["flags"].as_hash() {
			Some(flags) => {
				debug!("{:?}", flags);
				let rt = flags.get(&Yaml::from_str("readme_thumbnail")).unwrap();
				ProjectFlags {
					readme_thumbnail: rt.as_bool().unwrap_or(false),
					// readme_thumbnail: false,
				}
			}
			None => {
				ProjectFlags {
					readme_thumbnail: false,
				}
			}
		}
	}
}

#[derive(Serialize, Clone)]
pub struct Project {
	pub id: String,       // project id
	pub category: String, // current categories are: main, other

	pub title: String,       // project title
	pub description: String, // short text-only description
	pub body: String,        // full project description (contains HTML)

	pub thumbnail: String, // optional (although needed if in main category)

	pub url: String,    // url to project
	pub github: String, // url to github

	pub stars: i64,          // number of github stars
	pub forks: i64,          // number of github forks
	pub flags: ProjectFlags, // misc flags
}
