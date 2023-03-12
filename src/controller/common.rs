use crate::model::health_info_dto::HealthInfoDto;
use crate::register_module;
use actix_web::{get, web, Responder};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[utoipa::path(
    get,
    tag = "Common",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Ok", body = HealthInfoDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[get("/healthcheck")]
async fn health_check() -> impl Responder {
    web::Json(HealthInfoDto {
        version: VERSION.to_string(),
        status: "OK".to_string(),
    })
}

register_module!(health_check);
