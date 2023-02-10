use async_graphql::{InputObject, SimpleObject};

use crate::shared::input_to_simple_object_converter::InputToSimpleObjectConvertible;

#[derive(Clone, Debug, SimpleObject)]
pub struct TennisRuleSet {
    pub best_of: i32,
    pub games_per_set: i32,
    pub super_tiebreak: bool,
    pub last_set_tiebreak_instead_of_two_ahead: bool,
}

impl std::fmt::Display for TennisRuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.best_of,
            self.games_per_set,
            self.super_tiebreak,
            self.last_set_tiebreak_instead_of_two_ahead
        )
    }
}

#[derive(InputObject)]
pub struct InputTennisRuleSet {
    pub best_of: i32,
    pub games_per_set: i32,
    pub super_tiebreak: bool,
    pub last_set_tiebreak_instead_of_two_ahead: bool,
}

impl InputToSimpleObjectConvertible<TennisRuleSet> for InputTennisRuleSet {
    fn to_simple_object(&self) -> TennisRuleSet {
        TennisRuleSet {
            best_of: self.best_of,
            games_per_set: self.games_per_set,
            super_tiebreak: self.super_tiebreak,
            last_set_tiebreak_instead_of_two_ahead: self.last_set_tiebreak_instead_of_two_ahead,
        }
    }
}
