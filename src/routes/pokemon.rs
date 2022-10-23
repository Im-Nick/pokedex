use actix_web::{get, web, HttpResponse};
use poke_api::{api::PokeApi, errors::ErrorStatus};

use crate::errors::MyError;

#[get("/{name}")]
pub async fn get_pokemon(
    api: web::Data<PokeApi>,
    name: web::Path<String>,
) -> Result<HttpResponse, MyError> {
    let pokemon = api.search(&name.trim()).await?;
    Ok(HttpResponse::Ok().json(pokemon))
}

#[get("/translated/{name}")]
pub async fn get_pokemon_translation(
    api: web::Data<PokeApi>,
    name: web::Path<String>,
) -> Result<HttpResponse, MyError> {
    let mut pokemon = api.search(&name.trim()).await?;
    let translated_pokemon = api.translate(&mut pokemon).await;

    match translated_pokemon {
        Ok(translated_poke) => Ok(HttpResponse::Ok().json(translated_poke)),
        Err(e) => match e.status {
            ErrorStatus::TooManyRequests => Ok(HttpResponse::Ok().json(pokemon)),
            _ => Err(MyError::from(e)),
        },
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};
    use poke_api::api::PokeApi;

    use crate::server::app_handler::config_app;

    #[actix_web::test]
    async fn get_pokemon() {
        let app = test::init_service(App::new().configure(config_app)).await;

        let req = test::TestRequest::get()
            .uri("/pokemon/mewtwo")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{}", resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn get_translated_pokemon() {
        let app = test::init_service(App::new().configure(config_app)).await;
        let req = test::TestRequest::get()
            .uri("/pokemon/translated/mewtwo")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn get_pokemon_wrong_name() {
        let app = test::init_service(App::new().configure(config_app)).await;
        let req = test::TestRequest::get()
            .uri("/pokemon/metwo")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn get_pokemon_translated_wrong_name() {
        let app = test::init_service(App::new().configure(config_app)).await;
        let req = test::TestRequest::get()
            .uri("/pokemon/translated/metwo")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[tokio::test]
    async fn uppercase_get_pokemon() {
        let poke_api = PokeApi::new();
        let res = poke_api.search("Mewtwo").await;
        assert_eq!(res.is_ok(), true)
    }

    #[tokio::test]
    async fn max_name_length_get_pokemon() {
        let poke_api = PokeApi::new();
        let res = poke_api.search("mewtwotestcasemaxlengthof24char").await;
        assert_eq!(res.is_err(), true)
    }
}
