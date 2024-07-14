/// Struct for representing a unique identifier for an entry in a calendar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uid {
    inner: String,
}

impl Default for Uid {
    fn default() -> Self {
        Self {
            inner: uuid::Uuid::new_v4().to_string(),
        }
    }
}

impl std::fmt::Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.inner.as_str())
    }
}

impl From<&str> for Uid {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}
