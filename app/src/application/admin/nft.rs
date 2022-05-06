use crate::aws::s3::upload_object;
use crate::domain::asset::{Asset1155, Asset721};
use crate::domain::user::User;
use crate::domain::work::{Work, WorkStatus};
use crate::open_sea::metadata::Metadata;
use crate::{
    ddb, ethereum, internal_api, open_sea, AppError, AppResult, NFT_1155_ASSET_PATH_PREFIX,
    NFT_721_ASSET_PATH_PREFIX,
};
use bytes::Bytes;
use std::env;

pub struct Application {
    #[allow(dead_code)]
    me_id: String,
    work_dao: ddb::Dao<Work>,
    asset721_dao: ddb::Dao<Asset721>,
    asset1155_dao: ddb::Dao<Asset1155>,
    user_dao: ddb::Dao<User>,
    open_sea_cli: open_sea::Client,
    internal_api: internal_api::Client,
    ethereum_cli: ethereum::Client,
}

impl Application {
    pub async fn new(me_id: String) -> Self {
        let work_dao: ddb::Dao<Work> = ddb::Dao::new().await;
        let asset721_dao: ddb::Dao<Asset721> = ddb::Dao::new().await;
        let asset1155_dao: ddb::Dao<Asset1155> = ddb::Dao::new().await;
        let user_dao: ddb::Dao<User> = ddb::Dao::new().await;
        let open_sea_cli = open_sea::Client::new();
        let internal_api = internal_api::Client::new();
        let ethereum_cli = ethereum::Client::new();

        Self {
            me_id,
            work_dao,
            asset721_dao,
            asset1155_dao,
            user_dao,
            open_sea_cli,
            internal_api,
            ethereum_cli,
        }
    }

    pub async fn create_erc721(
        &self,
        id: String,
        gs_path: String,
        point: i32,
        level: i32,
    ) -> AppResult<()> {
        let user = self.user_dao.get(self.me_id.clone()).await?;
        let work = self.work_dao.get(id.clone()).await?;

        let urls = self
            .internal_api
            .get_signed_urls(vec![gs_path.clone()])
            .await?;
        let url = urls.first().unwrap();
        let bytes = reqwest::get(url).await?.bytes().await?;

        let s3_key = format!("{}/{}.png", NFT_721_ASSET_PATH_PREFIX, work.id.clone());
        let uploaded_url = upload_object(
            env::var("S3_USER_BUCKET").unwrap(),
            s3_key,
            bytes,
            "image/png".to_string(),
        )
        .await?;

        let metadata = Metadata::new(
            work.id.clone(),
            "create test nft from rust".to_string(),
            uploaded_url,
            point,
            level,
        );
        let metadata = serde_json::to_string(&metadata)?;

        let s3_key = format!(
            "{}/{}.metadata.json",
            NFT_721_ASSET_PATH_PREFIX,
            work.id.clone()
        );
        upload_object(
            env::var("S3_USER_BUCKET").unwrap(),
            s3_key,
            Bytes::from(metadata),
            "application/json".to_string(),
        )
        .await?;

        let name = self.ethereum_cli.get_erc721_nft_name().await?;
        let symbol = self.ethereum_cli.get_erc721_nft_symbol().await?;
        println!("{}, {}", name, symbol);

        self.ethereum_cli.mint_erc721(&user, work.id).await?;

        Ok(())
    }

    pub async fn create_erc1155(
        &self,
        id: String,
        gs_path: String,
        point: i32,
        level: i32,
        amount: u32,
    ) -> AppResult<()> {
        let user = self.user_dao.get(self.me_id.clone()).await?;
        let work = self.work_dao.get(id.clone()).await?;

        let urls = self
            .internal_api
            .get_signed_urls(vec![gs_path.clone()])
            .await?;
        let url = urls.first().unwrap();
        let bytes = reqwest::get(url).await?.bytes().await?;

        let s3_key = format!("{}/{}.png", NFT_1155_ASSET_PATH_PREFIX, work.id.clone());
        let uploaded_url = upload_object(
            env::var("S3_USER_BUCKET").unwrap(),
            s3_key,
            bytes,
            "image/png".to_string(),
        )
        .await?;

        let metadata = Metadata::new(
            work.id.clone(),
            "create test nft from rust".to_string(),
            uploaded_url,
            point,
            level,
        );
        let metadata = serde_json::to_string(&metadata)?;

        let s3_key = format!(
            "{}/{}.metadata.json",
            NFT_1155_ASSET_PATH_PREFIX,
            work.id.clone()
        );
        upload_object(
            env::var("S3_USER_BUCKET").unwrap(),
            s3_key,
            Bytes::from(metadata),
            "application/json".to_string(),
        )
        .await?;

        self.ethereum_cli
            .mint_erc1155(&user, work.id, amount)
            .await?;

        Ok(())
    }

    pub async fn sync_asset(&self) -> AppResult<()> {
        let contract_address =
            env::var("NFT_721_CONTRACT_ADDRESS").expect("should set contract address");
        let used_721_ids = self.ethereum_cli.get_erc721_used_names().await?;
        for work_id in used_721_ids {
            println!("sync work for erc721: {}", work_id);
            let work = self.work_dao.get(work_id.clone()).await;
            if let Err(err) = work {
                if err != AppError::NotFound {
                    return Err(err);
                }
                continue;
            }
            let mut work = work.ok().unwrap();

            let token_id = self.ethereum_cli.get_erc721_token_id_of(work_id).await?;
            let asset = self
                .open_sea_cli
                .get_asset(open_sea::api::get_asset::Input {
                    address: contract_address.clone(),
                    token_id: token_id.clone().to_string(),
                })
                .await?;

            work.status = WorkStatus::PublishNFT;

            let asset = Asset721::new(
                work.id.clone(),
                contract_address.clone(),
                token_id.clone().to_string(),
                asset.name,
                asset.description,
                asset.image_url,
                asset.image_preview_url,
                asset.permalink,
            );

            self.work_dao.put(&work).await?;
            self.asset721_dao.put(&asset).await?;
        }

        let contract_address =
            env::var("NFT_1155_CONTRACT_ADDRESS").expect("should set contract address");
        let used_1155_ids = self.ethereum_cli.get_erc1155_used_names().await?;
        for work_id in used_1155_ids {
            println!("sync work for erc1155: {}", work_id);
            let work = self.work_dao.get(work_id.clone()).await;
            if let Err(err) = work {
                if err != AppError::NotFound {
                    return Err(err);
                }
                continue;
            }
            let mut work = work.ok().unwrap();

            let token_id = self.ethereum_cli.get_erc1155_token_id_of(work_id).await?;
            let asset = self
                .open_sea_cli
                .get_asset(open_sea::api::get_asset::Input {
                    address: contract_address.clone(),
                    token_id: token_id.clone().to_string(),
                })
                .await?;

            work.status = WorkStatus::PublishNFT;

            let asset = Asset1155::new(
                work.id.clone(),
                contract_address.clone(),
                token_id.clone().to_string(),
                asset.name,
                asset.description,
                asset.image_url,
                asset.image_preview_url,
                asset.permalink,
            );

            self.work_dao.put(&work).await?;
            self.asset1155_dao.put(&asset).await?;
        }

        Ok(())
    }
}
