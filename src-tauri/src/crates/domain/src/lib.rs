use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub description: String,
}

pub trait PageRepository {
    fn find_all(&self) -> impl std::future::Future<Output = Result<Vec<Page>, String>> + Send;
}
