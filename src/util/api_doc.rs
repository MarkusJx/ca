use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controller::certificate::ca_certificate,
        crate::controller::certificate::sign,
        crate::controller::common::health_check,
        crate::controller::user::create,
        crate::generate_client
    ),
    components(
        schemas(crate::models::signing_request::SigningRequest),
        schemas(crate::models::error_dto::ErrorDto),
        schemas(crate::models::health_info_dto::HealthInfoDto),
        schemas(crate::models::user_dto::UserDto),
        schemas(crate::ClientCert)
    ),
    tags(
        (name = "Certificates", description = "Certificate endpoints"),
        (name = "Common", description = "Common endpoints"),
        (name = "Users", description = "User endpoints")
    ),
    info(
        title = "Certificate Authority API",
        description = "A simple API for managing certificates",
        version = "0.0.1",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
    )
)]
pub struct ApiDoc;
