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

    async fn find_by_id(&self, id: &str) -> Result<Option<Page>, String> {
        let pages = self.find_all().await?;
        Ok(pages.into_iter().find(|p| p.id == id))
    }

    async fn update_description(&self, id: &str, description: &str) -> Result<(), String> {
        let _ = (id, description);
        Ok(())
    }

    async fn update_title(&self, id: &str, title: &str) -> Result<(), String> {
        let _ = (id, title);
        Ok(())
    }

    async fn create(&self, id: &str, title: &str) -> Result<Page, String> {
        Ok(Page {
            id: id.to_string(),
            title: title.to_string(),
            description: "".to_string(),
        })
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

    async fn find_by_id(&self, id: &str) -> Result<Option<Page>, String> {
        let db = libsql::Builder::new_remote(self.url.clone(), "".to_string())
            .build()
            .await
            .map_err(|e| e.to_string())?;

        let conn = db.connect().map_err(|e| e.to_string())?;

        let mut rows = conn
            .query(
                "SELECT id, title, description FROM pages WHERE id = ?1",
                libsql::params![id],
            )
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            Ok(Some(Page {
                id: row.get::<String>(0).map_err(|e| e.to_string())?,
                title: row.get::<String>(1).map_err(|e| e.to_string())?,
                description: row.get::<String>(2).map_err(|e| e.to_string())?,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_description(&self, id: &str, description: &str) -> Result<(), String> {
        let db = libsql::Builder::new_remote(self.url.clone(), "".to_string())
            .build()
            .await
            .map_err(|e| e.to_string())?;

        let conn = db.connect().map_err(|e| e.to_string())?;

        conn.execute(
            "UPDATE pages SET description = ?1 WHERE id = ?2",
            libsql::params![description, id],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_title(&self, id: &str, title: &str) -> Result<(), String> {
        let db = libsql::Builder::new_remote(self.url.clone(), "".to_string())
            .build()
            .await
            .map_err(|e| e.to_string())?;

        let conn = db.connect().map_err(|e| e.to_string())?;

        conn.execute(
            "UPDATE pages SET title = ?1 WHERE id = ?2",
            libsql::params![title, id],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn create(&self, id: &str, title: &str) -> Result<Page, String> {
        let db = libsql::Builder::new_remote(self.url.clone(), "".to_string())
            .build()
            .await
            .map_err(|e| e.to_string())?;

        let conn = db.connect().map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO pages (id, title, description) VALUES (?1, ?2, ?3)",
            libsql::params![id, title, ""],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(Page {
            id: id.to_string(),
            title: title.to_string(),
            description: "".to_string(),
        })
    }
}
