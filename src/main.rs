use actix_web::{get, web, App, HttpServer, Responder};
use log::{ info, error };

use rspotify::{
    model::{AdditionalType, Country, Market},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting server on http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} 
