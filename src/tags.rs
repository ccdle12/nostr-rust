pub type Tags = Vec<Vec<String>>;

macro_rules! tags_from_vec_str {
    ($($tags:expr),*) => {
        vec![
            $($tags.iter().map(|x| x.to_string()).collect()),*
        ]
    }
}
