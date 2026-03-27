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
        let page = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| format!("Page not found: {id}"))?;

        let updated_page = page.with_title(title);
        self.repository.save(&updated_page).await
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
        let page = Page::create(id, title, "");
        self.repository.create(&page).await
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
        let page = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| format!("Page not found: {id}"))?;

        let updated_page = page.with_description(description);
        self.repository.save(&updated_page).await
    }
}
