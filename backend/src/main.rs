mod common;
mod v1;

use std::env::var;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;

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

    let address = var("ADDRESS").expect("ADDRESS must be set");

    let (host, port) = match address.find(':') {
        Some(index) => (
            &address[..index],
            address[index + 1..]
                .parse::<u16>()
                .expect("ADDRESS must be in the format host:port"),
        ),
        None => ("localhost", 8000),
    };

    HttpServer::new(|| App::new())
        .bind((host, port))
        .unwrap()
        .run()
        .await
}
