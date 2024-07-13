#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    #[default]
    Tentative,
    Confirmed,
    Cancelled,
    NeedsAction,
    Completed,
    InProcess,
    Draft,
    Final,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Status::*;
        match self {
            Tentative => f.write_str("TENTATIVE"),
            Confirmed => f.write_str("CONFIRMED"),
            Cancelled => f.write_str("CANCELLED"),
            NeedsAction => f.write_str("NEEDS-ACTION"),
            Completed => f.write_str("COMPLETED"),
            InProcess => f.write_str("IN-PROCESS"),
            Draft => f.write_str("DRAFT"),
            Final => f.write_str("FINAL"),
        }
    }
}
