use utoipa::openapi::security::{
    ApiKey, ApiKeyValue, Flow, Implicit, OAuth2, Scopes, SecurityScheme,
};
use utoipa::{Modify, OpenApi};

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
        crate::controller::client_controller::list,
        crate::controller::client_controller::by_id,
        crate::controller::client_controller::delete,
        crate::controller::signing_request_controller::by_client_id,
        crate::controller::signing_request_controller::get_all,
        crate::generate_client
    ),
    components(
        schemas(shared::model::new_signing_request_dto::NewSigningRequestDto),
        schemas(crate::model::error_dto::ErrorDto),
        schemas(shared::model::health_info_dto::HealthInfoDto),
        schemas(
            crate::model::user_dto::UserDto,
            crate::model::create_user_dto::CreateUserDto
        ),
        schemas(
            crate::model::client_dto::ClientDto,
            crate::model::create_client_dto::CreateClientDto
        ),
        schemas(crate::ClientCert),
        schemas(shared::model::signing_request_dto::SigningRequestDto),
    ),
    tags(
        (name = "Certificates", description = "Certificate endpoints"),
        (name = "Common", description = "Common endpoints"),
        (name = "Users", description = "User endpoints"),
        (name = "Clients", description = "Client endpoints"),
        (name = "Signing requests", description = "Signing Request endpoints")
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
            "oauth2",
            SecurityScheme::OAuth2(OAuth2::new(Some(Flow::Implicit(Implicit::new(
                "http://localhost:8090/realms/ca/protocol/openid-connect/auth",
                Scopes::new(),
            ))))),
        );
        components.add_security_scheme(
            "jwt",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
    }
}
