use crate::ddb::Dao;
use crate::domain::work::{Work, WorkStatus};
use crate::{ddb, AppError, AppResult};
use std::collections::HashMap;

pub struct Application {
    #[allow(dead_code)]
    me_id: String,
    work_dao: Dao<Work>,
}

impl Application {
    pub async fn new(me_id: String) -> Self {
        let work_dao: Dao<Work> = Dao::new().await;

        Self { me_id, work_dao }
    }

    pub async fn list(
        &self,
        next_key: Option<String>,
        limit: Option<i32>,
    ) -> AppResult<(Vec<Work>, Option<String>)> {
        let paging_key = ddb::PagingKey::decode(next_key)?;

        let (works, next_key) = self
            .work_dao
            .get_by_status_with_pager(WorkStatus::Free, paging_key, limit)
            .await?;

        Ok((works, next_key.encode()))
    }

    pub async fn get_multi(&self, ids: Vec<String>) -> AppResult<Vec<Work>> {
        let works = self.work_dao.get_multi(ids.clone()).await?;

        let mut work_map: HashMap<String, Work> = HashMap::new();
        for work in works {
            work_map.insert(work.id.clone(), work.to_owned());
        }

        let ordered_works: Vec<Work> = ids
            .iter()
            .map(|v| work_map.get(v).to_owned())
            .filter(|v| v.is_some() && v.unwrap().status == WorkStatus::Free)
            .map(|v| v.unwrap().to_owned())
            .collect();

        Ok(ordered_works)
    }

    pub async fn get(&self, id: String) -> AppResult<Work> {
        let work = self.work_dao.get(id).await?;

        if work.status != WorkStatus::Free {
            return Err(AppError::NotFound);
        }

        Ok(work)
    }
}
