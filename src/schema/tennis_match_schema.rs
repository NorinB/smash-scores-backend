use std::collections::HashMap;

use async_graphql::{Context, Enum, Object, Result, Schema, Subscription, ID};
use futures::lock::Mutex;
use futures_util::{Stream, StreamExt};

use crate::{
    model::tennis::{
        score::{InputTennisScoreData, TennisScoreData},
        tennis_match::{InputTennisMatch, TennisMatch},
    },
    shared::{
        graphql_error::SmashScoresGraphQLError,
        input_to_simple_object_converter::InputToSimpleObjectConvertible,
    },
};

use super::simple_broker::SimpleBroker;

pub type MatchSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub type Storage = Mutex<HashMap<ID, TennisMatch>>;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn all_tennis_matches(&self, ctx: &Context<'_>) -> Vec<TennisMatch> {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        storage
            .iter()
            .map(|(_, tennis_match)| tennis_match.to_owned())
            .collect()
    }

    async fn tennis_match(
        &self,
        ctx: &Context<'_>,
        match_id: ID,
    ) -> Result<TennisMatch, SmashScoresGraphQLError> {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        match storage
            .get(&match_id)
            .ok_or(SmashScoresGraphQLError::get_no_match_found_error(&match_id))
        {
            Ok(ongoing_match) => Ok(ongoing_match.to_owned()),
            Err(e) => Err(e),
        }
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
    ) -> Result<TennisScoreData, SmashScoresGraphQLError> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        let converted_point = new_point.to_simple_object();
        println!("Add point: {}", converted_point);
        match storage
            .get_mut(&match_id)
            .ok_or(SmashScoresGraphQLError::get_no_match_found_error(&match_id))
        {
            Ok(ongoing_match) => {
                ongoing_match.score_stack.push(converted_point.to_owned());
                SimpleBroker::publish(TennisPointsChanged {
                    points_change_type: TennisPointsChangeType::Added,
                    match_id: match_id.to_owned(),
                    changed_point: converted_point.to_owned(),
                });
                Ok(converted_point)
            }
            Err(e) => Err(e),
        }
    }

    async fn undo_point(
        &self,
        ctx: &Context<'_>,
        match_id: ID,
    ) -> Result<TennisScoreData, SmashScoresGraphQLError> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        match storage
            .get_mut(&match_id)
            .ok_or(SmashScoresGraphQLError::get_no_match_found_error(&match_id))
        {
            Ok(ongoing_match) => match ongoing_match.score_stack.pop() {
                Some(point) => {
                    SimpleBroker::publish(TennisPointsChanged {
                        points_change_type: TennisPointsChangeType::Removed,
                        match_id: match_id.to_owned(),
                        changed_point: point.to_owned(),
                    });
                    Ok(point)
                }
                None => Err(SmashScoresGraphQLError::get_no_point_to_undo_error(
                    &match_id,
                )),
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
        storage.insert(
            converted_tennis_match.id.to_owned(),
            converted_tennis_match.to_owned(),
        );
        converted_tennis_match
    }
}

#[derive(Clone)]
struct TennisPointsChanged {
    points_change_type: TennisPointsChangeType,
    match_id: ID,
    changed_point: TennisScoreData,
}

#[Object]
impl TennisPointsChanged {
    async fn points_change_type(&self) -> TennisPointsChangeType {
        self.points_change_type
    }

    async fn match_id(&self) -> &ID {
        &self.match_id
    }

    async fn changed_point(&self) -> &TennisScoreData {
        &self.changed_point
    }
}

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
enum TennisPointsChangeType {
    Added,
    Removed,
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn watch_match(&self, match_id: Option<ID>) -> impl Stream<Item = TennisPointsChanged> {
        SimpleBroker::<TennisPointsChanged>::subscribe().filter(move |event| {
            let res = if let Some(match_id) = match_id.clone() {
                println!("{:?}", event.match_id);
                match_id == event.match_id
            } else {
                false
            };
            async move { res }
        })
    }
}
