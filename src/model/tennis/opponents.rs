use async_graphql::{InputObject, SimpleObject};

use super::player::{InputTennisPlayer, TennisPlayer};
use crate::shared::input_to_simple_object_converter::InputToSimpleObjectConvertible;

#[derive(Clone, Debug, SimpleObject)]
pub struct TennisOpponents {
    pub home_player: Option<TennisPlayer>,
    pub home_doubles_partner: Option<TennisPlayer>,
    pub guest_player: Option<TennisPlayer>,
    pub guest_doubles_partner: Option<TennisPlayer>,
}

impl std::fmt::Display for TennisOpponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}, {:?}",
            self.home_player,
            self.home_doubles_partner,
            self.guest_player,
            self.guest_doubles_partner
        )
    }
}

#[derive(InputObject)]
pub struct InputTennisOpponents {
    pub home_player: Option<InputTennisPlayer>,
    pub home_doubles_partner: Option<InputTennisPlayer>,
    pub guest_player: Option<InputTennisPlayer>,
    pub guest_doubles_partner: Option<InputTennisPlayer>,
}

impl InputToSimpleObjectConvertible<TennisOpponents> for InputTennisOpponents {
    fn to_simple_object(&self) -> TennisOpponents {
        let convert = InputTennisOpponents::convert_to_optional_tennis_player;
        TennisOpponents {
            home_player: convert(&self.home_player),
            home_doubles_partner: convert(&self.home_doubles_partner),
            guest_player: convert(&self.guest_player),
            guest_doubles_partner: convert(&self.guest_doubles_partner),
        }
    }
}

impl InputTennisOpponents {
    fn convert_to_optional_tennis_player(
        input_player: &Option<InputTennisPlayer>,
    ) -> Option<TennisPlayer> {
        match input_player {
            Some(player) => Some(player.to_simple_object()),
            None => None,
        }
    }
}
