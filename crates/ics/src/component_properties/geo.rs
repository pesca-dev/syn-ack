#[derive(Debug, Clone, PartialEq)]
pub struct Geo {
    pub lat: f64,
    pub lon: f64,
}

impl Eq for Geo {}

impl std::fmt::Display for Geo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.lat, self.lon))
    }
}
