use crate::aws::s3;
use crate::csv_loader::load_from_csv;
use crate::ddb;
use crate::domain::work::{Thumbnail, Work, WorkStatus};
use crate::AppResult;
use http::Uri;
use std::env;

pub struct Application {
    me_id: String,
    work_dao: ddb::Dao<Work>,
    thumbnail_dao: ddb::Dao<Thumbnail>,
}

impl Application {
    pub async fn new(me_id: String) -> Self {
        let work_dao: ddb::Dao<Work> = ddb::Dao::new().await;
        let thumbnail_dao: ddb::Dao<Thumbnail> = ddb::Dao::new().await;

        Self {
            me_id,
            work_dao,
            thumbnail_dao,
        }
    }

    pub async fn list(
        &self,
        status: Option<WorkStatus>,
        next_key: Option<String>,
        limit: Option<i32>,
    ) -> AppResult<(Vec<Work>, Option<String>)> {
        let paging_key = ddb::PagingKey::decode(next_key)?;

        let (works, next_key) = if let Some(status) = status {
            self.work_dao
                .get_by_status_with_pager(status, paging_key, limit)
                .await?
        } else {
            self.work_dao.get_with_pager(paging_key, limit).await?
        };

        Ok((works, next_key.encode()))
    }

    pub async fn get(&self, id: String) -> AppResult<Work> {
        let work = self.work_dao.get(id).await?;
        Ok(work)
    }

    pub async fn import_work(&self, prefix: String, file_name: String) -> AppResult<()> {
        let s3_key = format!("{}/{}", prefix, file_name);
        let s3_data = s3::download_object(env::var("S3_USER_BUCKET").unwrap(), s3_key).await?;

        let works = load_from_csv::<Work>(s3_data, None)?;
        for work in works {
            self.work_dao.put(&work).await?;
        }

        Ok(())
    }

    pub async fn import_thumbnail(&self, prefix: String, file_name: String) -> AppResult<()> {
        let s3_key = format!("{}/{}", prefix, file_name);
        let s3_data = s3::download_object(env::var("S3_USER_BUCKET").unwrap(), s3_key).await?;

        let thumbnails = load_from_csv::<Thumbnail>(s3_data, None)?;
        for thumbnail in thumbnails {
            self.thumbnail_dao.put(&thumbnail).await?;
        }

        Ok(())
    }

    pub async fn pre_sign_for_upload(&self, prefix: String) -> AppResult<(Uri, String)> {
        let uuid = uuid::Uuid::new_v4().to_string();

        let s3_key = format!("{}/{}", prefix, uuid);
        let url =
            s3::pre_sign_for_upload(env::var("S3_USER_BUCKET").unwrap(), s3_key.clone()).await?;

        Ok((url, uuid))
    }

    pub async fn update_status(&self, id: String, status: WorkStatus) -> AppResult<()> {
        let mut work = self.work_dao.get(id.clone()).await?;
        work.update_status(status)?;

        self.work_dao.put(&work).await?;

        Ok(())
    }

    pub async fn delete(&self, id: String) -> AppResult<()> {
        self.work_dao.delete(id.clone()).await?;
        let thumbnails = self.thumbnail_dao.get_by_work(id).await?;
        for thumbnail in thumbnails {
            self.thumbnail_dao.delete(thumbnail.id.to_owned()).await?;
        }

        Ok(())
    }
}
