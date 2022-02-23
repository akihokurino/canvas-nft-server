use crate::graph::enums::WorkStatus;

#[derive(Debug, GraphQLInputObject)]
pub struct CreateWorkInput {
    pub file_name: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct CreateThumbnailInput {
    pub file_name: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct UpdateWorkStatusInput {
    pub id: String,
    pub status: WorkStatus,
}

#[derive(Debug, GraphQLInputObject)]
pub struct CreateNftInput {
    pub work_id: String,
    pub thumbnail_url: String,
    pub point: i32,
    pub level: i32,
}

#[derive(Debug, GraphQLInputObject)]
pub struct BindNftToWorkInput {
    pub work_id: String,
    pub contract_address: String,
    pub token_id: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct RegisterUserInput {
    pub address: String,
}
