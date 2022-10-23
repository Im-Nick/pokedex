pub mod pokemon;

pub mod index {
    use crate::errors::MyError;
    use actix_web::{get, HttpResponse};

    #[get("")]
    pub(crate) async fn health_check() -> Result<HttpResponse, MyError> {
        Ok(HttpResponse::Ok().finish())
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use crate::server::app_handler::config_app;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().configure(config_app)).await;
        let req = test::TestRequest::get()
            .uri("/")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
