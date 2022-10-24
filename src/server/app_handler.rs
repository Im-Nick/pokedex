use actix_web::dev::Service as _;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{self, ServiceConfig};

use poke_api::api;

use crate::routes::{index, pokemon};
use crate::settings::Settings;

pub(crate) fn config_app(config: &mut ServiceConfig) {
    set_app_data(config);
    index_handler(config);
    poke_api_handler(config);
}

fn set_app_data(config: &mut ServiceConfig) {
    let settings = Settings::new().expect("Failed to load toml file");
    let poke_api = api::PokeApi::new(
        settings.api.pokemon_api,
        settings.api.yoda_api,
        settings.api.shakespeare_api,
    );
    config.app_data(web::Data::new(poke_api.clone()));
}

fn index_handler(config: &mut ServiceConfig) {
    config.service(web::scope("/").service(index::health_check));
}

fn poke_api_handler(config: &mut ServiceConfig) {
    config.service(
        web::scope("/pokemon")
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    let name = res.request().match_info().query("name");

                    // Don't need to call endpoint if pokemon length too long.
                    if name.len() > 24 {
                        return Err(ErrorBadRequest("Pokemon name too long"));
                    }
                    Ok(res)
                })
            })
            .service(pokemon::get_pokemon)
            .service(pokemon::get_pokemon_translation),
    );
}
