use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl Page {
    pub fn create(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
        }
    }

    pub fn with_title(&self, title: impl Into<String>) -> Self {
        Self {
            id: self.id.clone(),
            title: title.into(),
            description: self.description.clone(),
        }
    }

    pub fn with_description(&self, description: impl Into<String>) -> Self {
        Self {
            id: self.id.clone(),
            title: self.title.clone(),
            description: description.into(),
        }
    }

    pub fn with_updates(&self, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: self.id.clone(),
            title: title.into(),
            description: description.into(),
        }
    }

    // reconstruct is used in repository layer
    pub fn reconstruct(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self::create(id, title, description)
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
}
