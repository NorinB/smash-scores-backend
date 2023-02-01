use std::collections::HashMap;

use async_graphql::{Context, EmptySubscription, Object, Schema, ID};
use futures::lock::Mutex;
use uuid7::uuid7;

use crate::model::tennis::{
    score::{InputTennisScoreData, TennisScoreData},
    shared::InputToSimpleObjectConvertible,
    tennis_match::{InputTennisMatch, OutputTennisMatch, TennisMatch},
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
        id: ID,
        new_point: InputTennisScoreData,
    ) -> TennisScoreData {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        let converted_point = new_point.to_simple_object();
        println!("Add point: {}", converted_point);
        let ongoing_match = storage.get_mut(&id).unwrap();
        ongoing_match.score_stack.push(converted_point.to_owned());
        converted_point
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
        storage.get(&ID(new_uuid)).unwrap().clone()
    }
}
