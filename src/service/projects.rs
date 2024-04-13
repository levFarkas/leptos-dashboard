use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

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
    vec![
        Project {
            name: "Project-1".to_string(),
            eks_version: "1.25".to_string(),
        },
        Project {
            name: "Project-2".to_string(),
            eks_version: "1.27".to_string(),
        },
        Project {
            name: "Project-3".to_string(),
            eks_version: "1.24".to_string(),
        },
        Project {
            name: "Project-4".to_string(),
            eks_version: "1.23".to_string(),
        },
    ]
}
