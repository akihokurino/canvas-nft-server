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
pub struct UpdateWorkPriceInput {
    pub id: String,
    pub price: i32,
}
