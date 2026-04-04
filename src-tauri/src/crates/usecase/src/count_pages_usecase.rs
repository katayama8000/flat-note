use domain::aggregate::value_object::UserId;
use domain::PageRepository;

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
