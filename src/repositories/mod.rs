//! Module for interacting on a low level with the database. This layer should not perform any
//! verification on the integrity of the data.
mod user;

pub use self::user::*;
