use domain::{Page, PageRepository};

pub struct GetPagesUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> GetPagesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Page>, String> {
        self.repository.find_all().await
    }
}

pub struct GetPageUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> GetPageUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: &str) -> Result<Option<Page>, String> {
        self.repository.find_by_id(id).await
    }
}

pub struct UpdateTitleUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> UpdateTitleUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: &str, title: &str) -> Result<(), String> {
        self.repository.update_title(id, title).await
    }
}

pub struct CreatePageUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> CreatePageUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: &str, title: &str) -> Result<Page, String> {
        self.repository.create(id, title).await
    }
}

pub struct UpdatePageUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> UpdatePageUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: &str, description: &str) -> Result<(), String> {
        self.repository.update_description(id, description).await
    }
}
