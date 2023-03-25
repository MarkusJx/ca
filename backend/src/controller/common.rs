use crate::config::app_state::AppState;
use crate::register_module;
use crate::util::types::WebResult;
use actix_web::web::Json;
use actix_web::{get, web};
use shared::model::health_info_dto::HealthInfoDto;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[utoipa::path(
    get,
    tag = "Common",
    context_path = "/api/v1",
    operation_id = "healthCheck",
    responses(
        (status = 200, description = "Ok", body = HealthInfoDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[get("/health")]
async fn health_check(data: web::Data<AppState>) -> WebResult<Json<HealthInfoDto>> {
    let info = data.keycloak_service.get_server_info().await.ok();

    Ok(Json(HealthInfoDto {
        version: VERSION.to_string(),
        keycloak_version: info.and_then(|i| i.system_info).and_then(|i| i.version),
        status: "OK".into(),
        ok: true,
        is_initialized: Some(
            data.root_certificate_service.find_active().await?.is_some()
                && data.certificate_service.find_active().await?.is_some(),
        ),
    }))
}

register_module!(health_check);
