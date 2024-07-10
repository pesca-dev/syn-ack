use ics_derive::Ics;

#[derive(Debug, Clone, PartialEq, Eq, Ics)]
pub enum Class {
    Public,
    Private,
    Confidential,
}
