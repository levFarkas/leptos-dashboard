use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;

use super::{
    aws_eks_versions::get_versions,
    projects::{get_projects, Project},
};
use chrono::{DateTime, Local, TimeZone, Utc};

pub struct ProjectHandler<'a> {
    projects: Vec<Project<'a>>,
}
impl<'a> ProjectHandler<'a> {
    pub fn new() -> Self {
        Self {
            projects: get_projects(),
        }
    }
    pub async fn get_active_projects(self) -> Vec<Project<'a>> {
        // TODO, use async operator
        // self.projects
        //     .iter()
        //     .filter(|p| project_within_month(&p, 12, 36).await)
        //     .cloned()
        //     .collect()
        let mut active_project = Vec::new();
        for p in self.projects {
            if project_within_month(&p, 12, 48).await {
                active_project.push(p);
            }
        }
        return active_project;
    }

    pub async fn get_nearly_expired_projects(self) -> Vec<Project<'a>> {
        let mut projects = Vec::new();
        for p in self.projects {
            if project_within_month(&p, 6, 12).await {
                projects.push(p);
            }
        }
        return projects;
    }

    pub async fn get_close_to_expired_projects(self) -> Vec<Project<'a>> {
        let mut projects = Vec::new();
        for p in self.projects {
            if project_within_month(&p, -6, 6).await {
                projects.push(p);
            }
        }
        return projects;
    }
}

async fn get_date(input: &String) -> DateTime<Local> {
    let months_id_map = HashMap::from([
        ("January", "01"),
        ("February", "02"),
        ("March", "03"),
        ("April", "04"),
        ("May", "05"),
        ("June", "06"),
        ("July", "07"),
        ("August", "08"),
        ("September", "09"),
        ("October", "10"),
        ("November", "11"),
        ("December", "12"),
    ]);

    let mut dt_str = input.clone();

    for (month, id) in &months_id_map {
        dt_str = dt_str.replace(month, id);
    }

    dt_str = dt_str.replace(",", "");
    let components: Vec<&str> = dt_str.split(" ").collect();
    let (y, m, d) = (components[2], components[0], components[1]);
    let naive: NaiveDateTime = NaiveDate::from_ymd_opt(
        y.parse::<i32>().unwrap(),
        m.parse::<u32>().unwrap(),
        d.parse::<u32>().unwrap(),
    )
    .unwrap()
    .and_hms_opt(1, 1, 1) // I don't need to know the hours/minutes/seconds only the date
    .unwrap();
    return TimeZone::from_utc_datetime(&Local, &naive);
}

async fn project_within_month<'a>(p: &Project<'a>, from_month: i64, to_month: i64) -> bool {
    let version_result = get_versions().await;
    match version_result {
        Ok(versions) => {
            for version in versions {
                if version.version == p.eks_version() {
                    let dt = get_date(&version.end_of_extended_support).await;
                    let today = Utc::now();
                    let diff = today.signed_duration_since(&dt);
                    let days = diff.num_days();
                    let months = days / 30 * -1;
                    if months > from_month && months <= to_month {
                        return true;
                    }
                    break;
                }
            }

            false
        }
        Err(_error) => false,
    }
}
