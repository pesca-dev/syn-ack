use ics_derive::Ics;

use crate::{Class, DTStamp, Geo, Uid};

/// Struct for representing an event entry in a calendar.
#[derive(Default, Debug, Clone, PartialEq, Eq, Ics)]
#[key = "VEVENT"]
pub struct Eventc {
    // required
    pub dtstamp: DTStamp,

    /// This property defines the persistent, globally unique identifier for the calendar component.
    uid: Uid,

    // This property specifies when the calendar component begins.
    // required if calendar has no method
    // this can also be a date instead of datetime
    pub dtstart: DTStamp,

    // optional, but only once
    pub class: Option<Class>,

    /// Information about the creation time of this event.
    pub created: Option<u64>,

    /// This property provides a more complete description of the calendar component than that provided by the "SUMMARY" property.
    pub description: Option<String>,

    /// This property specifies information related to the global position for the activity specified by a calendar component.
    pub geo: Option<Geo>,

    #[skip]
    pub last_mod: Option<()>,

    /// This property defines the intended venue for the activity defined by a calendar component.
    pub location: Option<String>,

    /// This property defines the organizer for a calendar component.
    #[skip]
    pub organizer: Option<()>,

    /// This property defines the relative priority for a calendar component.
    pub priority: Option<u8>,

    /// This property defines the revision sequence number of the calendar component within a sequence of revisions.
    #[key = "SEQUENCE"]
    pub seq: Option<u64>,

    /// This property defines the overall status or confirmation for the calendar component.
    #[skip]
    pub status: Option<()>,

    /// This property defines a short summary or subject for the calendar component.
    pub summary: Option<String>,

    /// This property defines whether or not an event is transparent to busy time searches.
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
    pub fn with_dtstamp(mut self, date: chrono::DateTime<chrono::Utc>) -> Self {
        self.dtstamp.date = Some(date);
        self
    }

    pub fn with_start(mut self, date: chrono::DateTime<chrono::Utc>) -> Self {
        self.dtstart.date = Some(date);
        self
    }

    pub fn with_geo(mut self, geo: Geo) -> Self {
        self.geo = Some(geo);
        self
    }

    pub fn with_location(mut self, location: impl ToString) -> Self {
        self.location = Some(location.to_string());
        self
    }
}
