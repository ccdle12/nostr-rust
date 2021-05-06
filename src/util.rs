use crate::error::Result;
use std::time::SystemTime;

/// A helper function to return a unix time stamp in seconds as a u32.
pub fn unix_u32_now() -> Result<u32> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as u32)?)
}
