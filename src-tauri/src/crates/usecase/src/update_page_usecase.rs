use domain::PageRepository;
use domain::aggregate::value_object::{PageDescription, PageId, UserId};

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
