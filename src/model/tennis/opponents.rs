use async_graphql::{InputObject, SimpleObject};

use super::player::TennisPlayer;

#[derive(Clone, Debug, InputObject, SimpleObject)]
pub struct TennisOpponents {
    pub home_player: Option<TennisPlayer>,
    pub home_doubles_partner: Option<TennisPlayer>,
    pub guest_player: Option<TennisPlayer>,
    pub guest_doubles_partner: Option<TennisPlayer>
}

impl std::fmt::Display for TennisOpponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}, {:?}, {:?}", self.home_player, self.home_doubles_partner, self.guest_player, self.guest_doubles_partner)
    }
}