use crate::aggregate::Page;
use crate::aggregate::value_object::{PageDescription, PageId, PageTitle, SortBy, UserId};

pub trait PageRepository {
    fn find_all(
        &self,
        owner_id: &UserId,
        sort_by: &SortBy,
    ) -> impl std::future::Future<Output = Result<Vec<Page>, String>> + Send;
    fn find_by_id(
        &self,
        owner_id: &UserId,
        id: &PageId,
    ) -> impl std::future::Future<Output = Result<Option<Page>, String>> + Send;
    fn save(
        &self,
        owner_id: &UserId,
        page: &Page,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn create(&self, page: &Page)
    -> impl std::future::Future<Output = Result<Page, String>> + Send;
    fn update_title_direct(
        &self,
        owner_id: &UserId,
        id: &PageId,
        title: &PageTitle,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn update_description_direct(
        &self,
        owner_id: &UserId,
        id: &PageId,
        description: &PageDescription,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn count(
        &self,
        owner_id: &UserId,
    ) -> impl std::future::Future<Output = Result<u64, String>> + Send;
    fn search(
        &self,
        owner_id: &UserId,
        keyword: &str,
        sort_by: &SortBy,
        limit: u32,
    ) -> impl std::future::Future<Output = Result<Vec<Page>, String>> + Send;
    fn suggest_titles(
        &self,
        owner_id: &UserId,
        keyword: &str,
        limit: u32,
    ) -> impl std::future::Future<Output = Result<Vec<String>, String>> + Send;
}
