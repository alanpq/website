use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectFlags {
    pub readme_thumbnail: bool, // is thumbnail taken from the README
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: String,       // project id
    pub category: String, // current categories are: main, other

    pub title: String,       // project title
    pub year: String,        // project year/year range (2013, 2020 - 2023, 2020 - now, etc)
    pub description: String, // short text-only description
    #[serde(skip_deserializing)]
    pub body: String, // full project description (contains HTML)

    pub thumbnail: Option<String>, // optional (although needed if in main category)

    pub url: Option<String>,    // url to project
    pub github: Option<String>, // url to github

    pub stars: Option<i64>,          // number of github stars
    pub forks: Option<i64>,          // number of github forks
    pub flags: Option<ProjectFlags>, // misc flags
}
