use domain::aggregate::value_object::{PageId, UserId};
use domain::{Page, PageRepository};

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
