use actix_web::{web, HttpRequest, HttpResponse, Result};
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

use crate::schema::tennis_match_schema::MatchSchema;

pub async fn index(schema: web::Data<MatchSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_ws(
    schema: web::Data<MatchSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

pub async fn gql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/")
                .finish(),
        )
}
