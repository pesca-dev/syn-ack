#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Transparency {
    #[default]
    Opaque,
    Transparent,
}

impl std::fmt::Display for Transparency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Transparency::*;
        match self {
            Opaque => f.write_str("OPAQUE"),
            Transparent => f.write_str("TRANSPARENT"),
        }
    }
}
