use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreatedAt(String);

impl CreatedAt {
    pub fn new(timestamp: impl Into<String>) -> Self {
        Self(timestamp.into())
    }

    pub fn now() -> Self {
        Self(Self::current_unix_timestamp())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn current_unix_timestamp() -> String {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        seconds.to_string()
    }
}

impl From<String> for CreatedAt {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for CreatedAt {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for CreatedAt {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
