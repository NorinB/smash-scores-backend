use std::collections::HashMap;

use async_graphql::{Context, EmptySubscription, Object, Schema, ID};
use futures::lock::Mutex;
use uuid7::uuid7;

use crate::model::tennis::{
    tennis_match::{OutputTennisMatch, TennisMatch}, score::TennisScoreData,
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
        point: TennisScoreData,
    ) -> TennisScoreData {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        println!("Add point: {}", point);
        let ongoing_match = storage.get_mut(&id).unwrap();
        ongoing_match.score_stack.push(point.clone());
        point
    }

    async fn create_match(&self, ctx: &Context<'_>, tennis_match: TennisMatch) -> TennisMatch {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        println!("Create Match: {}", tennis_match);
        storage.insert(ID(uuid7().to_string()), tennis_match.to_owned()).unwrap()
    }
}
