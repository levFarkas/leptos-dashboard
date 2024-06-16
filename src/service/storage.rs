use std::fs;

use leptos::{server, ServerFnError};

use super::projects::Project;

#[server(SaveStorage, "/save")]
pub async fn save_storage(project: Project, eks_version: String) -> Result<bool, ServerFnError> {
    Ok(do_save_storage(project, eks_version).await)
}

pub async fn do_save_storage(project: Project, eks_version: String) -> bool {
    let data = fs::read_to_string("/tmp/leptos-dashboard.json").expect("Unable to read file");
    let mut projects: Vec<Project> = serde_json::from_str(&data).unwrap();
    let index = projects
        .iter()
        .position(|x| *x.name == project.name)
        .unwrap();
    let new_project = Project::new(project.name, eks_version);
    projects.remove(index);
    projects.push(new_project);
    if let output = serde_json::to_string(&projects).expect("Unable to read file") {
        fs::write("/tmp/leptos-dashboard.json", output);
        return true;
    }

    return false;
}
