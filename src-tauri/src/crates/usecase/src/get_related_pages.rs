use domain::aggregate::value_object::PageId;
use domain::{Page, PageRepository};

pub struct GetRelatedPagesUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> GetRelatedPagesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, page_id: &PageId) -> Result<Vec<Page>, String> {
        self.repository.find_related_pages(page_id).await
    }
}
