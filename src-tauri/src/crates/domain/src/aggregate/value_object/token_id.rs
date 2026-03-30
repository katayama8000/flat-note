use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TokenId(i64);

impl TokenId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}
