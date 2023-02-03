use async_graphql::ID;

#[derive(Debug, Clone)]
pub struct GraphQLError {
    pub message: String,
}

impl std::fmt::Display for GraphQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn get_no_match_found_error(match_id: &ID) -> GraphQLError {
    GraphQLError {
        message: String::from(format!(
            "No ongoing match found with id: {}",
            match_id.to_string()
        )),
    }
}

pub fn get_no_point_to_undo_error(match_id: &ID) -> GraphQLError {
    GraphQLError {
        message: String::from(format!(
            "No available points to undo in match with id: {}",
            match_id.to_string()
        )),
    }
}
