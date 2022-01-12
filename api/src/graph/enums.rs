use app::domain;

#[derive(Debug, GraphQLEnum)]
pub enum WorkStatus {
    Prepare,
    Free,
}

impl WorkStatus {
    pub fn domain(&self) -> domain::work::WorkStatus {
        match self {
            WorkStatus::Prepare => domain::work::WorkStatus::Prepare,
            WorkStatus::Free => domain::work::WorkStatus::Free,
        }
    }
}

impl From<domain::work::WorkStatus> for WorkStatus {
    fn from(data: domain::work::WorkStatus) -> Self {
        match data {
            domain::work::WorkStatus::Prepare => WorkStatus::Prepare,
            domain::work::WorkStatus::Free => WorkStatus::Free,
        }
    }
}
