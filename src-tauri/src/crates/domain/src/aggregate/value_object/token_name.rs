use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TokenName(String);

impl TokenName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
