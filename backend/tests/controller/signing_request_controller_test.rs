use crate::controller::helpers::{create_now, create_user, encode_keycloak_token};
use crate::{init_test, module};
use actix_web::http::StatusCode;
use actix_web::test::{call_service, read_body_json, TestRequest};
use ca_backend::controller::signing_request_controller;
use ca_backend::entity::{client, signing_request};
use ca_backend::model::error_dto::ErrorDto;
use chrono::Days;
use sea_orm::{DatabaseBackend, MockDatabase};
use shared::model::signing_request_dto::SigningRequestDto;
use uuid::Uuid;

#[actix_web::test]
async fn test_by_client_id() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let now = create_now();
    let valid_until = now.checked_add_days(Days::new(1)).unwrap();

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            user_id,
            is_user_client: false,
            name: "test".to_string(),
            id: client_id.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            active: true,
            original_name: "original".to_string(),
            valid_until: Some(valid_until.clone()),
        }]])
        .append_query_results([[signing_request::Model {
            client_id,
            id: 1,
            hash: "hash".to_string(),
            subject_name: "subject_name".to_string(),
            issued_at: now.clone(),
            serial_number: "serial_number".to_string(),
        }]]);

    init_test!(
        app,
        module!(signing_request_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri(&format!("/api/v1/signing-request/{}", client_id))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: Vec<SigningRequestDto> = read_body_json(res).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].client_id, client_id.to_string());
    assert_eq!(body[0].hash, "hash");
    assert_eq!(body[0].subject_name, "subject_name");
    assert_eq!(body[0].issued_at, now.to_rfc3339());
    assert_eq!(body[0].serial_number, "serial_number");
}

#[actix_web::test]
async fn test_by_client_id_unauthorized() {
    let client_id = Uuid::new_v4();

    init_test!(app, module!(signing_request_controller::module));

    let req = TestRequest::get()
        .uri(&format!("/api/v1/signing-request/{}", client_id))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_by_client_id_invalid_id() {
    let user_id = Uuid::new_v4();

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]]);

    init_test!(
        app,
        module!(signing_request_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri("/api/v1/signing-request/invalid")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::BAD_REQUEST, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(body.message.unwrap(), "Invalid client id supplied");
    assert_eq!(body.code, 400);
}

#[actix_web::test]
async fn test_by_client_id_invalid_client() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([Vec::<client::Model>::new()]);

    init_test!(
        app,
        module!(signing_request_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri(&format!("/api/v1/signing-request/{}", client_id))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NOT_FOUND, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(body.message.unwrap(), "Client not found");
    assert_eq!(body.code, 404);
}

#[actix_web::test]
async fn test_by_client_id_mismatching_ids() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let now = create_now();
    let valid_until = now.checked_add_days(Days::new(1)).unwrap();

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            user_id: Uuid::new_v4(),
            is_user_client: false,
            name: "test".to_string(),
            id: client_id.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            active: true,
            original_name: "original".to_string(),
            valid_until: Some(valid_until.clone()),
        }]]);

    init_test!(
        app,
        module!(signing_request_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri(&format!("/api/v1/signing-request/{}", client_id))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(
        body.message.unwrap(),
        "User is not allowed to access this resource"
    );
    assert_eq!(body.code, 401);
}

#[actix_web::test]
async fn test_get_all() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let now = create_now();

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[signing_request::Model {
            client_id,
            id: 1,
            hash: "hash".to_string(),
            subject_name: "subject_name".to_string(),
            issued_at: now.clone(),
            serial_number: "serial_number".to_string(),
        }]]);

    init_test!(
        app,
        module!(signing_request_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri("/api/v1/signing-request")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: Vec<SigningRequestDto> = read_body_json(res).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].client_id, client_id.to_string());
    assert_eq!(body[0].hash, "hash");
    assert_eq!(body[0].subject_name, "subject_name");
    assert_eq!(body[0].issued_at, now.to_rfc3339());
    assert_eq!(body[0].serial_number, "serial_number");
}

#[actix_web::test]
async fn test_get_all_unauthorized() {
    init_test!(app, module!(signing_request_controller::module));

    let req = TestRequest::get()
        .uri("/api/v1/signing-request")
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}
