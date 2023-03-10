use crate::register_module;
use actix_web::Error;
use lazy_static::lazy_static;
use paperclip::actix::web::{HttpRequest, HttpResponse};
use paperclip::actix::{api_v2_operation, get};
use std::sync::Arc;
use utoipa_swagger_ui::Config;

lazy_static! {
    static ref CONFIG: Arc<Config<'static>> = Arc::new(Config::from("/api/spec/v1"));
}

#[api_v2_operation]
#[get("/swagger-ui/{filename:.*}")]
async fn get_swagger_ui(req: HttpRequest) -> HttpResponse {
    let file = req.path()[12..].to_string();

    match utoipa_swagger_ui::serve(file.as_ref(), CONFIG.clone()) {
        Ok(swagger_file) => swagger_file
            .map(|file| {
                HttpResponse::Ok()
                    .content_type(file.content_type)
                    .body(file.bytes.to_vec())
            })
            .unwrap_or_else(|| HttpResponse::NotFound().finish()),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

register_module!(get_swagger_ui);
