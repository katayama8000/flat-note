use crate::aggregate::{Page, Token};
use crate::aggregate::value_object::{PageDescription, PageId, PageTitle, SortBy, TokenId, TokenName};

pub trait PageRepository {
    fn find_all(
        &self,
        sort_by: &SortBy,
    ) -> impl std::future::Future<Output = Result<Vec<Page>, String>> + Send;
    fn find_by_id(
        &self,
        id: &PageId,
    ) -> impl std::future::Future<Output = Result<Option<Page>, String>> + Send;
    fn save(&self, page: &Page) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn create(&self, page: &Page)
    -> impl std::future::Future<Output = Result<Page, String>> + Send;
    fn update_title_direct(
        &self,
        id: &PageId,
        title: &PageTitle,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn update_description_direct(
        &self,
        id: &PageId,
        description: &PageDescription,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn count(&self) -> impl std::future::Future<Output = Result<u64, String>> + Send;

    // New methods for token management
    fn find_or_create_token(
        &self,
        name: &TokenName,
    ) -> impl std::future::Future<Output = Result<Token, String>> + Send;
    fn find_tokens_by_page_id(
        &self,
        page_id: &PageId,
    ) -> impl std::future::Future<Output = Result<Vec<Token>, String>> + Send;
    fn sync_page_tokens(
        &self,
        page_id: &PageId,
        token_ids: &Vec<TokenId>,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn find_related_pages(
        &self,
        page_id: &PageId,
    ) -> impl std::future::Future<Output = Result<Vec<Page>, String>> + Send;
}
