//! A library implementation of the nostr protocol.
//!
//! nostr sources:
//! - [nostr nips (nostr improvement proposals)](https://github.com/fiatjaf/nostr)
mod error;

mod event;
mod util;

pub use event::{Event, Kind, Tag};
pub use util::unix_u32_now;
