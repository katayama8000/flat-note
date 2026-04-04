use domain::aggregate::value_object::{PageDescription, PageId, PageTitle, SortBy, UserId};
use domain::{Page, PageRepository};

pub struct GetPagesUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> GetPagesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &str, sort_by: &SortBy) -> Result<Vec<Page>, String> {
        self.repository
            .find_all(&UserId::new(user_id), sort_by)
            .await
    }
}

pub struct GetPageUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> GetPageUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &str, id: &str) -> Result<Option<Page>, String> {
        self.repository
            .find_by_id(&UserId::new(user_id), &PageId::new(id))
            .await
    }
}

pub struct UpdateTitleUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> UpdateTitleUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &str, id: &str, title: &str) -> Result<(), String> {
        let owner_id = UserId::new(user_id);
        let page = self
            .repository
            .find_by_id(&owner_id, &PageId::new(id))
            .await?
            .ok_or_else(|| format!("Page not found: {id}"))?;

        let updated_page = page.with_title(title);
        self.repository.save(&owner_id, &updated_page).await
    }

    pub async fn execute_direct(&self, user_id: &str, id: &str, title: &str) -> Result<(), String> {
        self.repository
            .update_title_direct(
                &UserId::new(user_id),
                &PageId::new(id),
                &PageTitle::new(title),
            )
            .await
    }
}

pub struct CreatePageUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> CreatePageUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &str, id: &str, title: &str) -> Result<Page, String> {
        let page = Page::create(id, user_id, title, "");
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

    pub async fn execute(&self, user_id: &str, id: &str, description: &str) -> Result<(), String> {
        let owner_id = UserId::new(user_id);
        let page = self
            .repository
            .find_by_id(&owner_id, &PageId::new(id))
            .await?
            .ok_or_else(|| format!("Page not found: {id}"))?;

        let updated_page = page.with_description(description);
        self.repository.save(&owner_id, &updated_page).await
    }

    pub async fn execute_direct(
        &self,
        user_id: &str,
        id: &str,
        description: &str,
    ) -> Result<(), String> {
        self.repository
            .update_description_direct(
                &UserId::new(user_id),
                &PageId::new(id),
                &PageDescription::new(description),
            )
            .await
    }
}

pub struct CountPagesUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> CountPagesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &str) -> Result<u64, String> {
        self.repository.count(&UserId::new(user_id)).await
    }
}
