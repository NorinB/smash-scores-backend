use std::collections::HashMap;

use async_graphql::{Context, EmptySubscription, Object, Schema, ID};
use futures::lock::Mutex;
use uuid7::uuid7;

use crate::{
    model::tennis::{
        score::{InputTennisScoreData, TennisScoreData},
        shared::InputToSimpleObjectConvertible,
        tennis_match::{InputTennisMatch, OutputTennisMatch, TennisMatch},
    },
    shared::graphql_error::{get_no_match_found_error, get_no_point_to_undo_error, GraphQLError},
};

pub type MatchSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub type Storage = Mutex<HashMap<ID, TennisMatch>>;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn matches(&self, ctx: &Context<'_>) -> Vec<OutputTennisMatch> {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        storage
            .iter()
            .map(|(id, tennis_match)| OutputTennisMatch {
                id: id.to_owned(),
                match_settings: tennis_match.match_settings.to_owned(),
                score_stack: tennis_match.score_stack.to_owned(),
            })
            .collect()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_point(
        &self,
        ctx: &Context<'_>,
        match_id: ID,
        new_point: InputTennisScoreData,
    ) -> std::result::Result<TennisScoreData, GraphQLError> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        let converted_point = new_point.to_simple_object();
        println!("Add point: {}", converted_point);
        match storage
            .get_mut(&match_id)
            .ok_or(get_no_match_found_error(&match_id))
        {
            Ok(ongoing_match) => {
                ongoing_match.score_stack.push(converted_point.to_owned());
                Ok(converted_point)
            }
            Err(e) => Err(e),
        }
    }

    async fn undo_point(
        &self,
        ctx: &Context<'_>,
        match_id: ID,
    ) -> std::result::Result<TennisScoreData, GraphQLError> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        match storage
            .get_mut(&match_id)
            .ok_or(get_no_match_found_error(&match_id))
        {
            Ok(ongoing_match) => match ongoing_match.score_stack.pop() {
                Some(point) => Ok(point),
                None => Err(get_no_point_to_undo_error(&match_id)),
            },
            Err(e) => Err(e),
        }
    }

    async fn create_match(
        &self,
        ctx: &Context<'_>,
        input_tennis_match: InputTennisMatch,
    ) -> TennisMatch {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        let converted_tennis_match = input_tennis_match.to_simple_object();
        println!("Create Match: {}", converted_tennis_match);
        let new_uuid = uuid7().to_string();
        storage.insert(ID(new_uuid.to_string()), converted_tennis_match);
        storage.get(&ID(new_uuid)).unwrap().to_owned()
    }
}
