use std::fmt::Display;

use async_graphql::{InputObject, SimpleObject, ID};

use super::{
    match_settings::{InputTennisMatchSettings, TennisMatchSettings},
    score::{InputTennisScoreData, TennisScoreData},
};
use crate::shared::input_to_simple_object_converter::InputToSimpleObjectConvertible;

#[derive(Clone, Debug, SimpleObject)]
pub struct TennisMatch {
    pub id: ID,
    pub score_stack: Vec<TennisScoreData>,
    pub match_settings: TennisMatchSettings,
}

impl Display for TennisMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.score_stack, self.match_settings)
    }
}

#[derive(InputObject)]
pub struct InputTennisMatch {
    pub id: ID,
    pub score_stack: Vec<InputTennisScoreData>,
    pub match_settings: InputTennisMatchSettings,
}

impl InputToSimpleObjectConvertible<TennisMatch> for InputTennisMatch {
    fn to_simple_object(&self) -> TennisMatch {
        TennisMatch {
            id: self.id.to_owned(),
            score_stack: self
                .score_stack
                .iter()
                .map(|score_data| score_data.to_simple_object())
                .collect(),
            match_settings: self.match_settings.to_simple_object(),
        }
    }
}
