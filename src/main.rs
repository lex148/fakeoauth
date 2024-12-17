use crate::controllers::*;
use actix_web::{App, HttpServer};
use std::env;
use std::net::SocketAddr;

mod controllers;
mod errors;
mod helpers;
mod models;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    // read the environment variables
    let port = env::var("PORT").unwrap_or_else(|_| "5860".to_owned());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let bind_interface: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    // boot up the server
    log::info!("Server Running: http://{}", bind_interface);
    HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/assets", "./src/assets"))
            .service(assets_controller::styles)
            // The auth routes
            .service(greetings_controller::index)
            .service(greetings_controller::index2)
            // internal, you clicked the turtle
            .service(logins_controller::index)
            // Token exchange
            .service(tokens_controller::index)
            .service(tokens_controller::index2)
            // Other server requesting info
            .service(userinfos_controller::index)
            .service(userinfos_controller::index2)
    })
    .bind(bind_interface)?
    .run()
    .await
}
