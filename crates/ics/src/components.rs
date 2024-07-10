use ics_derive::Ics;

use crate::Eventc;

#[derive(Debug, Clone, PartialEq, Eq, Ics)]
pub enum Components {
    Eventc(Eventc),
    Todoc,
    Journalc,
    Freebusyc,
    Timezonec,
    IanaComp(IanaComp),
    XComp(XComp),
}

#[derive(Debug, Clone, PartialEq, Eq, Ics)]
pub struct IanaComp {}

#[derive(Debug, Clone, PartialEq, Eq, Ics)]
pub struct XComp {}
