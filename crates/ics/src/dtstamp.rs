#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DTStamp {
    pub date: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for DTStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(inner) = self.date {
            f.write_str(inner.format("%Y%m%dT%H%M%SZ").to_string().as_str())?;
        }

        Ok(())
    }
}
