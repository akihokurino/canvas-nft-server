use crate::domain::nft::NFT;
use crate::domain::work::Work;
use crate::{ddb, open_sea, AppResult};

pub struct Application {
    #[allow(dead_code)]
    me_id: String,
    work_dao: ddb::Dao<Work>,
    nft_dao: ddb::Dao<NFT>,
    open_sea_cli: open_sea::Client,
}

impl Application {
    pub async fn new(me_id: String) -> Self {
        let work_dao: ddb::Dao<Work> = ddb::Dao::new().await;
        let nft_dao: ddb::Dao<NFT> = ddb::Dao::new().await;
        let open_sea_client = open_sea::Client::new();

        Self {
            me_id,
            work_dao,
            nft_dao,
            open_sea_cli: open_sea_client,
        }
    }

    pub async fn bind_work(
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

        // TODO: usd -> jpy
        work.update_price(0)?;

        let payment_tokens: Vec<&open_sea::api::get_asset::PaymentToken> = asset
            .collection
            .payment_tokens
            .iter()
            .filter(|v| v.symbol.to_owned().unwrap_or_default() == "ETH")
            .collect();
        let payment_token = payment_tokens.first();

        let nft = NFT::new(
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
