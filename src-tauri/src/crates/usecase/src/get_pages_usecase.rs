use domain::aggregate::value_object::{SortBy, UserId};
use domain::{Page, PageRepository};

pub struct GetPagesUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> GetPagesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &str, sort_by: &SortBy) -> Result<Vec<Page>, String> {
        self.repository.find_all(&UserId::new(user_id), sort_by).await
    }
}
