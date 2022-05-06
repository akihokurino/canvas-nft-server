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
    pub thumbnail_url: String,
    pub point: i32,
    pub level: i32,
}

#[derive(Debug, GraphQLInputObject)]
pub struct CreateNft1155Input {
    pub work_id: String,
    pub thumbnail_url: String,
    pub point: i32,
    pub level: i32,
    pub amount: i32,
}

#[derive(Debug, GraphQLInputObject)]
pub struct BindNftToWorkInput {
    pub work_id: String,
    pub contract_address: String,
    pub token_id: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct RegisterUserInput {
    pub email: String,
    pub password: String,
    pub wallet_address: String,
    pub wallet_secret: String,
}
