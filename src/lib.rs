//! A library implementation of the nostr protocol.
//!
//! nostr sources:
//! - [nostr nips (nostr improvement proposals)](https://github.com/fiatjaf/nostr)
mod error;

#[macro_use]
mod tags;

mod event;
mod util;

pub use event::{Event, Kind};
pub use tags::Tags;
pub use util::unix_u32_now;
