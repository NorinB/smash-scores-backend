use async_graphql::{InputObject, SimpleObject};

use super::{
    opponents::{InputTennisOpponents, TennisOpponents},
    rule_set::{InputTennisRuleSet, TennisRuleSet},
};
use crate::shared::input_to_simple_object_converter::InputToSimpleObjectConvertible;

#[derive(Clone, Debug, SimpleObject)]
pub struct TennisMatchSettings {
    pub name: String,
    pub rule_set: TennisRuleSet,
    pub opponents: TennisOpponents,
    pub weather: String,
    pub venue: String,
    pub doubles: bool,
}

impl std::fmt::Display for TennisMatchSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}",
            self.name, self.rule_set, self.opponents, self.weather, self.venue, self.doubles
        )
    }
}

#[derive(InputObject)]
pub struct InputTennisMatchSettings {
    pub name: String,
    pub rule_set: InputTennisRuleSet,
    pub opponents: InputTennisOpponents,
    pub weather: String,
    pub venue: String,
    pub doubles: bool,
}

impl InputToSimpleObjectConvertible<TennisMatchSettings> for InputTennisMatchSettings {
    fn to_simple_object(&self) -> TennisMatchSettings {
        TennisMatchSettings {
            name: self.name.to_string(),
            rule_set: self.rule_set.to_simple_object(),
            opponents: self.opponents.to_simple_object(),
            weather: self.weather.to_string(),
            venue: self.venue.to_string(),
            doubles: self.doubles,
        }
    }
}
