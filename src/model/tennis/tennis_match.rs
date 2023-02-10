use async_graphql::{Enum, InputObject, SimpleObject, ID};
use strum::Display;

use super::{
    match_settings::{InputTennisMatchSettings, TennisMatchSettings},
    player::{InputTennisPlayer, TennisPlayer},
    score::{InputTennisScoreData, TennisScoreData},
};
use crate::shared::input_to_simple_object_converter::InputToSimpleObjectConvertible;

#[derive(Clone, Debug, SimpleObject)]
pub struct TennisMatch {
    pub id: ID,
    pub score_stack: Vec<TennisScoreData>,
    pub match_settings: TennisMatchSettings,
    pub serving_starter_home: Vec<Option<TennisPlayer>>,
    pub serving_starter_guest: Vec<Option<TennisPlayer>>,
    pub starting_time: String,
    pub starting_team: OpponentType,
}

impl std::fmt::Display for TennisMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {:?}, {}, {:?}, {:?}, {}, {}",
            self.id.to_string(),
            self.score_stack,
            self.match_settings,
            self.serving_starter_home,
            self.serving_starter_guest,
            self.starting_time,
            self.starting_team
        )
    }
}

#[derive(Clone, Copy, Debug, Display, Enum, Eq, PartialEq)]
pub enum OpponentType {
    Home,
    Guest,
}

#[derive(InputObject)]
pub struct InputTennisMatch {
    pub id: ID,
    pub score_stack: Vec<InputTennisScoreData>,
    pub match_settings: InputTennisMatchSettings,
    pub serving_starter_home: Vec<Option<InputTennisPlayer>>,
    pub serving_starter_guest: Vec<Option<InputTennisPlayer>>,
    pub starting_time: String,
    pub starting_team: OpponentType,
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
            serving_starter_home: self
                .serving_starter_home
                .iter()
                .map(|input_player| match input_player {
                    Some(player) => Some(player.to_simple_object()),
                    None => None,
                })
                .collect(),
            serving_starter_guest: self
                .serving_starter_guest
                .iter()
                .map(|input_player| match input_player {
                    Some(player) => Some(player.to_simple_object()),
                    None => None,
                })
                .collect(),
            starting_time: self.starting_time.to_string(),
            starting_team: self.starting_team,
        }
    }
}
