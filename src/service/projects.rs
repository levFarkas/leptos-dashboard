use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub eks_version: String,
}

impl Project {
    pub fn new(name: String, eks_version: String) -> Self {
        Self { name, eks_version }
    }
}

#[server(GetProjects, "/projects")]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let project = get_projects_from_db();
    Ok(get_projects_from_db().await)
}

async fn get_projects_from_db() -> Vec<Project> {
    let data = fs::read_to_string("/tmp/leptos-dashboard.json").expect("Unable to read file");
    let projects: Vec<Project> = serde_json::from_str(&data).unwrap();
    return projects;
}
