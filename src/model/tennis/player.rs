use std::fmt::Display;

use async_graphql::{InputObject, SimpleObject};

use crate::shared::input_to_simple_object_converter::InputToSimpleObjectConvertible;

#[derive(Clone, Debug, SimpleObject)]
pub struct TennisPlayer {
    id: i32,
    first_name: String,
    last_name: String,
    right_handed: bool,
}

impl Display for TennisPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.id, self.first_name, self.last_name, self.right_handed
        )
    }
}

#[derive(InputObject)]
pub struct InputTennisPlayer {
    id: i32,
    first_name: String,
    last_name: String,
    right_handed: bool,
}

impl InputToSimpleObjectConvertible<TennisPlayer> for InputTennisPlayer {
    fn to_simple_object(&self) -> TennisPlayer {
        TennisPlayer {
            id: self.id,
            first_name: self.first_name.to_string(),
            last_name: self.last_name.to_string(),
            right_handed: self.right_handed,
        }
    }
}
