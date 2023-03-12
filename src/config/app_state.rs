use crate::config::config::Config;
use crate::service::client_service::ClientService;
use crate::service::keycloak_service::KeycloakService;
use crate::service::signing_request_service::SigningRequestService;
use crate::service::user_service::UserService;

pub struct AppState {
    pub config: Config,
    pub keycloak_service: KeycloakService,
    pub client_service: ClientService,
    pub user_service: UserService,
    pub signing_request_service: SigningRequestService,
}
