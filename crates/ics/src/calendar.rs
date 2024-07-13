use ics_derive::{Ics, Utils};

use crate::{CalProps, Components};

#[derive(Default, Debug, Clone, PartialEq, Eq, Ics, Utils)]
#[key = "VCALENDAR"]
pub struct Calendar {
    #[transparent]
    props: CalProps,
    components: Vec<Components>,
}
