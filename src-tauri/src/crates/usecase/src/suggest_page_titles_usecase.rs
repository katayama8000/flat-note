use domain::PageRepository;
use domain::aggregate::value_object::UserId;

pub struct SuggestPageTitlesUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> SuggestPageTitlesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        user_id: &str,
        keyword: &str,
        limit: u32,
    ) -> Result<Vec<String>, String> {
        self.repository
            .suggest_titles(&UserId::new(user_id), keyword, limit)
            .await
    }
}
