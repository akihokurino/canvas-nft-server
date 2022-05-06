use crate::aws::s3;
use crate::aws::s3::upload_object;
use crate::csv_loader::load_from_csv;
use crate::domain::work::{Thumbnail, Work, WorkStatus};
use crate::AppResult;
use crate::{ddb, internal_api, THUMBNAIL_CSV_PATH_PREFIX, WORK_CSV_PATH_PREFIX};
use http::Uri;
use std::collections::HashMap;
use std::env;

pub struct Application {
    #[allow(dead_code)]
    me_id: String,
    work_dao: ddb::Dao<Work>,
    thumbnail_dao: ddb::Dao<Thumbnail>,
    internal_api: internal_api::Client,
}

impl Application {
    pub async fn new(me_id: String) -> Self {
        let work_dao: ddb::Dao<Work> = ddb::Dao::new().await;
        let thumbnail_dao: ddb::Dao<Thumbnail> = ddb::Dao::new().await;
        let internal_api = internal_api::Client::new();

        Self {
            me_id,
            work_dao,
            thumbnail_dao,
            internal_api,
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
        Ok(work)
    }

    pub async fn sync_work_thumbnail(&self) -> AppResult<()> {
        let urls = self
            .internal_api
            .get_signed_urls(vec![
                "gs://canvas-329810-csv/work.csv".to_string(),
                "gs://canvas-329810-csv/thumbnail.csv".to_string(),
            ])
            .await?;

        let url = urls[0].clone();
        let bytes = reqwest::get(url).await?.bytes().await?;
        let s3_key = format!("{}/work.csv", WORK_CSV_PATH_PREFIX);
        upload_object(
            env::var("S3_USER_BUCKET").unwrap(),
            s3_key,
            bytes.clone(),
            "text/csv".to_string(),
        )
        .await?;
        let works = load_from_csv::<Work>(bytes, None)?;
        for work in works {
            self.work_dao.put(&work).await?;
        }

        let url = urls[1].clone();
        let bytes = reqwest::get(url).await?.bytes().await?;
        let s3_key = format!("{}/thumbnail.csv", THUMBNAIL_CSV_PATH_PREFIX);
        upload_object(
            env::var("S3_USER_BUCKET").unwrap(),
            s3_key,
            bytes.clone(),
            "text/csv".to_string(),
        )
        .await?;
        let thumbnails = load_from_csv::<Thumbnail>(bytes, None)?;
        for thumbnail in thumbnails {
            self.thumbnail_dao.put(&thumbnail).await?;
        }

        Ok(())
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

    pub async fn delete(&self, id: String) -> AppResult<()> {
        self.work_dao.delete(id.clone()).await?;
        let thumbnails = self.thumbnail_dao.get_by_work(id).await?;
        for thumbnail in thumbnails {
            self.thumbnail_dao.delete(thumbnail.id.to_owned()).await?;
        }

        Ok(())
    }
}
