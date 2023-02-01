use async_graphql::{InputObject, SimpleObject, Enum};
use strum::Display;

use super::player::TennisPlayer;

#[derive(Clone, Debug, SimpleObject, InputObject)]
pub struct TennisScoreData {
    pub player: TennisPlayer,
    pub reason: ScoringReason
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
    DoubleFault
}