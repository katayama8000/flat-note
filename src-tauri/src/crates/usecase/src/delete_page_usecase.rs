use domain::PageRepository;
use domain::aggregate::value_object::{PageId, UserId};

pub struct DeletePageUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> DeletePageUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &str, id: &str) -> Result<(), String> {
        let owner_id = UserId::new(user_id);
        let page_id = PageId::new(id);

        self.repository
            .find_by_id(&owner_id, &page_id)
            .await?
            .ok_or_else(|| format!("Page not found: {id}"))?;

        self.repository.delete(&owner_id, &page_id).await
    }
}
