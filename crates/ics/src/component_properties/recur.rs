use ics_derive::Utils;

/// This value type is used to identify properties that contain
/// a recurrence rule specification.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Recur {
    rules: Vec<RecurRule>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Utils)]
pub struct RecurRule {}
