use domain::aggregate::value_object::{
    PageDescription, PageId, PageTitle, SortBy, TokenId, TokenName,
};
use domain::{Page, PageRepository, Token};

pub struct InMemoryPageRepository;

impl PageRepository for InMemoryPageRepository {
    async fn find_all(&self, sort_by: &SortBy) -> Result<Vec<Page>, String> {
        let mut pages = vec![
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
        ];
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

    async fn find_by_id(&self, id: &PageId) -> Result<Option<Page>, String> {
        let pages = self.find_all(&SortBy::CreatedAt).await?;
        Ok(pages.into_iter().find(|p| p.id() == id))
    }

    async fn save(&self, page: &Page) -> Result<(), String> {
        let _ = page;
        Ok(())
    }

    async fn create(&self, page: &Page) -> Result<Page, String> {
        Ok(page.clone())
    }

    async fn update_title_direct(&self, _id: &PageId, _title: &PageTitle) -> Result<(), String> {
        Ok(())
    }

    async fn update_description_direct(
        &self,
        _id: &PageId,
        _description: &PageDescription,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn count(&self) -> Result<u64, String> {
        let pages = self.find_all(&SortBy::CreatedAt).await?;
        Ok(pages.len() as u64)
    }

    async fn find_or_create_token(&self, name: &TokenName) -> Result<Token, String> {
        Ok(Token::new(1, name.value()))
    }

    async fn find_tokens_by_page_id(&self, _page_id: &PageId) -> Result<Vec<Token>, String> {
        Ok(vec![])
    }

    async fn sync_page_tokens(&self, _page_id: &PageId, _token_ids: &Vec<TokenId>) -> Result<(), String> {
        Ok(())
    }

    async fn find_related_pages(&self, _page_id: &PageId) -> Result<Vec<Page>, String> {
        Ok(vec![])
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
    async fn find_all(&self, sort_by: &SortBy) -> Result<Vec<Page>, String> {
        let conn = self.connect().await?;
        let sort_column = match sort_by {
            SortBy::CreatedAt => "created_at",
            SortBy::UpdatedAt => "updated_at",
        };
        let query = format!(
            "SELECT id, title, description, created_at, updated_at FROM pages ORDER BY {} DESC",
            sort_column
        );

        let mut rows = conn.query(&query, ()).await.map_err(|e| e.to_string())?;

        let mut pages = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let id = row.get::<String>(0).map_err(|e| e.to_string())?;
            let title = row.get::<String>(1).map_err(|e| e.to_string())?;
            let description = row.get::<String>(2).map_err(|e| e.to_string())?;
            let created_at = row.get::<String>(3).map_err(|e| e.to_string())?;
            let updated_at = row.get::<String>(4).map_err(|e| e.to_string())?;
            pages.push(Page::reconstruct(
                id,
                title,
                description,
                created_at,
                updated_at,
            ));
        }

        Ok(pages)
    }

    async fn find_by_id(&self, id: &PageId) -> Result<Option<Page>, String> {
        let conn = self.connect().await?;

        let mut rows = conn
            .query(
                "SELECT id, title, description, created_at, updated_at FROM pages WHERE id = ?1",
                libsql::params![id.value()],
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
                page.title().value(),
                page.description().value(),
                page.id().value()
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
                page.id().value(),
                page.title().value(),
                page.description().value()
            ],
        )
        .await
        .map_err(|e| e.to_string())?;

        self.find_by_id(page.id())
            .await?
            .ok_or_else(|| format!("Page not found after creation: {}", page.id().value()))
    }

    async fn update_title_direct(&self, id: &PageId, title: &PageTitle) -> Result<(), String> {
        let conn = self.connect().await?;

        conn.execute(
            "UPDATE pages SET title = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            libsql::params![title.value(), id.value()],
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_description_direct(
        &self,
        id: &PageId,
        description: &PageDescription,
    ) -> Result<(), String> {
        let conn = self.connect().await?;

        conn.execute(
            "UPDATE pages SET description = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            libsql::params![description.value(), id.value()],
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

    async fn find_or_create_token(&self, name: &TokenName) -> Result<Token, String> {
        let conn = self.connect().await?;
        conn.execute("INSERT OR IGNORE INTO tokens (name) VALUES (?1)", libsql::params![name.value()]).await.map_err(|e| e.to_string())?;
        let mut rows = conn.query("SELECT id, name FROM tokens WHERE name = ?1", libsql::params![name.value()]).await.map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let id = row.get::<i64>(0).map_err(|e| e.to_string())?;
            let name = row.get::<String>(1).map_err(|e| e.to_string())?;
            Ok(Token::new(id, name))
        } else {
            Err("Failed to find or create token".to_string())
        }
    }

    async fn find_tokens_by_page_id(&self, page_id: &PageId) -> Result<Vec<Token>, String> {
        let conn = self.connect().await?;
        let mut rows = conn.query("SELECT t.id, t.name FROM tokens t INNER JOIN page_tokens pt ON t.id = pt.token_id WHERE pt.page_id = ?1", libsql::params![page_id.value()]).await.map_err(|e| e.to_string())?;
        let mut tokens = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let id = row.get::<i64>(0).map_err(|e| e.to_string())?;
            let name = row.get::<String>(1).map_err(|e| e.to_string())?;
            tokens.push(Token::new(id, name));
        }
        Ok(tokens)
    }

    async fn sync_page_tokens(&self, page_id: &PageId, token_ids: &Vec<TokenId>) -> Result<(), String> {
        let conn = self.connect().await?;
        conn.execute("DELETE FROM page_tokens WHERE page_id = ?1", libsql::params![page_id.value()]).await.map_err(|e| e.to_string())?;
        for token_id in token_ids {
            conn.execute("INSERT INTO page_tokens (page_id, token_id) VALUES (?1, ?2)", libsql::params![page_id.value(), token_id.value()]).await.map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn find_related_pages(&self, page_id: &PageId) -> Result<Vec<Page>, String> {
        let conn = self.connect().await?;
        let mut rows = conn.query(
            "SELECT DISTINCT p.id, p.title, p.description, p.created_at, p.updated_at FROM pages p JOIN page_tokens pt1 ON p.id = pt1.page_id JOIN page_tokens pt2 ON pt1.token_id = pt2.token_id WHERE pt2.page_id = ?1 AND p.id != ?1",
            libsql::params![page_id.value()]
        ).await.map_err(|e| e.to_string())?;

        let mut pages = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let id = row.get::<String>(0).map_err(|e| e.to_string())?;
            let title = row.get::<String>(1).map_err(|e| e.to_string())?;
            let description = row.get::<String>(2).map_err(|e| e.to_string())?;
            let created_at = row.get::<String>(3).map_err(|e| e.to_string())?;
            let updated_at = row.get::<String>(4).map_err(|e| e.to_string())?;
            pages.push(Page::reconstruct(
                id,
                title,
                description,
                created_at,
                updated_at,
            ));
        }

        Ok(pages)
    }
}
