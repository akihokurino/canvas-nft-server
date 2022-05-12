#[derive(Debug, GraphQLInputObject)]
pub struct ImportWorkInput {
    pub file_name: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct ImportThumbnailInput {
    pub file_name: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct CreateNft721Input {
    pub work_id: String,
    pub gs_path: String,
    pub point: i32,
    pub level: i32,
}

#[derive(Debug, GraphQLInputObject)]
pub struct CreateNft1155Input {
    pub work_id: String,
    pub gs_path: String,
    pub point: i32,
    pub level: i32,
    pub amount: i32,
}

#[derive(Debug, GraphQLInputObject)]
pub struct RegisterUserInput {
    pub email: String,
    pub password: String,
    pub wallet_address: String,
    pub wallet_secret: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct SellNftInput {
    pub work_id: String,
    pub ether: f64,
}
