use std::fmt::Display;

use async_graphql::{ID, SimpleObject, InputObject};

use super::{score::TennisScoreData, match_settings::TennisMatchSettings};

#[derive(Clone, Debug, SimpleObject, InputObject)]
pub struct TennisMatch {
    pub score_stack: Vec<TennisScoreData>,
    pub match_settings: TennisMatchSettings
}

impl Display for TennisMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.score_stack, self.match_settings)
    }
}

#[derive(SimpleObject)]
pub struct OutputTennisMatch {
    pub id: ID,
    pub score_stack: Vec<TennisScoreData>,
    pub match_settings: TennisMatchSettings
}