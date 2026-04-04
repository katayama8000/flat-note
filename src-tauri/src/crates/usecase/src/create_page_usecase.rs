use domain::{Page, PageRepository};

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
