pub mod asset;
pub mod user;
pub mod work;

#[derive(Debug, GraphQLObject)]
pub struct PreSignUploadUrl {
    pub url: String,
    pub file_name: String,
}

#[derive(Debug, GraphQLObject)]
pub struct OwnNft {
    pub erc721: bool,
    pub erc1155: bool,
}
