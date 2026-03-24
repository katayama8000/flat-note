use domain::{Page, PageRepository};

pub struct InMemoryPageRepository;

impl PageRepository for InMemoryPageRepository {
    fn find_all(&self) -> Vec<Page> {
        vec![
            Page {
                id: "1".to_string(),
                title: "Page returned from Rust".to_string(),
                description: "This page was generated in Rust".to_string(),
            },
            Page {
                id: "2".to_string(),
                title: "Second page".to_string(),
                description: "Automatically serialized to JSON via Serialize derive".to_string(),
            },
        ]
    }
}
