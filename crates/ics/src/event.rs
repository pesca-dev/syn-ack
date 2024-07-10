use ics_derive::Ics;

use crate::{Class, DTStamp};
#[derive(Default, Debug, Clone, PartialEq, Eq, Ics)]
#[key = "VEVENT"]
pub struct Eventc {
    // required
    pub dtstamp: DTStamp,
    // required
    pub uid: (),
    // required if calendar has no method
    pub dtstart: (),

    // optional, but only once
    pub class: Option<Class>,

    /// Information about the creation time of this event.
    pub created: Option<u64>,
    pub description: Option<String>,
    #[skip]
    pub geo: Option<()>,
    #[skip]
    pub last_mod: Option<()>,
    #[skip]
    pub location: Option<()>,
    #[skip]
    pub organizer: Option<()>,
    #[skip]
    pub priority: Option<()>,
    #[skip]
    pub seq: Option<()>,
    #[skip]
    pub status: Option<()>,
    #[skip]
    pub summary: Option<()>,
    #[skip]
    pub transp: Option<()>,
    #[skip]
    pub url: Option<()>,
    #[skip]
    pub recurid: Option<()>,

    // optional, should not appeare more than once
    #[skip]
    pub rrule: Option<()>,

    // both may appear, but mutually exclusive
    // TODO: maybe introduce enum for that
    #[skip]
    pub dtend: Option<()>,
    #[skip]
    pub duration: Option<()>,

    #[skip]
    pub attach: Vec<()>,
    #[skip]
    pub attendee: Vec<()>,
    #[skip]
    pub categories: Vec<()>,
    #[skip]
    pub comment: Vec<()>,
    #[skip]
    pub contact: Vec<()>,
    #[skip]
    pub exdate: Vec<()>,
    #[skip]
    pub rstatus: Vec<()>,
    #[skip]
    pub related: Vec<()>,
    #[skip]
    pub resources: Vec<()>,
    #[skip]
    pub rdate: Vec<()>,
    #[skip]
    pub x_prop: Vec<()>,
    #[skip]
    pub iana_prop: Vec<()>,
}

impl Eventc {
    pub fn with_date(mut self, date: chrono::DateTime<chrono::Utc>) -> Self {
        self.dtstamp.date = Some(date);
        self
    }
}
