use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PageDescription(String);

impl PageDescription {
    pub fn new(description: impl Into<String>) -> Self {
        Self(description.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for PageDescription {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PageDescription {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for PageDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
