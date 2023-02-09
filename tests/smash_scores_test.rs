use actix_web::{
    dev::{Service, ServiceResponse},
    guard, test,
    web::{self, Data},
    App,
};
use async_graphql::{Request, Schema, Variables};
use serde::de::value::Error;
use serde_json::json;
use smash_scores_backend::{
    api::index::{index, index_ws},
    schema::tennis_match_schema::{MutationRoot, QueryRoot, Storage, SubscriptionRoot},
};

async fn setup_smash_scores_test(
) -> impl Service<actix_http::Request, Response = ServiceResponse, Error = actix_web::Error> {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    test::init_service(
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            ),
    )
    .await
}

fn get_smash_scores_schema() -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
    Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish()
}

#[test]
async fn create_match() -> Result<(), Error> {
    let schema = get_smash_scores_schema();

    let create_match_request = "
    createMatch(inputTennisMatch: $emptyTennisMatch) {
        id,
        scoreStack {
           player {
          firstName
        },
        reason
      }
        matchSettings {
          name,
          ruleSet {
            bestOf
          },
          opponents {
            homePlayer {
              firstName
            },
            guestPlayer {
              firstName
            }
          },
          weather,
          venue,
          doubles
        },
        servingStarterHome {
          id,
          firstName,
          lastName,
          rightHanded
        },
        servingStarterGuest {
          id,
          firstName,
          lastName,
          rightHanded
        },
        startingTime,
        startingTeam
      }
    ";

    let result = schema
        .execute(
            Request::new(create_match_request).variables(Variables::from_json(json!({
                "emptyTennisMatch": {
                    "id": "0",
                    "scoreStack": [],
                    "matchSettings": {
                        "name": "Tournament",
                        "ruleSet": {
                            "bestOf": 3,
                            "gamesPerSet": 6,
                            "superTiebreak": true,
                            "lastSetTiebreakInsteadOfTwoAhead": false
                          },
                          "opponents": {
                              "homePlayer": {
                                  "id": 0,
                                  "firstName": "Noah",
                                  "lastName": "Bauer",
                                  "rightHanded": true
                                },
                        "homeDoublesPartner": null,
                        "guestPlayer": {
                            "id": 1,
                                  "firstName": "Christian",
                                  "lastName": "Krämer",
                                  "rightHanded": true
                        },
                        "guestDoublesPartner": null
                      },
                      "weather": "Sunny",
                      "venue": "Center Court",
                      "doubles": false
                    },
                    "servingStarterHome": {
                      "id": 0,
                      "firstName": "Noah",
                      "lastName": "Bauer",
                      "rightHanded": true
                    },
                    "servingStarterGuest": {
                      "id": 1,
                      "firstName": "Christian",
                      "lastName": "Krämer",
                      "rightHanded": true
                    },
                    "startingTime": "15:00",
                    "startingTeam": "HOME"
                  }
            }))),
        )
        .await;

    let data = result.data.into_json().unwrap();
    let json_match = &data["createMatch"];

    println!("{}", json_match);

    assert_eq!(json_match["id"], 0);

    Ok(())
}
