use chrono::DateTime;

#[derive(Clone, Debug, Copy)]
pub struct Project<'a> {
    name: &'static str,
    eks_version: &'a str,
}

impl<'a> Project<'a> {
    pub fn name(self) -> &'a str {
        return self.name;
    }

    pub fn eks_version(self) -> &'a str {
        return self.eks_version;
    }
}

pub fn get_projects<'a>() -> Vec<Project<'a>> {
    vec![
        Project {
            name: "Project-1",
            eks_version: "1.25",
        },
        Project {
            name: "Project-2",
            eks_version: "1.27",
        },
        Project {
            name: "Project-3",
            eks_version: "1.24",
        },
        Project {
            name: "Project-4",
            eks_version: "1.23",
        },
    ]
}
