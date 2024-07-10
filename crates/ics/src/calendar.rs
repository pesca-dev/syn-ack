use ics_derive::Ics;

use crate::{CalProps, Components};

#[derive(Debug, Clone, PartialEq, Eq, Ics)]
#[key = "VCALENDAR"]
pub struct Calendar {
    props: CalProps,
    components: Vec<Components>,
}
