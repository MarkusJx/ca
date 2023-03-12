use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controller::certificate::ca_certificate,
        crate::controller::certificate::sign,
        crate::controller::common::health_check,
        crate::controller::user::create,
        crate::controller::user::list,
        crate::controller::user::get,
        crate::controller::user::delete,
        crate::generate_client
    ),
    components(
        schemas(crate::model::signing_request::SigningRequest),
        schemas(crate::model::error_dto::ErrorDto),
        schemas(crate::model::health_info_dto::HealthInfoDto),
        schemas(crate::model::user_dto::UserDto),
        schemas(crate::model::create_user_dto::CreateUserDto),
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
