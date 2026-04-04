use domain::aggregate::value_object::{PageId, PageTitle, UserId};
use domain::PageRepository;

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
