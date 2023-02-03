use actix_web::{
    guard,
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use async_graphql::{Schema, http::GraphiQLSource, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use smash_scores_backend::schema::match_info::{Storage, MatchSchema, QueryRoot, MutationRoot};

async fn index(schema: web::Data<MatchSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Compress::default())
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playground))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
