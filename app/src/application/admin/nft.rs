use crate::aws::s3::upload_object;
use crate::aws::{lambda, sns};
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

    pub async fn prepare_erc721(
        &self,
        work_id: String,
        gs_path: String,
        point: i32,
        level: i32,
    ) -> AppResult<()> {
        let work = self.work_dao.get(work_id.clone()).await?;

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

        let asset = Asset721::new(work.id.clone());
        self.asset721_dao.put(&asset).await?;

        let payload = sns::MintNft721Payload {
            executor_id: self.me_id.clone(),
            work_id: work.id.clone(),
        };

        sns::publish(sns::Task::MintNFT721(payload)).await?;

        Ok(())
    }

    pub async fn mint_erc721(&self, work_id: String) -> AppResult<()> {
        let user = self.user_dao.get(self.me_id.clone()).await?;
        let mut work = self.work_dao.get(work_id.clone()).await?;

        self.ethereum_cli
            .mint_erc721(&user, work_id.clone())
            .await?;

        self.save_asset721(work_id.clone()).await?;

        work.status = WorkStatus::PublishNFT;
        self.work_dao.put(&work).await?;

        Ok(())
    }

    pub async fn prepare_erc1155(
        &self,
        work_id: String,
        gs_path: String,
        point: i32,
        level: i32,
        amount: u32,
    ) -> AppResult<()> {
        let work = self.work_dao.get(work_id.clone()).await?;

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

        let asset = Asset1155::new(work.id.clone());
        self.asset1155_dao.put(&asset).await?;

        let payload = sns::MintNft1155Payload {
            executor_id: self.me_id.clone(),
            work_id: work.id.clone(),
            amount: amount.to_owned(),
        };

        sns::publish(sns::Task::MintNFT1155(payload)).await?;

        Ok(())
    }

    pub async fn mint_erc1155(&self, work_id: String, amount: u32) -> AppResult<()> {
        let user = self.user_dao.get(self.me_id.clone()).await?;
        let mut work = self.work_dao.get(work_id.clone()).await?;

        self.ethereum_cli
            .mint_erc1155(&user, work_id.clone(), amount)
            .await?;

        self.save_asset1155(work_id.clone()).await?;

        work.status = WorkStatus::PublishNFT;
        self.work_dao.put(&work).await?;

        Ok(())
    }

    async fn save_asset721(&self, work_id: String) -> AppResult<()> {
        let contract_address =
            env::var("NFT_721_CONTRACT_ADDRESS").expect("should set contract address");
        let token_id = self
            .ethereum_cli
            .get_erc721_token_id_of(work_id.clone())
            .await?;
        let asset = self
            .open_sea_cli
            .get_asset(open_sea::api::get_asset::Input {
                address: contract_address.clone(),
                token_id: token_id.clone().to_string(),
            })
            .await?;

        let mut updated = self.asset721_dao.get(work_id.clone()).await?;
        updated.published(
            contract_address.clone(),
            token_id.clone().to_string(),
            asset.name,
            asset.description,
            asset.image_url,
            asset.image_preview_url,
            asset.permalink,
        );

        self.asset721_dao.put(&updated).await?;

        Ok(())
    }

    async fn save_asset1155(&self, work_id: String) -> AppResult<()> {
        let contract_address =
            env::var("NFT_1155_CONTRACT_ADDRESS").expect("should set contract address");
        let token_id = self
            .ethereum_cli
            .get_erc1155_token_id_of(work_id.clone())
            .await?;
        let asset = self
            .open_sea_cli
            .get_asset(open_sea::api::get_asset::Input {
                address: contract_address.clone(),
                token_id: token_id.clone().to_string(),
            })
            .await?;

        let mut updated = self.asset1155_dao.get(work_id.clone()).await?;
        updated.published(
            contract_address.clone(),
            token_id.clone().to_string(),
            asset.name,
            asset.description,
            asset.image_url,
            asset.image_preview_url,
            asset.permalink,
        );

        self.asset1155_dao.put(&updated).await?;

        Ok(())
    }

    pub async fn sync_asset(&self) -> AppResult<()> {
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

            self.save_asset721(work.id.clone()).await?;

            work.status = WorkStatus::PublishNFT;
            self.work_dao.put(&work).await?;
        }

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

            self.save_asset1155(work.id.clone()).await?;

            work.status = WorkStatus::PublishNFT;
            self.work_dao.put(&work).await?;
        }

        Ok(())
    }

    pub async fn sell_721(&self, work_id: String, ether: f64) -> AppResult<()> {
        let user = self.user_dao.get(self.me_id.clone()).await?;

        let contract_address =
            env::var("NFT_721_CONTRACT_ADDRESS").expect("should set contract address");
        let token_id = self
            .ethereum_cli
            .get_erc721_token_id_of(work_id.clone())
            .await?;

        lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::sell(
            user,
            contract_address,
            token_id.to_string(),
            ether,
        ))
        .await?;

        Ok(())
    }

    pub async fn sell_1155(&self, work_id: String, ether: f64) -> AppResult<()> {
        let user = self.user_dao.get(self.me_id.clone()).await?;

        let contract_address =
            env::var("NFT_1155_CONTRACT_ADDRESS").expect("should set contract address");
        let token_id = self
            .ethereum_cli
            .get_erc1155_token_id_of(work_id.clone())
            .await?;

        lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::sell(
            user,
            contract_address,
            token_id.to_string(),
            ether,
        ))
        .await?;

        Ok(())
    }

    pub async fn transfer_721(&self, work_id: String, to_address: String) -> AppResult<()> {
        let user = self.user_dao.get(self.me_id.clone()).await?;

        let contract_address =
            env::var("NFT_721_CONTRACT_ADDRESS").expect("should set contract address");
        let token_id = self
            .ethereum_cli
            .get_erc721_token_id_of(work_id.clone())
            .await?;

        lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::transfer_721(
            user,
            contract_address,
            token_id.to_string(),
            to_address,
        ))
        .await?;

        Ok(())
    }

    pub async fn transfer_1155(&self, work_id: String, to_address: String) -> AppResult<()> {
        let user = self.user_dao.get(self.me_id.clone()).await?;

        let contract_address =
            env::var("NFT_1155_CONTRACT_ADDRESS").expect("should set contract address");
        let token_id = self
            .ethereum_cli
            .get_erc1155_token_id_of(work_id.clone())
            .await?;

        lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::transfer_1155(
            user,
            contract_address,
            token_id.to_string(),
            to_address,
        ))
        .await?;

        Ok(())
    }

    pub async fn is_own_721(&self, work_id: String) -> AppResult<bool> {
        let contract_address =
            env::var("NFT_721_CONTRACT_ADDRESS").expect("should set contract address");
        let token_id = self
            .ethereum_cli
            .get_erc721_token_id_of(work_id.clone())
            .await?;

        if token_id == 0 {
            return Ok(false);
        }

        self.is_own(contract_address, token_id).await
    }

    pub async fn is_own_1155(&self, work_id: String) -> AppResult<bool> {
        let contract_address =
            env::var("NFT_1155_CONTRACT_ADDRESS").expect("should set contract address");
        let token_id = self
            .ethereum_cli
            .get_erc1155_token_id_of(work_id.clone())
            .await?;

        if token_id == 0 {
            return Ok(false);
        }

        self.is_own(contract_address, token_id).await
    }

    async fn is_own(&self, contract_address: String, token_id: u128) -> AppResult<bool> {
        let user = self.user_dao.get(self.me_id.clone()).await?;

        let asset = self
            .open_sea_cli
            .get_asset(open_sea::api::get_asset::Input {
                address: contract_address.clone(),
                token_id: token_id.clone().to_string(),
            })
            .await?;

        for item in asset.top_ownerships {
            if self
                .ethereum_cli
                .equal_address(item.owner.address, user.wallet_address.clone())
            {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
