#[derive(Debug, GraphQLInputObject)]
pub struct RegisterUserInput {
    pub email: String,
    pub password: String,
    pub wallet_address: String,
    pub wallet_secret: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct ImportWorkInput {
    pub file_name: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct ImportThumbnailInput {
    pub file_name: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct CreateERC721Input {
    pub work_id: String,
    pub gs_path: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct CreateERC1155Input {
    pub work_id: String,
    pub gs_path: String,
    pub amount: i32,
}

#[derive(Debug, GraphQLInputObject)]
pub struct SellERC721Input {
    pub work_id: String,
    pub ether: f64,
}

#[derive(Debug, GraphQLInputObject)]
pub struct SellERC1155Input {
    pub work_id: String,
    pub ether: f64,
    pub amount: i32,
}

#[derive(Debug, GraphQLInputObject)]
pub struct TransferERC721Input {
    pub work_id: String,
    pub to_address: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct TransferERC1155Input {
    pub work_id: String,
    pub to_address: String,
    pub amount: i32,
}
