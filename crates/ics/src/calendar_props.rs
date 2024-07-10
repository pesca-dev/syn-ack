use ics_derive::Ics;

#[derive(Debug, Clone, PartialEq, Eq, Ics)]
pub struct CalProps {
    prodid: String,

    /// Version of the iCalendar specification required to parse this iCalendar object.
    /// Currently, version 2.0 is the default.
    version: String,
    calscale: String,
    pub method: Option<String>,
    pub x_prop: Vec<String>,
    pub iana_prop: Vec<String>,
}

impl CalProps {
    pub fn new() -> Self {
        CalProps {
            prodid: "-//H1ghBre4k3r//ICS Rust v0.1//EN".to_string(),
            version: "2.0".into(),
            calscale: "GREGORIAN".into(),
            method: None,
            x_prop: vec![],
            iana_prop: vec![],
        }
    }
}

impl Default for CalProps {
    fn default() -> Self {
        Self::new()
    }
}
