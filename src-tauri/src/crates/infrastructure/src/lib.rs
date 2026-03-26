use domain::{Page, PageRepository};

pub struct InMemoryPageRepository;

impl PageRepository for InMemoryPageRepository {
    async fn find_all(&self) -> Result<Vec<Page>, String> {
        Ok(vec![
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
        ])
    }
}

pub struct LibSqlPageRepository {
    url: String,
}

impl LibSqlPageRepository {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

impl PageRepository for LibSqlPageRepository {
    async fn find_all(&self) -> Result<Vec<Page>, String> {
        let db = libsql::Builder::new_remote(self.url.clone(), "".to_string())
            .build()
            .await
            .map_err(|e| e.to_string())?;

        let conn = db.connect().map_err(|e| e.to_string())?;

        let mut rows = conn
            .query("SELECT id, title, description FROM pages", ())
            .await
            .map_err(|e| e.to_string())?;

        let mut pages = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            pages.push(Page {
                id: row.get::<String>(0).map_err(|e| e.to_string())?,
                title: row.get::<String>(1).map_err(|e| e.to_string())?,
                description: row.get::<String>(2).map_err(|e| e.to_string())?,
            });
        }

        Ok(pages)
    }
}
