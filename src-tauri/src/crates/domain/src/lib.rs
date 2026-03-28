use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Page {
    pub fn create(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let now = Self::current_unix_timestamp();
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn with_title(&self, title: impl Into<String>) -> Self {
        Self {
            id: self.id.clone(),
            title: title.into(),
            description: self.description.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }

    pub fn with_description(&self, description: impl Into<String>) -> Self {
        Self {
            id: self.id.clone(),
            title: self.title.clone(),
            description: description.into(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }

    pub fn with_updates(&self, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: self.id.clone(),
            title: title.into(),
            description: description.into(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }

    // reconstruct is used in repository layer
    pub fn reconstruct(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        created_at: impl Into<String>,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            created_at: created_at.into(),
            updated_at: updated_at.into(),
        }
    }

    fn current_unix_timestamp() -> String {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        seconds.to_string()
    }
}

pub trait PageRepository {
    fn find_all(&self) -> impl std::future::Future<Output = Result<Vec<Page>, String>> + Send;
    fn find_by_id(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Page>, String>> + Send;
    fn save(&self, page: &Page) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn create(&self, page: &Page)
    -> impl std::future::Future<Output = Result<Page, String>> + Send;
    fn update_title_direct(
        &self,
        id: &str,
        title: &str,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn update_description_direct(
        &self,
        id: &str,
        description: &str,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn count(&self) -> impl std::future::Future<Output = Result<u64, String>> + Send;
}
