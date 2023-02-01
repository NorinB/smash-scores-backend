use std::fmt::Display;

use async_graphql::{InputObject, SimpleObject};

#[derive(Clone, Debug, SimpleObject, InputObject)]
pub struct TennisPlayer {
    id: i32,
    first_name: String,
    last_name: String,
    right_handed: bool
}

impl Display for TennisPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}", self.id, self.first_name, self.last_name, self.right_handed)
    }
}
