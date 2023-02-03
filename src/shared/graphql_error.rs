use async_graphql::ID;

#[derive(Debug, Clone)]
pub struct SmashScoresGraphQLError {
    pub message: String,
}

impl std::fmt::Display for SmashScoresGraphQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl SmashScoresGraphQLError {
    pub fn get_no_match_found_error(match_id: &ID) -> SmashScoresGraphQLError {
        SmashScoresGraphQLError {
            message: String::from(format!(
                "No ongoing match found with id: {}",
                match_id.to_string()
            )),
        }
    }

    pub fn get_no_point_to_undo_error(match_id: &ID) -> SmashScoresGraphQLError {
        SmashScoresGraphQLError {
            message: String::from(format!(
                "No available points to undo in match with id: {}",
                match_id.to_string()
            )),
        }
    }

    pub fn get_point_has_been_undone_error() -> SmashScoresGraphQLError {
        SmashScoresGraphQLError {
            message: String::from("Point has been undone, so there is no point to be added"),
        }
    }

    pub fn get_generic_error() -> SmashScoresGraphQLError {
        SmashScoresGraphQLError {
            message: String::from("An error occured"),
        }
    }
}
