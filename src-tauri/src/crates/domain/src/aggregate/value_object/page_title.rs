use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PageTitle(String);

impl PageTitle {
    pub fn new(title: impl Into<String>) -> Self {
        Self(title.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for PageTitle {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PageTitle {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for PageTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
