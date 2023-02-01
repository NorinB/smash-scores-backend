use async_graphql::{SimpleObject, InputObject};

use super::{rule_set::TennisRuleSet, opponents::TennisOpponents};

#[derive(Clone, Debug, InputObject, SimpleObject)]
pub struct TennisMatchSettings {
    pub name: String,
    pub rule_set: TennisRuleSet,
    pub opponents: TennisOpponents,
    pub weather: String,
    pub venue: String,
    pub doubles: bool
}

impl std::fmt::Display for TennisMatchSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}, {}, {}", self.name, self.rule_set, self.opponents, self.weather, self.venue, self.doubles)
    }
}