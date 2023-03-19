use crate::config::app_state::AppState;
use crate::middleware::extractors::KeycloakUserClaims;
use crate::middleware::keycloak_middleware;
use crate::middleware::keycloak_roles::AdminRole;
use crate::register_module;
use crate::util::types::WebResult;
use actix_web::get;
use actix_web::web::{Data, Json};

#[utoipa::path(
    get,
    tag = "Admin",
    context_path = "/api/v1",
    operation_id = "listRoles",
    responses(
        (status = 200, description = "Ok", body = Vec<String>),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[get("/admin/roles", wrap = "keycloak_middleware::Keycloak")]
async fn list_roles(
    data: Data<AppState>,
    _claims: KeycloakUserClaims<AdminRole>,
) -> WebResult<Json<Vec<String>>> {
    Ok(Json(
        data.keycloak_service
            .get_roles()
            .await?
            .into_iter()
            .filter(|r| {
                !r.starts_with("default-roles-")
                    && r != "uma_authorization"
                    && r != "offline_access"
            })
            .collect(),
    ))
}

register_module!(list_roles);
