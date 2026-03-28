use domain::{Page, PageRepository};

pub struct InMemoryPageRepository;

impl PageRepository for InMemoryPageRepository {
    async fn find_all(&self) -> Result<Vec<Page>, String> {
        Ok(vec![
            Page::reconstruct(
                "1",
                "Page returned from Rust",
                "This page was generated in Rust",
                "1710000000",
                "1710000000",
            ),
            Page::reconstruct(
                "2",
                "Second page",
                "Automatically serialized to JSON via Serialize derive",
                "1710001000",
                "1710001000",
            ),
        ])
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Page>, String> {
        let pages = self.find_all().await?;
        Ok(pages.into_iter().find(|p| p.id == id))
    }

    async fn save(&self, page: &Page) -> Result<(), String> {
        let _ = page;
        Ok(())
    }

    async fn create(&self, page: &Page) -> Result<Page, String> {
        Ok(page.clone())
    }

    async fn update_title_direct(&self, _id: &str, _title: &str) -> Result<(), String> {
        Ok(())
    }

    async fn update_description_direct(&self, _id: &str, _description: &str) -> Result<(), String> {
        Ok(())
    }

    async fn count(&self) -> Result<u64, String> {
        let pages = self.find_all().await?;
        Ok(pages.len() as u64)
    }
}

pub struct LibSqlPageRepository {
    url: String,
}

impl LibSqlPageRepository {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    async fn connect(&self) -> Result<libsql::Connection, String> {
        let db = libsql::Builder::new_remote(self.url.clone(), "".to_string())
            .build()
            .await
            .map_err(|e| e.to_string())?;

        let conn = db.connect().map_err(|e| e.to_string())?;
        Ok(conn)
    }

}

impl PageRepository for LibSqlPageRepository {
    async fn find_all(&self) -> Result<Vec<Page>, String> {
        let conn = self.connect().await?;

        let mut rows = conn
            .query(
                "SELECT id, title, description, created_at, updated_at FROM pages",
                (),
            )
            .await
            .map_err(|e| e.to_string())?;

        let mut pages = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let id = row.get::<String>(0).map_err(|e| e.to_string())?;
            let title = row.get::<String>(1).map_err(|e| e.to_string())?;
            let description = row.get::<String>(2).map_err(|e| e.to_string())?;
            let created_at = row.get::<String>(3).map_err(|e| e.to_string())?;
            let updated_at = row.get::<String>(4).map_err(|e| e.to_string())?;
            pages.push(Page::reconstruct(id, title, description, created_at, updated_at));
        }

        Ok(pages)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Page>, String> {
        let conn = self.connect().await?;

        let mut rows = conn
            .query(
                "SELECT id, title, description, created_at, updated_at FROM pages WHERE id = ?1",
                libsql::params![id],
            )
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let page = Page::reconstruct(
                row.get::<String>(0).map_err(|e| e.to_string())?,
                row.get::<String>(1).map_err(|e| e.to_string())?,
                row.get::<String>(2).map_err(|e| e.to_string())?,
                row.get::<String>(3).map_err(|e| e.to_string())?,
                row.get::<String>(4).map_err(|e| e.to_string())?,
            );
            Ok(Some(page))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, page: &Page) -> Result<(), String> {
        let conn = self.connect().await?;

        conn.execute(
            "UPDATE pages SET title = ?1, description = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3",
            libsql::params![
                page.title.as_str(),
                page.description.as_str(),
                page.id.as_str()
            ],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn create(&self, page: &Page) -> Result<Page, String> {
        let conn = self.connect().await?;

        conn.execute(
            "INSERT INTO pages (id, title, description) VALUES (?1, ?2, ?3)",
            libsql::params![
                page.id.as_str(),
                page.title.as_str(),
                page.description.as_str()
            ],
        )
        .await
        .map_err(|e| e.to_string())?;

        self.find_by_id(page.id.as_str())
            .await?
            .ok_or_else(|| format!("Page not found after creation: {}", page.id))
    }

    async fn update_title_direct(&self, id: &str, title: &str) -> Result<(), String> {
        let conn = self.connect().await?;

        conn.execute(
            "UPDATE pages SET title = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            libsql::params![title, id],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_description_direct(&self, id: &str, description: &str) -> Result<(), String> {
        let conn = self.connect().await?;

        conn.execute(
            "UPDATE pages SET description = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            libsql::params![description, id],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn count(&self) -> Result<u64, String> {
        let conn = self.connect().await?;

        let mut rows = conn
            .query("SELECT COUNT(*) FROM pages", ())
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let count = row.get::<u64>(0).map_err(|e| e.to_string())?;
            Ok(count)
        } else {
            Ok(0)
        }
    }
}
