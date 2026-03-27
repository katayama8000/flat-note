use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub description: String,
}

pub trait PageRepository {
    fn find_all(&self) -> impl std::future::Future<Output = Result<Vec<Page>, String>> + Send;
    fn find_by_id(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Page>, String>> + Send;
    fn update_description(
        &self,
        id: &str,
        description: &str,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn update_title(
        &self,
        id: &str,
        title: &str,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn create(
        &self,
        id: &str,
        title: &str,
    ) -> impl std::future::Future<Output = Result<Page, String>> + Send;
}
