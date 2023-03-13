use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{OpenIdConnect, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controller::certificate::ca_certificate,
        crate::controller::certificate::sign,
        crate::controller::common::health_check,
        crate::controller::user_controller::create,
        crate::controller::user_controller::list,
        crate::controller::user_controller::get,
        crate::controller::user_controller::delete,
        crate::controller::user_controller::by_name,
        crate::controller::client_controller::create,
        crate::generate_client
    ),
    components(
        schemas(crate::model::signing_request::SigningRequest, ),
        schemas(crate::model::error_dto::ErrorDto),
        schemas(crate::model::health_info_dto::HealthInfoDto),
        schemas(
            crate::model::user_dto::UserDto,
            crate::model::create_user_dto::CreateUserDto
        ),
        schemas(
            crate::model::client_dto::ClientDto,
            crate::model::create_client_dto::CreateClientDto
        ),
        schemas(crate::ClientCert)
    ),
    tags(
        (name = "Certificates", description = "Certificate endpoints"),
        (name = "Common", description = "Common endpoints"),
        (name = "Users", description = "User endpoints"),
        (name = "Clients", description = "Client endpoints")
    ),
    info(
        title = "Certificate Authority API",
        description = "A simple API for managing certificates",
        version = "0.0.1",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "api_key",
            SecurityScheme::OpenIdConnect(OpenIdConnect::new("http://localhost:8090/realms/ca/.well-known/openid-configuration")),
        )
    }
}
