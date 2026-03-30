use domain::aggregate::value_object::{PageId, TokenName};
use domain::PageRepository;
use regex::Regex;

pub struct SyncTokensUseCase<R: PageRepository> {
    repository: R,
}

impl<R: PageRepository> SyncTokensUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, page_id: &PageId, content: &str) -> Result<(), String> {
        let token_names = self.extract_tokens(content);
        let mut token_ids = Vec::new();
        for name in token_names {
            let token = self.repository.find_or_create_token(&name).await?;
            token_ids.push(token.id().clone());
        }
        self.repository.sync_page_tokens(page_id, &token_ids).await
    }

    fn extract_tokens(&self, content: &str) -> Vec<TokenName> {
        let re = Regex::new(r"#(?P<tag>[a-zA-Z0-9-_]+)|\[(?P<link>[^\]]+)\]").unwrap();
        re.captures_iter(content)
            .map(|cap| {
                if let Some(tag) = cap.name("tag") {
                    TokenName::new(tag.as_str())
                } else if let Some(link) = cap.name("link") {
                    TokenName::new(link.as_str())
                } else {
                    unreachable!()
                }
            })
            .collect()
    }
}
