use domain::aggregate::value_object::{PageDescription, PageId, PageTitle, SortBy, UserId};
use domain::{Page, PageRepository};

pub struct InMemoryPageRepository;

impl PageRepository for InMemoryPageRepository {
    async fn find_all(&self, owner_id: &UserId, sort_by: &SortBy) -> Result<Vec<Page>, String> {
        let mut pages = vec![
            Page::reconstruct(
                "1",
                "me-local-001",
                "Page returned from Rust",
                "This page was generated in Rust",
                "1710000000",
                "1710000000",
            ),
            Page::reconstruct(
                "2",
                "me-local-001",
                "Second page",
                "Automatically serialized to JSON via Serialize derive",
                "1710001000",
                "1710001000",
            ),
        ];
        pages.retain(|p| p.owner_id() == owner_id);
        match sort_by {
            SortBy::CreatedAt => {
                pages.sort_by(|a, b| a.created_at().value().cmp(b.created_at().value()))
            }
            SortBy::UpdatedAt => {
                pages.sort_by(|a, b| a.updated_at().value().cmp(b.updated_at().value()))
            }
        }
        Ok(pages)
    }

    async fn find_by_id(&self, owner_id: &UserId, id: &PageId) -> Result<Option<Page>, String> {
        let pages = self.find_all(owner_id, &SortBy::CreatedAt).await?;
        Ok(pages.into_iter().find(|p| p.id() == id))
    }

    async fn save(&self, _owner_id: &UserId, page: &Page) -> Result<(), String> {
        let _ = page;
        Ok(())
    }

    async fn create(&self, page: &Page) -> Result<Page, String> {
        Ok(page.clone())
    }

    async fn update_title_direct(
        &self,
        _owner_id: &UserId,
        _id: &PageId,
        _title: &PageTitle,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn update_description_direct(
        &self,
        _owner_id: &UserId,
        _id: &PageId,
        _description: &PageDescription,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn count(&self, owner_id: &UserId) -> Result<u64, String> {
        let pages = self.find_all(owner_id, &SortBy::CreatedAt).await?;
        Ok(pages.len() as u64)
    }

    async fn search(
        &self,
        owner_id: &UserId,
        keyword: &str,
        sort_by: &SortBy,
        limit: u32,
    ) -> Result<Vec<Page>, String> {
        let normalized = keyword.trim().to_lowercase();
        let mut pages = self.find_all(owner_id, sort_by).await?;
        if normalized.is_empty() {
            pages.truncate(limit as usize);
            return Ok(pages);
        }

        pages.retain(|page| {
            let title = page.title().value().to_lowercase();
            let description = page.description().value().to_lowercase();
            title.contains(&normalized) || description.contains(&normalized)
        });
        pages.truncate(limit as usize);
        Ok(pages)
    }

    async fn suggest_titles(
        &self,
        owner_id: &UserId,
        keyword: &str,
        limit: u32,
    ) -> Result<Vec<String>, String> {
        let normalized = keyword.trim().to_lowercase();
        if normalized.is_empty() {
            return Ok(Vec::new());
        }

        let pages = self.find_all(owner_id, &SortBy::UpdatedAt).await?;
        let mut titles = Vec::new();
        for page in pages {
            let title = page.title().value().trim();
            if title.is_empty() {
                continue;
            }

            if !title.to_lowercase().contains(&normalized) {
                continue;
            }

            if titles
                .iter()
                .any(|item: &String| item.eq_ignore_ascii_case(title))
            {
                continue;
            }

            titles.push(title.to_string());
            if titles.len() >= limit as usize {
                break;
            }
        }

        Ok(titles)
    }
}

pub struct LibSqlPageRepository {
    url: String,
}

impl LibSqlPageRepository {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    fn local_db_path(url: &str) -> Option<&str> {
        if let Some(path) = url.strip_prefix("file://") {
            return Some(path);
        }

        if let Some(path) = url.strip_prefix("file:") {
            return Some(path);
        }

        None
    }

    fn is_remote_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://") || url.starts_with("libsql://")
    }

    async fn connect(&self) -> Result<libsql::Connection, String> {
        let db = if let Some(path) = Self::local_db_path(&self.url) {
            libsql::Builder::new_local(path)
                .build()
                .await
                .map_err(|e| e.to_string())?
        } else if Self::is_remote_url(&self.url) {
            let auth_token = std::env::var("FLAT_NOTE_DB_AUTH_TOKEN").unwrap_or_default();
            libsql::Builder::new_remote(self.url.clone(), auth_token)
                .build()
                .await
                .map_err(|e| e.to_string())?
        } else {
            libsql::Builder::new_local(&self.url)
                .build()
                .await
                .map_err(|e| e.to_string())?
        };

        let conn = db.connect().map_err(|e| e.to_string())?;
        Self::ensure_schema(&conn).await?;
        Ok(conn)
    }

    async fn ensure_schema(conn: &libsql::Connection) -> Result<(), String> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pages (id TEXT PRIMARY KEY, owner_id TEXT NOT NULL, title TEXT NOT NULL, description TEXT NOT NULL, created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP)",
            (),
        )
        .await
        .map_err(|e| e.to_string())?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_pages_owner_id_updated_at ON pages (owner_id, updated_at DESC)",
            (),
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl PageRepository for LibSqlPageRepository {
    async fn find_all(&self, owner_id: &UserId, sort_by: &SortBy) -> Result<Vec<Page>, String> {
        let conn = self.connect().await?;
        let sort_column = match sort_by {
            SortBy::CreatedAt => "created_at",
            SortBy::UpdatedAt => "updated_at",
        };
        let query = format!(
            "SELECT id, owner_id, title, description, created_at, updated_at FROM pages WHERE owner_id = ?1 ORDER BY {} DESC",
            sort_column
        );

        let mut rows = conn
            .query(&query, libsql::params![owner_id.value()])
            .await
            .map_err(|e| e.to_string())?;

        let mut pages = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let id = row.get::<String>(0).map_err(|e| e.to_string())?;
            let owner_id = row.get::<String>(1).map_err(|e| e.to_string())?;
            let title = row.get::<String>(2).map_err(|e| e.to_string())?;
            let description = row.get::<String>(3).map_err(|e| e.to_string())?;
            let created_at = row.get::<String>(4).map_err(|e| e.to_string())?;
            let updated_at = row.get::<String>(5).map_err(|e| e.to_string())?;
            pages.push(Page::reconstruct(
                id,
                owner_id,
                title,
                description,
                created_at,
                updated_at,
            ));
        }

        Ok(pages)
    }

    async fn find_by_id(&self, owner_id: &UserId, id: &PageId) -> Result<Option<Page>, String> {
        let conn = self.connect().await?;

        let mut rows = conn
            .query(
                "SELECT id, owner_id, title, description, created_at, updated_at FROM pages WHERE id = ?1 AND owner_id = ?2",
                libsql::params![id.value(), owner_id.value()],
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
                row.get::<String>(5).map_err(|e| e.to_string())?,
            );
            Ok(Some(page))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, owner_id: &UserId, page: &Page) -> Result<(), String> {
        let conn = self.connect().await?;

        conn.execute(
            "UPDATE pages SET title = ?1, description = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3 AND owner_id = ?4",
            libsql::params![
                page.title().value(),
                page.description().value(),
                page.id().value(),
                owner_id.value()
            ],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn create(&self, page: &Page) -> Result<Page, String> {
        let conn = self.connect().await?;

        conn.execute(
            "INSERT INTO pages (id, owner_id, title, description) VALUES (?1, ?2, ?3, ?4)",
            libsql::params![
                page.id().value(),
                page.owner_id().value(),
                page.title().value(),
                page.description().value()
            ],
        )
        .await
        .map_err(|e| e.to_string())?;

        self.find_by_id(page.owner_id(), page.id())
            .await?
            .ok_or_else(|| format!("Page not found after creation: {}", page.id().value()))
    }

    async fn update_title_direct(
        &self,
        owner_id: &UserId,
        id: &PageId,
        title: &PageTitle,
    ) -> Result<(), String> {
        let conn = self.connect().await?;

        conn.execute(
            "UPDATE pages SET title = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2 AND owner_id = ?3",
            libsql::params![title.value(), id.value(), owner_id.value()],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_description_direct(
        &self,
        owner_id: &UserId,
        id: &PageId,
        description: &PageDescription,
    ) -> Result<(), String> {
        let conn = self.connect().await?;

        conn.execute(
            "UPDATE pages SET description = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2 AND owner_id = ?3",
            libsql::params![description.value(), id.value(), owner_id.value()],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn count(&self, owner_id: &UserId) -> Result<u64, String> {
        let conn = self.connect().await?;

        let mut rows = conn
            .query(
                "SELECT COUNT(*) FROM pages WHERE owner_id = ?1",
                libsql::params![owner_id.value()],
            )
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let count = row.get::<u64>(0).map_err(|e| e.to_string())?;
            Ok(count)
        } else {
            Ok(0)
        }
    }

    async fn search(
        &self,
        owner_id: &UserId,
        keyword: &str,
        sort_by: &SortBy,
        limit: u32,
    ) -> Result<Vec<Page>, String> {
        let conn = self.connect().await?;
        let sort_column = match sort_by {
            SortBy::CreatedAt => "created_at",
            SortBy::UpdatedAt => "updated_at",
        };

        let query = format!(
            "SELECT id, owner_id, title, description, created_at, updated_at FROM pages WHERE owner_id = ?1 AND (LOWER(title) LIKE '%' || LOWER(?2) || '%' OR LOWER(description) LIKE '%' || LOWER(?2) || '%') ORDER BY {} DESC LIMIT ?3",
            sort_column
        );

        let mut rows = conn
            .query(&query, libsql::params![owner_id.value(), keyword, limit])
            .await
            .map_err(|e| e.to_string())?;

        let mut pages = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            pages.push(Page::reconstruct(
                row.get::<String>(0).map_err(|e| e.to_string())?,
                row.get::<String>(1).map_err(|e| e.to_string())?,
                row.get::<String>(2).map_err(|e| e.to_string())?,
                row.get::<String>(3).map_err(|e| e.to_string())?,
                row.get::<String>(4).map_err(|e| e.to_string())?,
                row.get::<String>(5).map_err(|e| e.to_string())?,
            ));
        }

        Ok(pages)
    }

    async fn suggest_titles(
        &self,
        owner_id: &UserId,
        keyword: &str,
        limit: u32,
    ) -> Result<Vec<String>, String> {
        let conn = self.connect().await?;

        let mut rows = conn
            .query(
                "SELECT DISTINCT title FROM pages WHERE owner_id = ?1 AND LOWER(title) LIKE '%' || LOWER(?2) || '%' ORDER BY updated_at DESC LIMIT ?3",
                libsql::params![owner_id.value(), keyword, limit],
            )
            .await
            .map_err(|e| e.to_string())?;

        let mut titles = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let title = row.get::<String>(0).map_err(|e| e.to_string())?;
            if !title.trim().is_empty() {
                titles.push(title);
            }
        }

        Ok(titles)
    }
}
