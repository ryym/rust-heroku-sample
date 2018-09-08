extern crate actix_web;
extern crate env_logger;

use actix_web::{server, App, HttpRequest, Responder};
use std::env;

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let host = format!("0.0.0.0:{}", port);

    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    }).bind(host)
        .unwrap()
        .run();
}
