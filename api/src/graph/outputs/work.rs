use crate::graph::enums::WorkStatus;
use crate::graph::Context;
use app::domain;
use juniper::FieldResult;

#[derive(Debug, Clone)]
pub struct Work {
    data: domain::work::Work,
}

#[juniper::graphql_object(Context = Context)]
impl Work {
    fn id(&self) -> String {
        self.data.id.to_owned()
    }

    fn video_path(&self) -> String {
        self.data.video_path.to_owned()
    }

    fn status(&self) -> WorkStatus {
        WorkStatus::from(self.data.status.to_owned())
    }

    fn price(&self) -> i32 {
        self.data.price.to_owned()
    }

    async fn thumbnails(&self, context: &Context) -> FieldResult<Vec<Thumbnail>> {
        let thumbnails = context
            .thumbnail_by_work_loader
            .load(self.data.id.to_owned())
            .await?;
        Ok(thumbnails
            .iter()
            .map(|v| Thumbnail::from(v.to_owned()))
            .collect())
    }
}

impl From<domain::work::Work> for Work {
    fn from(data: domain::work::Work) -> Self {
        Self { data }
    }
}

#[derive(Debug, Clone)]
pub struct WorkEdge {
    pub node: Work,
}

#[juniper::graphql_object(Context = Context)]
impl WorkEdge {
    fn node(&self) -> Work {
        self.node.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct WorkConnection {
    pub edges: Vec<WorkEdge>,
    pub next_key: Option<String>,
    pub total_count: Option<i32>,
}

#[juniper::graphql_object(Context = Context)]
impl WorkConnection {
    fn edges(&self) -> Vec<WorkEdge> {
        self.edges.to_owned()
    }

    fn next_key(&self) -> Option<String> {
        self.next_key.to_owned()
    }

    fn total_count(&self) -> Option<i32> {
        self.total_count.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Thumbnail {
    data: domain::work::Thumbnail,
}

#[juniper::graphql_object(Context = Context)]
impl Thumbnail {
    fn id(&self) -> String {
        self.data.id.to_owned()
    }

    fn image_path(&self) -> String {
        self.data.image_path.to_owned()
    }

    fn order(&self) -> i32 {
        self.data.order.to_owned()
    }
}

impl From<domain::work::Thumbnail> for Thumbnail {
    fn from(data: domain::work::Thumbnail) -> Self {
        Self { data }
    }
}
