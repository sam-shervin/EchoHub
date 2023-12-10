mod common;
mod v1;

use std::env::var;

use actix_web::{web::Data, App, HttpServer};
use common::types::AppState;
use dotenvy::dotenv;
use sea_orm::Database;
use v1::api::auth::{signin, signup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match dotenv() {
        Ok(_) => {
            env_logger::init();
        }
        Err(e) => {
            panic!("{}", e);
        }
    };

    let _guard = sentry::init((
        var("SENTRY_DSN").expect("SENTRY_DSN must be set"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let address = match var("ADDRESS") {
        Ok(address) => address,
        Err(_) => "localhost:8000".to_owned(),
    };

    let (host, port) = match address.find(':') {
        Some(index) => (
            &address[..index],
            address[index + 1..]
                .parse::<u16>()
                .expect("ADDRESS must be in the format host:port"),
        ),
        None => ("localhost", 8000),
    };

    let postgres_uri = var("POSTGRES_URI").expect("POSTGRES_URI must be set");
    let app_state = AppState::new(Database::connect(postgres_uri).await.unwrap());

    HttpServer::new(move || {
        App::new()
            .service(signup)
            .service(signin)
            .app_data(Data::new(app_state.clone()))
    })
    .bind((host, port))
    .unwrap()
    .run()
    .await
}
