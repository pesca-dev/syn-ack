use ics_derive::Ics;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Calendar {
    props: CalProps,
    components: Vec<Components>,
}

pub trait Ics {
    fn print_ics(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, Ics)]
pub struct CalProps {
    prodid: String,

    /// Version of the iCalendar specification required to parse this iCalendar object.
    /// Currently, version 2.0 is the default.
    version: String,
    // TODO: incorrect type
    pub calscale: Option<String>,
    // TODO: incorrect type
    pub method: Option<String>,
    // TODO: incorrect type
    pub x_prop: Vec<String>,
    // TODO: incorrect type
    pub iana_prop: Vec<String>,
}

impl CalProps {
    pub fn new() -> Self {
        CalProps {
            prodid: "-//H1ghBre4k3r//ICS Rust v0.1//EN".to_string(),
            version: "2.0".into(),
            calscale: None,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Components {
    Eventc(Eventc),
    Todoc,
    Journalc,
    Freebusyc,
    Timezonec,
    IanaComp(IanaComp),
    XComp(XComp),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IanaComp {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XComp {}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Eventc {
    // required
    pub dtstamp: (),
    // required
    pub uid: (),
    // required if calendar has no method
    pub dtstart: (),

    // optional, but only once
    pub class: Option<Class>,

    /// Information about the creation time of this event.
    pub created: Option<u64>,
    pub description: Option<()>,
    pub geo: Option<()>,
    pub last_mod: Option<()>,
    pub location: Option<()>,
    pub organizer: Option<()>,
    pub priority: Option<()>,
    pub seq: Option<()>,
    pub status: Option<()>,
    pub summary: Option<()>,
    pub transp: Option<()>,
    pub url: Option<()>,
    pub recurid: Option<()>,

    // optional, should not appeare more than once
    pub rrule: Option<()>,

    // both may appear, but mutually exclusive
    // TODO: maybe introduce enum for that
    pub dtend: Option<()>,
    pub duration: Option<()>,

    pub attach: Vec<()>,
    pub attendee: Vec<()>,
    pub categories: Vec<()>,
    pub comment: Vec<()>,
    pub contact: Vec<()>,
    pub exdate: Vec<()>,
    pub rstatus: Vec<()>,
    pub related: Vec<()>,
    pub resources: Vec<()>,
    pub rdate: Vec<()>,
    pub x_prop: Vec<()>,
    pub iana_prop: Vec<()>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Class {
    Public,
    Private,
    Confidential,
}
