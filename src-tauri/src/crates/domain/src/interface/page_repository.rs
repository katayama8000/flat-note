use crate::aggregate::Page;
use crate::aggregate::value_object::{PageDescription, PageId, PageTitle};

pub trait PageRepository {
    fn find_all(&self) -> impl std::future::Future<Output = Result<Vec<Page>, String>> + Send;
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
}
