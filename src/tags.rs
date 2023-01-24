/// Alias for a vector of a vector of strings. The first element of each subarray
/// is the tag identifier:
///   - "p": pubkey, which points to a pubkey of someone that is referred to in the event
///   - "e": event, which points to the id of an event this event is quoting, replying to or
///   referring to somehow
pub type Tags = Vec<Vec<String>>;

/// Helper macro to convert a collection of str slices to the formatting for the type Tags.
#[macro_export]
macro_rules! tags_from_strs {
    ($($tags:expr),*) => {
        vec![
            $($tags.iter().map(|x| x.to_string()).collect()),*
        ]
    }
}
