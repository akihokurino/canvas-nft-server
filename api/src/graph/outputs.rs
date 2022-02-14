pub mod nft;
pub mod user;
pub mod work;

#[derive(Debug, GraphQLObject)]
pub struct PreSignUploadUrl {
    pub url: String,
    pub file_name: String,
}
