use std::fmt::Display;

use async_graphql::{InputObject, SimpleObject, ID};

use super::{
    match_settings::{InputTennisMatchSettings, TennisMatchSettings},
    score::{InputTennisScoreData, TennisScoreData},
    shared::InputToSimpleObjectConvertible,
};

#[derive(Clone, Debug, SimpleObject)]
pub struct TennisMatch {
    pub score_stack: Vec<TennisScoreData>,
    pub match_settings: TennisMatchSettings,
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
    pub match_settings: TennisMatchSettings,
}

#[derive(InputObject)]
pub struct InputTennisMatch {
    pub score_stack: Vec<InputTennisScoreData>,
    pub match_settings: InputTennisMatchSettings,
}

impl InputToSimpleObjectConvertible<TennisMatch> for InputTennisMatch {
    fn to_simple_object(&self) -> TennisMatch {
        TennisMatch {
            score_stack: self
                .score_stack
                .iter()
                .map(|score_data| score_data.to_simple_object())
                .collect(),
            match_settings: self.match_settings.to_simple_object(),
        }
    }
}
