use ca_backend::entity::user;
use ca_backend::service::keycloak_service::{KeycloakService, MockKeycloakService};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{DatabaseBackend, MockDatabase};
use serde::{Deserialize, Serialize};

pub struct TestInitData {
    pub db: MockDatabase,
    pub kc: Box<dyn KeycloakService>,
}

impl Default for TestInitData {
    fn default() -> Self {
        let db = MockDatabase::new(DatabaseBackend::Postgres);
        let kc = Box::new(MockKeycloakService::new());
        Self { db, kc }
    }
}

const PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgR+h47OtUoz6wIAMB
WlmWx4Po7hmcrFaFa8ud4C3PYGChRANCAARZstJw8BpJg0SQ5f6GltzBTcg68eVt
khtsfvVErcsvhY27Ykk5dhoI1tM4xYaatR8uN07dCuJ3wu7cmCP3hmlM
-----END PRIVATE KEY-----";

pub fn encode_keycloak_token(
    subject: &uuid::Uuid,
    name: &str,
    email: &str,
    roles: &[&str],
) -> String {
    #[derive(Debug, Serialize, Deserialize)]
    struct RealmAccess {
        roles: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        iss: String,
        sub: String,
        aud: String,
        exp: i64,
        iat: i64,
        jti: String,
        typ: String,
        azp: String,
        session_state: String,
        acr: String,
        allowed_origins: Vec<String>,
        realm_access: RealmAccess,
        scope: String,
        email_verified: bool,
        name: String,
        email: String,
    }

    let key = EncodingKey::from_ec_pem(PRIVATE_KEY.as_bytes()).unwrap();
    format!(
        "Bearer {}",
        encode(
            &Header {
                alg: Algorithm::ES256,
                ..Default::default()
            },
            &Claims {
                iss: "http://localhost:8080/auth/realms/ca".to_string(),
                sub: subject.to_string(),
                aud: "ca".to_string(),
                exp: 1_000_000_000_000,
                iat: 1_000_000_000_000,
                jti: uuid::Uuid::new_v4().to_string(),
                typ: "Bearer".to_string(),
                azp: "ca".to_string(),
                session_state: uuid::Uuid::new_v4().to_string(),
                acr: "test".to_string(),
                allowed_origins: vec!["http://localhost:8080".to_string()],
                realm_access: RealmAccess {
                    roles: roles.into_iter().map(|r| r.to_string()).collect(),
                },
                scope: "test".to_string(),
                email_verified: true,
                name: name.to_string(),
                email: email.to_string(),
            },
            &key,
        )
        .unwrap()
    )
}

pub fn create_user(id: &uuid::Uuid, name: &str) -> user::Model {
    let now: DateTimeWithTimeZone = chrono::Utc::now().into();
    user::Model {
        name: name.into(),
        active: true,
        id: id.clone(),
        original_name: name.into(),
        created_at: now.clone(),
        updated_at: now.clone(),
        external_id: Some(id.to_string()),
    }
}

pub fn create_now() -> DateTimeWithTimeZone {
    chrono::Utc::now().into()
}

#[macro_export]
macro_rules! module {
    ($module: expr) => {
        actix_web::web::scope("/api/v1").module($module)
    };
}

#[macro_export]
macro_rules! scope {
    ($method: expr) => {
        actix_web::web::scope("/api/v1").service($method)
    };
}

#[macro_export]
macro_rules! init_test {
    ($app: ident, $module: expr) => {
        init_test!($app, $module, TestInitData::default());
    };
    ($app: ident, $module: expr, $data: expr) => {
        use crate::controller::helpers::TestInitData;
        shared::util::logger::init_logger(log::LevelFilter::Debug).unwrap();

        ca_backend::middleware::keycloak_middleware::set_keycloak_public_key(
            "-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEWbLScPAaSYNEkOX+hpbcwU3IOvHl
bZIbbH71RK3LL4WNu2JJOXYaCNbTOMWGmrUfLjdO3Qrid8Lu3Jgj94ZpTA==
-----END PUBLIC KEY-----".into(),
            ca_backend::middleware::keycloak_middleware::KeyType::ECDSA,
        ).unwrap();

        let data = $data;
        let db = std::sync::Arc::new(data.db.into_connection());
        let state = ca_backend::config::app_state::AppState {
            client_service: ca_backend::service::client_service::ClientService::new(db.clone()),
            user_service: ca_backend::service::user_service::UserService::new(db.clone()),
            signing_request_service: ca_backend::service::signing_request_service::SigningRequestService::new(db.clone()),
            token_service: ca_backend::service::token_service::TokenService::new(db.clone()),
            certificate_service: ca_backend::service::certificate_service::CertificateService::new(db.clone()),
            root_certificate_service: ca_backend::service::root_certificate_service::RootCertificateService::new(db.clone()),
            config: ca_backend::config::config::Config {
                jwt_secret: "secret".into(),
                ..ca_backend::config::config::Config::init().unwrap()
            },
            keycloak_service: std::sync::Arc::new(data.kc),
        };

        #[allow(unused_imports)]
        use ca_backend::util::traits::register_module::RegisterModule;

        let module = $module;
        let $app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(state))
                .service(module)
        ).await;
    };
}
