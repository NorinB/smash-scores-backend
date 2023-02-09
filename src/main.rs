use actix_web::{
    guard,
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpServer,
};
use async_graphql::Schema;

use smash_scores_backend::{
    api::index::{gql_playground, index, index_ws},
    schema::tennis_match_schema::{MutationRoot, QueryRoot, Storage, SubscriptionRoot},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Compress::default())
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(gql_playground))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
