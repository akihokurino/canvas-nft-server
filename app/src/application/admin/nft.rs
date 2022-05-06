use crate::aws::s3::upload_object;
use crate::domain::asset::Asset;
use crate::domain::user::User;
use crate::domain::work::Work;
use crate::open_sea::metadata::Metadata;
use crate::{
    ddb, ethereum, internal_api, open_sea, AppResult, NFT_1155_ASSET_PATH_PREFIX,
    NFT_721_ASSET_PATH_PREFIX,
};
use bytes::Bytes;
use std::env;

pub struct Application {
    #[allow(dead_code)]
    me_id: String,
    work_dao: ddb::Dao<Work>,
    nft_dao: ddb::Dao<Asset>,
    user_dao: ddb::Dao<User>,
    open_sea_cli: open_sea::Client,
    internal_api: internal_api::Client,
    ethereum_cli: ethereum::Client,
}

impl Application {
    pub async fn new(me_id: String) -> Self {
        let work_dao: ddb::Dao<Work> = ddb::Dao::new().await;
        let nft_dao: ddb::Dao<Asset> = ddb::Dao::new().await;
        let user_dao: ddb::Dao<User> = ddb::Dao::new().await;
        let open_sea_cli = open_sea::Client::new();
        let internal_api = internal_api::Client::new();
        let ethereum_cli = ethereum::Client::new();

        Self {
            me_id,
            work_dao,
            nft_dao,
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

    pub async fn sync_status(
        &self,
        id: String,
        contract_address: String,
        token_id: String,
    ) -> AppResult<()> {
        let mut work = self.work_dao.get(id.clone()).await?;

        let asset = self
            .open_sea_cli
            .get_asset(open_sea::api::get_asset::Input {
                address: contract_address.clone(),
                token_id: token_id.clone(),
            })
            .await?;

        let payment_tokens: Vec<&open_sea::api::get_asset::PaymentToken> = asset
            .collection
            .payment_tokens
            .iter()
            .filter(|v| v.symbol.to_owned().unwrap_or_default() == "ETH")
            .collect();
        let payment_token = payment_tokens.first();

        let nft = Asset::new(
            work.id.clone(),
            contract_address,
            token_id,
            asset.name,
            asset.description,
            asset.image_url,
            asset.image_preview_url,
            asset.permalink,
            payment_token.map_or(0.0, |v| v.usd_price.unwrap_or_default()),
            payment_token.map_or(0.0, |v| v.eth_price.unwrap_or_default()),
        );

        self.work_dao.put(&work).await?;
        self.nft_dao.put(&nft).await?;

        Ok(())
    }
}
