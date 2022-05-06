use app::domain;

#[derive(Debug, GraphQLEnum)]
pub enum WorkStatus {
    Prepare,
    PublishNFT,
    SellOrder,
}

impl WorkStatus {
    pub fn domain(&self) -> domain::work::WorkStatus {
        match self {
            WorkStatus::Prepare => domain::work::WorkStatus::Prepare,
            WorkStatus::PublishNFT => domain::work::WorkStatus::PublishNFT,
            WorkStatus::SellOrder => domain::work::WorkStatus::SellOrder,
        }
    }
}

impl From<domain::work::WorkStatus> for WorkStatus {
    fn from(data: domain::work::WorkStatus) -> Self {
        match data {
            domain::work::WorkStatus::Prepare => WorkStatus::Prepare,
            domain::work::WorkStatus::PublishNFT => WorkStatus::PublishNFT,
            domain::work::WorkStatus::SellOrder => WorkStatus::SellOrder,
        }
    }
}
