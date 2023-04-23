#![allow(non_upper_case_globals)]

use crate::controller::helpers::{create_user, encode_keycloak_token};
use crate::{init_test, scope};
use actix_web::http::StatusCode;
use actix_web::test::{call_service, read_body_json, TestRequest};
use ca_backend::controller::certificate_controller;
use ca_backend::entity::{certificate, client, root_certificate, signing_request, token};
use ca_backend::model::ca_certificate_dto::CACertificateDto;
use ca_backend::model::generate_intermediate_dto::GenerateIntermediateDto;
use ca_backend::model::token_claims::TokenClaims;
use chrono::{Days, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use lazy_static::lazy_static;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use shared::model::new_signing_request_dto::NewSigningRequestDto;
use shared::model::signing_request_dto::SigningRequestDto;
use uuid::Uuid;

const ROOT_CERT: &str = "-----BEGIN CERTIFICATE-----
MIIB+TCCAZ+gAwIBAgITK6wu3NyqstyKN74dy8XM51J4VTAKBggqhkjOPQQDAjBb
MQswCQYDVQQGEwJERTEPMA0GA1UECAwGQmVybGluMQ8wDQYDVQQHDAZCZXJsaW4x
CzAJBgNVBAoMAkNBMQswCQYDVQQLDAJDQTEQMA4GA1UEAwwHQ0EgUm9vdDAeFw0y
MzA0MjMxMzI5NTFaFw0yOTA0MjExMzI5NTFaMFsxCzAJBgNVBAYTAkRFMQ8wDQYD
VQQIDAZCZXJsaW4xDzANBgNVBAcMBkJlcmxpbjELMAkGA1UECgwCQ0ExCzAJBgNV
BAsMAkNBMRAwDgYDVQQDDAdDQSBSb290MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcD
QgAEzEsQxr6PkOONpAMawuaF+8pAKBbqXun5rXDVOk+QWscg0rGUpHVY7RVYAuUN
/EWtbpJvyOkrxSjrV+BDlCWL1aNCMEAwDwYDVR0TAQH/BAUwAwEB/zAOBgNVHQ8B
Af8EBAMCAYYwHQYDVR0OBBYEFJM5vAS+EvMFbz/Qft4VhoqX5XoUMAoGCCqGSM49
BAMCA0gAMEUCICUzOcGKJozXUaUSlV0k00pTucfYYAHSmZYgQeh0pRbZAiEAqvwl
b2wkpF3NOFx8RgCtG12VzMY6pv3JPPDPrMrGNjU=
-----END CERTIFICATE-----";

const ROOT_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgc6uDisLDznNzTqKM
LBmCJ36U6FWaxuI7RPj8hwZ47AihRANCAATMSxDGvo+Q442kAxrC5oX7ykAoFupe
6fmtcNU6T5BaxyDSsZSkdVjtFVgC5Q38Ra1ukm/I6SvFKOtX4EOUJYvV
-----END PRIVATE KEY-----";

const CSR: &str = "-----BEGIN CERTIFICATE REQUEST-----
MIIBBDCBqwIBADBJMQswCQYDVQQGEwJHQjENMAsGA1UECAwEdGVzdDENMAsGA1UE
BwwEdGVzdDENMAsGA1UECgwEdGVzdDENMAsGA1UEAwwEdGVzdDBZMBMGByqGSM49
AgEGCCqGSM49AwEHA0IABEnVTPW2pTKE0iaF3hm4NzH2Puxj+yMAM5jMBaTGlEcj
xvz96NzFM0F/4Na3w/7h2PLGBSzSzq1GG1oLr/IRz5ygADAKBggqhkjOPQQDAgNI
ADBFAiA8fv5MccIGHkjqCAf0PVBiYwUNRCZbqh8TA4IjhvWvqAIhAKAM6q7djsaX
/dn7gG6rXawptg0EUEQay9dTLH0zdcvR
-----END CERTIFICATE REQUEST-----";

lazy_static! {
    static ref now: DateTimeWithTimeZone = chrono::Utc::now().into();
}

fn create_token(client_id: &Uuid, secret: Option<&str>) -> String {
    format!(
        "Bearer {}",
        encode(
            &Header::default(),
            &TokenClaims {
                sub: client_id.to_string(),
                iat: now.timestamp() as usize,
                exp: Utc::now()
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .timestamp() as usize,
            },
            &EncodingKey::from_secret(secret.unwrap_or("secret").as_bytes()),
        )
        .unwrap()
    )
}

#[actix_web::test]
async fn test_get_intermediate() {
    let db =
        MockDatabase::new(DatabaseBackend::Postgres).append_query_results([[certificate::Model {
            id: 1,
            active: true,
            valid_until: now.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            private: None,
            public: "public".into(),
        }]]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri("/api/v1/certificate/intermediate")
        .to_request();

    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: CACertificateDto = read_body_json(res).await;
    assert_eq!(body.certificate, "public");
    assert_eq!(body.root, false);
    assert_eq!(body.valid_until, now.to_rfc3339());
    assert_eq!(body.created_at, now.to_rfc3339());
    assert_eq!(body.private_key, None);
}

#[actix_web::test]
async fn test_get_intermediate_no_intermediate() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([Vec::<certificate::Model>::new()]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri("/api/v1/certificate/intermediate")
        .to_request();

    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NOT_FOUND, res.status());
}

#[actix_web::test]
async fn test_get_root_cert() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).append_query_results([[
        root_certificate::Model {
            id: 1,
            active: true,
            valid_until: now.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            created_by: Uuid::new_v4(),
            public: ROOT_CERT.into(),
        },
    ]]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri("/api/v1/certificate/root")
        .to_request();

    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: CACertificateDto = read_body_json(res).await;
    assert_eq!(body.certificate, ROOT_CERT);
    assert_eq!(body.root, true);
    assert_eq!(body.valid_until, now.to_rfc3339());
    assert_eq!(body.created_at, now.to_rfc3339());
    assert_eq!(body.private_key, None);
}

#[actix_web::test]
async fn test_get_root_cert_no_root() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([Vec::<root_certificate::Model>::new()]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri("/api/v1/certificate/root")
        .to_request();

    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NOT_FOUND, res.status());
}

#[actix_web::test]
async fn test_generate_intermediate() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[root_certificate::Model {
            id: 1,
            active: true,
            valid_until: now.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            created_by: Uuid::new_v4(),
            public: ROOT_CERT.into(),
        }]])
        .append_query_results([Vec::<certificate::Model>::new()])
        .append_query_results([[certificate::Model {
            id: 1,
            active: true,
            valid_until: now.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            private: Some("private".into()),
            public: "public".into(),
        }]])
        .append_exec_results([MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let token = encode_keycloak_token(&user_id, "test", "test", &["admin"]);
    let req = TestRequest::post()
        .uri("/api/v1/certificate/intermediate/generate")
        .append_header(("Authorization", token))
        .set_json(GenerateIntermediateDto {
            root_certificate: ROOT_PRIVATE_KEY.into(),
        })
        .to_request();

    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: CACertificateDto = read_body_json(res).await;
    assert_eq!(body.certificate, "public");
    assert_eq!(body.private_key, None);
    assert_eq!(body.root, false);
    assert_eq!(body.valid_until, now.to_rfc3339());
    assert_eq!(body.created_at, now.to_rfc3339());
}

#[actix_web::test]
async fn test_generate_intermediate_invalid_private_key() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[root_certificate::Model {
            id: 1,
            active: true,
            valid_until: now.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            created_by: Uuid::new_v4(),
            public: ROOT_CERT.into(),
        }]]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let token = encode_keycloak_token(&user_id, "test", "test", &["admin"]);
    let req = TestRequest::post()
        .uri("/api/v1/certificate/intermediate/generate")
        .append_header(("Authorization", token))
        .set_json(GenerateIntermediateDto {
            root_certificate: "ROOT_PRIVATE_KEY".into(),
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::BAD_REQUEST, res.status());
}

#[actix_web::test]
async fn test_generate_intermediate_no_root() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([Vec::<root_certificate::Model>::new()]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let token = encode_keycloak_token(&user_id, "test", "test", &["admin"]);
    let req = TestRequest::post()
        .uri("/api/v1/certificate/intermediate/generate")
        .append_header(("Authorization", token))
        .set_json(GenerateIntermediateDto {
            root_certificate: ROOT_PRIVATE_KEY.into(),
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::NOT_FOUND, res.status());
}

#[actix_web::test]
async fn test_generate_intermediate_insufficient_privileges() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[root_certificate::Model {
            id: 1,
            active: true,
            valid_until: now.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            created_by: Uuid::new_v4(),
            public: ROOT_CERT.into(),
        }]]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let token = encode_keycloak_token(&user_id, "test", "test", &["user"]);
    let req = TestRequest::post()
        .uri("/api/v1/certificate/intermediate/generate")
        .append_header(("Authorization", token))
        .set_json(GenerateIntermediateDto {
            root_certificate: ROOT_PRIVATE_KEY.into(),
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_generate_root_cert() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([Vec::<root_certificate::Model>::new()])
        .append_query_results([[root_certificate::Model {
            id: 1,
            public: "public".into(),
            active: true,
            created_by: user_id.clone(),
            valid_until: now.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
        }]])
        .append_exec_results([MockExecResult {
            rows_affected: 1,
            last_insert_id: 1,
        }]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::post()
        .uri("/api/v1/certificate/root/generate")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["admin"]),
        ))
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::OK, res.status());
    let body: CACertificateDto = read_body_json(res).await;
    assert!(body.root);
    assert!(body.private_key.is_some());
    assert!(body
        .private_key
        .unwrap()
        .starts_with("-----BEGIN PRIVATE KEY-----"));
    assert_eq!("public", body.certificate);
    assert_eq!(now.to_rfc3339(), body.created_at);
    assert_eq!(now.to_rfc3339(), body.valid_until);
}

#[actix_web::test]
async fn test_generate_root_cert_insufficient_privileges() {
    init_test!(
        app,
        scope!(certificate_controller::register())
    );

    let req = TestRequest::post()
        .uri("/api/v1/certificate/root/generate")
        .append_header((
            "Authorization",
            encode_keycloak_token(&Uuid::new_v4(), "test", "test", &["user"]),
        ))
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_sign() {
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[token::Model {
            id: client_id.clone(),
            active: true,
            token_hash: "".to_string(),
            client_id: client_id.clone(),
        }]])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id: Default::default(),
            name: "test".to_string(),
            original_name: "test".to_string(),
            active: true,
            is_user_client: false,
            valid_until: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        }]])
        .append_query_results([[certificate::Model {
            id: 0,
            public: ROOT_CERT.into(),
            private: Some(ROOT_PRIVATE_KEY.into()),
            active: true,
            valid_until: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]])
        .append_query_results([[signing_request::Model {
            id: 1,
            client_id,
            hash: "hash".into(),
            serial_number: "serial".into(),
            subject_name: "subject".into(),
            issued_at: now.clone(),
        }]])
        .append_exec_results([MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::post()
        .uri("/api/v1/certificate/sign")
        .append_header(("Authorization", create_token(&client_id, None)))
        .set_json(NewSigningRequestDto {
            request: CSR.into(),
            alternative_names: None,
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::OK, res.status());
    let body: SigningRequestDto = read_body_json(res).await;
    assert!(body.certificate.is_some());
    assert_eq!(client_id.to_string(), body.client_id);
    assert_eq!("hash", body.hash);
    assert_eq!("serial", body.serial_number);
    assert_eq!("subject", body.subject_name);
    assert_eq!(now.to_rfc3339(), body.issued_at);
}

#[actix_web::test]
async fn test_sign_invalid_csr() {
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[token::Model {
            id: client_id.clone(),
            active: true,
            token_hash: "".to_string(),
            client_id: client_id.clone(),
        }]])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id: Default::default(),
            name: "test".to_string(),
            original_name: "test".to_string(),
            active: true,
            is_user_client: false,
            valid_until: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        }]]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::post()
        .uri("/api/v1/certificate/sign")
        .append_header(("Authorization", create_token(&client_id, None)))
        .set_json(NewSigningRequestDto {
            request: "CSR".into(),
            alternative_names: None,
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::BAD_REQUEST, res.status());
}

#[actix_web::test]
async fn test_sign_invalid_token() {
    init_test!(app, scope!(certificate_controller::register()));

    let req = TestRequest::post()
        .uri("/api/v1/certificate/sign")
        .append_header((
            "Authorization",
            create_token(&Uuid::new_v4(), Some("secret123")),
        ))
        .set_json(NewSigningRequestDto {
            request: CSR.into(),
            alternative_names: None,
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_sign_no_active_token() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([Vec::<token::Model>::new()]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::post()
        .uri("/api/v1/certificate/sign")
        .append_header(("Authorization", create_token(&Uuid::new_v4(), None)))
        .set_json(NewSigningRequestDto {
            request: CSR.into(),
            alternative_names: None,
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_sign_no_ca_cert() {
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[token::Model {
            id: client_id.clone(),
            active: true,
            token_hash: "".to_string(),
            client_id: client_id.clone(),
        }]])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id: Default::default(),
            name: "test".to_string(),
            original_name: "test".to_string(),
            active: true,
            is_user_client: false,
            valid_until: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        }]])
        .append_query_results([Vec::<certificate::Model>::new()]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::post()
        .uri("/api/v1/certificate/sign")
        .append_header(("Authorization", create_token(&client_id, None)))
        .set_json(NewSigningRequestDto {
            request: CSR.into(),
            alternative_names: None,
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::BAD_REQUEST, res.status());
}

#[actix_web::test]
async fn test_user_sign() {
    let client_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id: user_id.clone(),
            name: "test".to_string(),
            original_name: "test".to_string(),
            active: true,
            is_user_client: false,
            valid_until: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        }]])
        .append_query_results([[certificate::Model {
            id: 0,
            public: ROOT_CERT.into(),
            private: Some(ROOT_PRIVATE_KEY.into()),
            active: true,
            valid_until: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]])
        .append_query_results([[signing_request::Model {
            id: 1,
            client_id,
            hash: "hash".into(),
            serial_number: "serial".into(),
            subject_name: "subject".into(),
            issued_at: now.clone(),
        }]])
        .append_exec_results([MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }]);

    init_test!(
        app,
        scope!(certificate_controller::register()),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::post()
        .uri("/api/v1/certificate/user-sign")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .set_json(NewSigningRequestDto {
            request: CSR.into(),
            alternative_names: None,
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::OK, res.status());
    let body: SigningRequestDto = read_body_json(res).await;
    assert!(body.certificate.is_some());
    assert_eq!(client_id.to_string(), body.client_id);
    assert_eq!("hash", body.hash);
    assert_eq!("serial", body.serial_number);
    assert_eq!("subject", body.subject_name);
    assert_eq!(now.to_rfc3339(), body.issued_at);
}

#[actix_web::test]
async fn test_user_sign_invalid_token() {
    init_test!(app, scope!(certificate_controller::register()));

    let req = TestRequest::post()
        .uri("/api/v1/certificate/user-sign")
        .append_header(("Authorization", create_token(&Uuid::new_v4(), None)))
        .set_json(NewSigningRequestDto {
            request: CSR.into(),
            alternative_names: None,
        })
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}
