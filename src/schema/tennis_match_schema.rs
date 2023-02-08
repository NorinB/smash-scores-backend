use std::collections::HashMap;

use async_graphql::{Context, Enum, Object, Result, Schema, Subscription, ID};
use futures::lock::Mutex;
use futures_util::{Stream, StreamExt};

use crate::{
    model::tennis::{
        player::{InputTennisPlayer, TennisPlayer},
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
                SimpleBroker::publish(MatchEvent {
                    match_event_type: MatchEventType::PointAdded,
                    match_id: match_id.to_owned(),
                    changed_point: Some(converted_point.to_owned()),
                    serving_starter_home: None,
                    serving_starter_guest: None,
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
                    SimpleBroker::publish(MatchEvent {
                        match_event_type: MatchEventType::PointRemoved,
                        match_id: match_id.to_owned(),
                        changed_point: Some(point.to_owned()),
                        serving_starter_home: None,
                        serving_starter_guest: None,
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

    async fn delete_match(
        &self,
        ctx: &Context<'_>,
        match_id: ID,
    ) -> Result<TennisMatch, SmashScoresGraphQLError> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        match storage.remove(&match_id) {
            Some(tennis_match) => Ok(tennis_match),
            None => Err(SmashScoresGraphQLError::get_no_match_found_error(&match_id)),
        }
    }

    async fn add_serving_starter(
        &self,
        ctx: &Context<'_>,
        match_id: ID,
        player: InputTennisPlayer,
        is_home: bool,
    ) -> Result<TennisPlayer, SmashScoresGraphQLError> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        match storage.get_mut(&match_id) {
            Some(tennis_match) => {
                let converted_player = player.to_simple_object();
                let serving_starter_to_return = match is_home {
                    true => {
                        tennis_match
                            .serving_starter_home
                            .push(Some(converted_player.to_owned()));
                        (
                            Some(tennis_match.serving_starter_home.to_owned()),
                            None,
                            MatchEventType::ServingStarterHomeChanged,
                        )
                    }
                    _ => {
                        tennis_match
                            .serving_starter_guest
                            .push(Some(converted_player.to_owned()));
                        (
                            None,
                            Some(tennis_match.serving_starter_guest.to_owned()),
                            MatchEventType::ServingStarterGuestChanged,
                        )
                    }
                };
                SimpleBroker::publish(MatchEvent {
                    match_id: match_id.to_owned(),
                    changed_point: None,
                    serving_starter_home: serving_starter_to_return.0,
                    serving_starter_guest: serving_starter_to_return.1,
                    match_event_type: serving_starter_to_return.2,
                });
                Ok(converted_player)
            }
            None => Err(SmashScoresGraphQLError::get_no_match_found_error(&match_id)),
        }
    }

    async fn remove_serving_starter(
        &self,
        ctx: &Context<'_>,
        match_id: ID,
        is_home: bool,
    ) -> Result<Option<TennisPlayer>, SmashScoresGraphQLError> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        match storage.get_mut(&match_id) {
            Some(tennis_match) => {
                let serving_starter_to_return = match is_home {
                    true => (
                        Some(tennis_match.serving_starter_home.to_owned()),
                        None,
                        MatchEventType::ServingStarterHomeChanged,
                        tennis_match.serving_starter_home.pop().unwrap(),
                    ),
                    _ => (
                        None,
                        Some(tennis_match.serving_starter_guest.to_owned()),
                        MatchEventType::ServingStarterGuestChanged,
                        tennis_match.serving_starter_guest.pop().unwrap(),
                    ),
                };
                SimpleBroker::publish(MatchEvent {
                    match_id: match_id.to_owned(),
                    changed_point: None,
                    serving_starter_home: serving_starter_to_return.0,
                    serving_starter_guest: serving_starter_to_return.1,
                    match_event_type: serving_starter_to_return.2,
                });
                Ok(serving_starter_to_return.3)
            }
            None => Err(SmashScoresGraphQLError::get_no_match_found_error(&match_id)),
        }
    }
}

#[derive(Clone)]
struct MatchEvent {
    match_event_type: MatchEventType,
    match_id: ID,
    serving_starter_home: Option<Vec<Option<TennisPlayer>>>,
    serving_starter_guest: Option<Vec<Option<TennisPlayer>>>,
    changed_point: Option<TennisScoreData>,
}

#[Object]
impl MatchEvent {
    async fn event_type(&self) -> MatchEventType {
        self.match_event_type
    }

    async fn match_id(&self) -> &ID {
        &self.match_id
    }

    async fn changed_point(&self) -> &Option<TennisScoreData> {
        &self.changed_point
    }

    async fn serving_starter_home(&self) -> &Option<Vec<Option<TennisPlayer>>> {
        &self.serving_starter_home
    }

    async fn serving_starter_guest(&self) -> &Option<Vec<Option<TennisPlayer>>> {
        &self.serving_starter_guest
    }
}

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
enum MatchEventType {
    PointAdded,
    PointRemoved,
    ServingStarterHomeChanged,
    ServingStarterGuestChanged,
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn watch_match(&self, match_id: Option<ID>) -> impl Stream<Item = MatchEvent> {
        SimpleBroker::<MatchEvent>::subscribe().filter(move |event| {
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
