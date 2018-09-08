extern crate actix_web;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;

use actix_web::{server, App, HttpRequest, Responder};
use diesel::prelude::*;
use std::{env, error::Error};

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() -> Result<(), Box<Error>> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL")?;

    let _ = PgConnection::establish(&database_url)?;
    println!("CONNECTED");

    let host = if env::var("LOCAL_SERVER").ok().is_some() {
        "127.0.0.1"
    } else {
        "0.0.0.0"
    };
    let port = env::var("PORT").unwrap_or("8080".to_owned());

    let host = format!("{}:{}", host, port);

    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    }).bind(host)
        .unwrap()
        .run();

    Ok(())
}
