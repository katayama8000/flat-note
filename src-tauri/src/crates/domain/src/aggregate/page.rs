use super::value_object::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    id: PageId,
    title: PageTitle,
    description: PageDescription,
    #[serde(rename = "createdAt")]
    created_at: CreatedAt,
    #[serde(rename = "updatedAt")]
    updated_at: UpdatedAt,
}

impl Page {
    pub fn create(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let now = CreatedAt::now();
        Self {
            id: PageId::new(id),
            title: PageTitle::new(title),
            description: PageDescription::new(description),
            created_at: now.clone(),
            updated_at: UpdatedAt::new(now.value()),
        }
    }

    pub fn reconstruct(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        created_at: impl Into<String>,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            id: PageId::new(id),
            title: PageTitle::new(title),
            description: PageDescription::new(description),
            created_at: CreatedAt::new(created_at),
            updated_at: UpdatedAt::new(updated_at),
        }
    }

    pub fn with_title(&self, title: impl Into<String>) -> Self {
        Self {
            id: self.id.clone(),
            title: PageTitle::new(title),
            description: self.description.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }

    pub fn with_description(&self, description: impl Into<String>) -> Self {
        Self {
            id: self.id.clone(),
            title: self.title.clone(),
            description: PageDescription::new(description),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }

    pub fn with_updates(&self, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: self.id.clone(),
            title: PageTitle::new(title),
            description: PageDescription::new(description),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }

    // Getters
    pub fn id(&self) -> &PageId {
        &self.id
    }

    pub fn title(&self) -> &PageTitle {
        &self.title
    }

    pub fn description(&self) -> &PageDescription {
        &self.description
    }

    pub fn created_at(&self) -> &CreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &UpdatedAt {
        &self.updated_at
    }
}
