use super::value_object::{TokenId, TokenName};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Token {
    id: TokenId,
    name: TokenName,
}

impl Token {
    pub fn new(id: i64, name: impl Into<String>) -> Self {
        Self {
            id: TokenId::new(id),
            name: TokenName::new(name),
        }
    }

    pub fn id(&self) -> &TokenId {
        &self.id
    }

    pub fn name(&self) -> &TokenName {
        &self.name
    }
}
