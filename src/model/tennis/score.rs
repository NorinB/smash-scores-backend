use async_graphql::{Enum, InputObject, SimpleObject};
use strum::Display;

use super::player::{InputTennisPlayer, TennisPlayer};
use crate::shared::input_to_simple_object_converter::InputToSimpleObjectConvertible;

// TODO: OutPut and Input Data einzeln machen
#[derive(Clone, Debug, SimpleObject)]
pub struct TennisScoreData {
    pub player: TennisPlayer,
    pub reason: ScoringReason,
}

impl std::fmt::Display for TennisScoreData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.player, self.reason)
    }
}

#[derive(Clone, Copy, Debug, Display, Enum, Eq, PartialEq)]
pub enum ScoringReason {
    Ace,
    Winner,
    UnforcedError,
    Fault,
    DoubleFault,
}

#[derive(InputObject)]
pub struct InputTennisScoreData {
    pub player: InputTennisPlayer,
    pub reason: ScoringReason,
}

impl InputToSimpleObjectConvertible<TennisScoreData> for InputTennisScoreData {
    fn to_simple_object(&self) -> TennisScoreData {
        TennisScoreData {
            player: self.player.to_simple_object(),
            reason: self.reason,
        }
    }
}
