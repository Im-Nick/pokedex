pub mod app_handler;

extern crate actix_web;
extern crate env_logger;
extern crate poke_api;

use crate::settings::Settings;
use actix_web::{middleware::Logger, App, HttpServer};
use log::info;

use self::app_handler::config_app;

pub(crate) async fn run() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let settings = Settings::new().expect("failed to load settings file");
    let server_address = (settings.host.to_owned(), settings.port.to_owned());

    HttpServer::new(move || App::new().wrap(Logger::default()).configure(config_app))
        .bind(server_address.clone())?
        .run()
        .await
        .and_then(move |e| {
            info!(
                "Server up and running at {}:{}",
                server_address.0, server_address.1
            );
            Ok(e)
        })
}
