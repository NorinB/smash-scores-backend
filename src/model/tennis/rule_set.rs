use async_graphql::{InputObject, SimpleObject};

#[derive(Clone, Debug, InputObject, SimpleObject)]
pub struct TennisRuleSet {
    pub best_of: i32,
    pub games_per_set: i32,
    pub super_tiebreak: bool,
    pub last_set_tiebreak_instead_of_two_ahead: bool
}

impl std::fmt::Display for TennisRuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}", self.best_of, self.games_per_set, self.super_tiebreak, self.last_set_tiebreak_instead_of_two_ahead)
    }
}