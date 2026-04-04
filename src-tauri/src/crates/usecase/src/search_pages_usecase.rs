use domain::aggregate::value_object::{SortBy, UserId};
use domain::{Page, PageRepository};

pub struct SearchPagesUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> SearchPagesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        user_id: &str,
        keyword: &str,
        sort_by: &SortBy,
        limit: u32,
    ) -> Result<Vec<Page>, String> {
        self.repository
            .search(&UserId::new(user_id), keyword, sort_by, limit)
            .await
    }
}
