use crate::controller::helpers::{create_now, create_user, encode_keycloak_token};
use crate::{init_test, module};
use actix_web::test::{call_service, read_body_json, TestRequest};
use ca_backend::controller::client_controller;
use ca_backend::entity::{client, signing_request, token};
use ca_backend::model::client_dto::ClientDto;
use ca_backend::model::create_client_dto::CreateClientDto;
use ca_backend::model::error_dto::ErrorDto;
use ca_backend::model::token_claims::TokenClaims;
use chrono::Days;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use openssl::sha::Sha256;
use reqwest::StatusCode;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use shared::util::traits::u8_vec_to_string::U8VecToString;
use uuid::Uuid;

const DATE_FMT: &str = "%d.%m.%Y %H:%M:%S";

#[actix_web::test]
async fn test_create() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([Vec::<client::Model>::new()])
        .append_query_results([Vec::<token::Model>::new()])
        .append_query_results([Vec::<client::Model>::new()])
        .append_query_results([[client::Model {
            id: client_id,
            name: "test".into(),
            original_name: "original".into(),
            active: true,
            is_user_client: false,
            valid_until: create_now().checked_add_days(Days::new(1)),
            created_at: create_now(),
            updated_at: create_now(),
            user_id: user_id.clone(),
        }]])
        .append_exec_results([MockExecResult {
            rows_affected: 1,
            last_insert_id: 1,
        }])
        .append_query_results([Vec::<token::Model>::new()])
        .append_query_results([[token::Model {
            id: Default::default(),
            client_id: Default::default(),
            token_hash: "hash".to_string(),
            active: true,
        }]])
        .append_exec_results([MockExecResult {
            rows_affected: 1,
            last_insert_id: 1,
        }]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let valid_until = create_now().checked_add_days(Days::new(1)).unwrap();
    let req = TestRequest::post()
        .uri("/api/v1/client")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .set_json(CreateClientDto {
            valid_until: valid_until.to_rfc3339(),
            name: Some("test".into()),
        })
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: ClientDto = read_body_json(res).await;
    assert_eq!(body.id, client_id.to_string());
    assert_eq!(body.name, "test");
    assert_eq!(body.active, true);
    assert_eq!(body.display_name, "original");
    assert_eq!(body.user_id, user_id.to_string());
    assert!(body.token.is_some());
    assert_eq!(
        jsonwebtoken::decode::<TokenClaims>(
            body.token.unwrap().as_str(),
            &DecodingKey::from_secret("secret".as_bytes()),
            &Validation::new(Algorithm::HS256)
        )
        .err(),
        None
    );
    assert_eq!(body.token_hash, "hash");
    assert_eq!(
        DateTimeWithTimeZone::parse_from_rfc3339(&body.valid_until.unwrap())
            .unwrap()
            .format(DATE_FMT)
            .to_string(),
        valid_until.format(DATE_FMT).to_string()
    );
}

#[actix_web::test]
async fn test_create_invalid_token() {
    init_test!(app, module!(client_controller::module));

    let valid_until = create_now().checked_add_days(Days::new(1)).unwrap();
    let req = TestRequest::post()
        .uri("/api/v1/client")
        .append_header(("Authorization", "Bearer token"))
        .set_json(CreateClientDto {
            valid_until: valid_until.to_rfc3339(),
            name: Some("test".into()),
        })
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_create_client_already_exists() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: Default::default(),
            user_id,
            name: "".to_string(),
            original_name: "".to_string(),
            active: false,
            is_user_client: false,
            valid_until: None,
            created_at: Default::default(),
            updated_at: Default::default(),
        }]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let valid_until = create_now().checked_add_days(Days::new(1)).unwrap();
    let req = TestRequest::post()
        .uri("/api/v1/client")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .set_json(CreateClientDto {
            valid_until: valid_until.to_rfc3339(),
            name: Some("test".into()),
        })
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::BAD_REQUEST, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(
        body.message.unwrap(),
        "A client with that name already exists"
    );
    assert_eq!(body.code, 400);
}

#[actix_web::test]
async fn test_create_no_name_supplied() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let valid_until = create_now().checked_add_days(Days::new(1)).unwrap();
    let req = TestRequest::post()
        .uri("/api/v1/client")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .set_json(CreateClientDto {
            valid_until: valid_until.to_rfc3339(),
            name: None,
        })
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::BAD_REQUEST, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(body.message.unwrap(), "Client name must be supplied");
    assert_eq!(body.code, 400);
}

#[actix_web::test]
async fn test_create_invalid_expiry() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([Vec::<client::Model>::new()]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let valid_until = create_now().checked_sub_days(Days::new(1)).unwrap();
    let req = TestRequest::post()
        .uri("/api/v1/client")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .set_json(CreateClientDto {
            valid_until: valid_until.to_rfc3339(),
            name: Some("test".into()),
        })
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::BAD_REQUEST, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(
        body.message.unwrap(),
        "The expiry date must be in the future"
    );
    assert_eq!(body.code, 400);
}

#[actix_web::test]
async fn test_regenerate_token() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let valid_until = create_now().checked_add_days(Days::new(1)).unwrap();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id,
            name: "test".to_string(),
            original_name: "original".to_string(),
            active: true,
            is_user_client: false,
            valid_until: Some(create_now()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]])
        .append_query_results([Vec::<token::Model>::new()])
        .append_query_results([Vec::<token::Model>::new()])
        .append_query_results([Vec::<token::Model>::new()])
        .append_query_results([[token::Model {
            id: Default::default(),
            client_id: Default::default(),
            token_hash: "hash".to_string(),
            active: true,
        }]])
        .append_exec_results([MockExecResult {
            rows_affected: 1,
            last_insert_id: 1,
        }])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id,
            name: "test".to_string(),
            original_name: "original".to_string(),
            active: true,
            is_user_client: false,
            valid_until: Some(valid_until.clone()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]])
        .append_exec_results([MockExecResult {
            rows_affected: 1,
            last_insert_id: 1,
        }]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::put()
        .uri(&format!(
            "/api/v1/client/regenerate/{}",
            client_id.to_string()
        ))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .set_json(CreateClientDto {
            valid_until: valid_until.to_rfc3339(),
            name: Some("test2".into()),
        })
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: ClientDto = read_body_json(res).await;
    assert_eq!(body.name, "test");
    assert_eq!(body.display_name, "original");
    assert_eq!(body.id, client_id.to_string());
    assert_eq!(body.user_id, user_id.to_string());
    assert!(body.token.is_some());
    assert_eq!(
        jsonwebtoken::decode::<TokenClaims>(
            body.token.unwrap().as_str(),
            &DecodingKey::from_secret("secret".as_bytes()),
            &Validation::new(Algorithm::HS256)
        )
        .err(),
        None
    );
    assert_eq!(body.token_hash, "hash");
    assert_eq!(
        DateTimeWithTimeZone::parse_from_rfc3339(&body.valid_until.unwrap())
            .unwrap()
            .format(DATE_FMT)
            .to_string(),
        valid_until.format(DATE_FMT).to_string()
    );
}

#[actix_web::test]
async fn test_regenerate_token_unauthorized() {
    init_test!(app, module!(client_controller::module));

    let req = TestRequest::put()
        .uri(&format!(
            "/api/v1/client/regenerate/{}",
            Uuid::new_v4().to_string()
        ))
        .append_header(("Authorization", "Bearer token"))
        .set_json(CreateClientDto {
            valid_until: create_now().to_rfc3339(),
            name: Some("test2".into()),
        })
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_regenerate_token_client_not_found() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let valid_until = create_now().checked_add_days(Days::new(1)).unwrap();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([Vec::<client::Model>::new()]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::put()
        .uri(&format!(
            "/api/v1/client/regenerate/{}",
            client_id.to_string()
        ))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .set_json(CreateClientDto {
            valid_until: valid_until.to_rfc3339(),
            name: Some("test2".into()),
        })
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NOT_FOUND, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(body.message.unwrap(), "Client not found");
    assert_eq!(body.code, 404);
}

#[actix_web::test]
async fn test_regenerate_token_no_access() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let valid_until = create_now().checked_add_days(Days::new(1)).unwrap();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id: Uuid::new_v4(),
            name: "test".to_string(),
            original_name: "original".to_string(),
            active: true,
            is_user_client: false,
            valid_until: Some(create_now()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::put()
        .uri(&format!(
            "/api/v1/client/regenerate/{}",
            client_id.to_string()
        ))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .set_json(CreateClientDto {
            valid_until: valid_until.to_rfc3339(),
            name: Some("test2".into()),
        })
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NOT_FOUND, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(body.message.unwrap(), "User is not the owner of the client");
    assert_eq!(body.code, 404);
}

#[actix_web::test]
async fn test_by_id() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let now = create_now();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id,
            name: "test".to_string(),
            original_name: "original".to_string(),
            active: true,
            is_user_client: false,
            valid_until: Some(now.clone()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]])
        .append_query_results([[token::Model {
            id: Default::default(),
            client_id,
            token_hash: "hash".to_string(),
            active: true,
        }]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri(&format!("/api/v1/client/{}", client_id.to_string()))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: ClientDto = read_body_json(res).await;
    assert_eq!(body.id, client_id.to_string());
    assert_eq!(body.name, "test");
    assert_eq!(body.display_name, "original");
    assert_eq!(body.active, true);
    assert_eq!(body.valid_until, Some(now.to_rfc3339()));
    assert_eq!(body.token, None);
    assert_eq!(body.token_hash, "hash");
}

#[actix_web::test]
async fn test_by_id_unauthorized() {
    init_test!(app, module!(client_controller::module));

    let req = TestRequest::get()
        .uri(&format!("/api/v1/client/{}", Uuid::new_v4().to_string()))
        .append_header(("Authorization", "Bearer token"))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_by_id_no_client() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([Vec::<client::Model>::new()]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri(&format!("/api/v1/client/{}", Uuid::new_v4().to_string()))
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
async fn test_by_id_no_access() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id: Uuid::new_v4(),
            name: "test".to_string(),
            original_name: "original".to_string(),
            active: true,
            is_user_client: false,
            valid_until: None,
            created_at: Default::default(),
            updated_at: Default::default(),
        }]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri(&format!("/api/v1/client/{}", Uuid::new_v4().to_string()))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NOT_FOUND, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(
        body.message.unwrap(),
        "You are not authorized to access this client"
    );
    assert_eq!(body.code, 404);
}

#[actix_web::test]
async fn test_by_id_user_token() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let now = create_now();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id.clone(),
            user_id,
            name: "test".to_string(),
            original_name: "original".to_string(),
            active: true,
            is_user_client: true,
            valid_until: Some(now.clone()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri(&format!("/api/v1/client/{}", client_id.to_string()))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: ClientDto = read_body_json(res).await;
    assert_eq!(body.id, client_id.to_string());
    assert_eq!(body.name, "test");
    assert_eq!(body.display_name, "original");
    assert_eq!(body.active, true);
    assert_eq!(body.valid_until, Some(now.to_rfc3339()));
    assert_eq!(body.token, None);
    assert_eq!(body.token_hash, {
        let mut hash = Sha256::new();
        hash.update(user_id.as_bytes());
        hash.finish().to_vec().to_hex_string("")
    });
}

#[actix_web::test]
async fn test_list() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[
            client::Model {
                id: Default::default(),
                user_id,
                name: "client1".to_string(),
                original_name: "original1".to_string(),
                active: true,
                is_user_client: false,
                valid_until: None,
                created_at: Default::default(),
                updated_at: Default::default(),
            },
            client::Model {
                id: Default::default(),
                user_id,
                name: "client2".to_string(),
                original_name: "original2".to_string(),
                active: true,
                is_user_client: false,
                valid_until: None,
                created_at: Default::default(),
                updated_at: Default::default(),
            },
        ]])
        .append_query_results([
            [token::Model {
                id: Default::default(),
                client_id: Default::default(),
                token_hash: "hash1".to_string(),
                active: false,
            }],
            [token::Model {
                id: Default::default(),
                client_id: Default::default(),
                token_hash: "hash2".to_string(),
                active: false,
            }],
        ]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri("/api/v1/client/list")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: Vec<ClientDto> = read_body_json(res).await;
    assert_eq!(body.len(), 2);
    assert_eq!(body[0].name, "client1");
    assert_eq!(body[0].display_name, "original1");
    assert_eq!(body[0].active, true);
    assert_eq!(body[0].valid_until, None);
    assert_eq!(body[0].token, None);
    assert_eq!(body[0].token_hash, "hash1");
    assert_eq!(body[1].name, "client2");
    assert_eq!(body[1].display_name, "original2");
    assert_eq!(body[1].active, true);
    assert_eq!(body[1].valid_until, None);
    assert_eq!(body[1].token, None);
    assert_eq!(body[1].token_hash, "hash2");
}

#[actix_web::test]
async fn test_list_user_client() {
    let user_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: Default::default(),
            user_id,
            name: "client1".to_string(),
            original_name: "original1".to_string(),
            active: true,
            is_user_client: true,
            valid_until: None,
            created_at: Default::default(),
            updated_at: Default::default(),
        }]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::get()
        .uri("/api/v1/client/list")
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::OK, res.status());
    let body: Vec<ClientDto> = read_body_json(res).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].name, "client1");
    assert_eq!(body[0].display_name, "original1");
    assert_eq!(body[0].active, true);
    assert_eq!(body[0].valid_until, None);
    assert_eq!(body[0].token, None);
    assert_eq!(body[0].token_hash, {
        let mut hash = Sha256::new();
        hash.update(user_id.as_bytes());
        hash.finish().to_vec().to_hex_string("")
    });
}

#[actix_web::test]
async fn test_list_unauthorized() {
    init_test!(app, module!(client_controller::module));

    let req = TestRequest::get().uri("/api/v1/client/list").to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_delete() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id,
            user_id,
            name: "client1".to_string(),
            original_name: "original1".to_string(),
            active: true,
            is_user_client: false,
            valid_until: Some(create_now()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]])
        .append_query_results([Vec::<signing_request::Model>::new()])
        .append_query_results([Vec::<token::Model>::new()])
        .append_query_results([[client::Model {
            id: client_id,
            user_id,
            name: "client1".to_string(),
            original_name: "original1".to_string(),
            active: false,
            is_user_client: false,
            valid_until: None,
            created_at: Default::default(),
            updated_at: Default::default(),
        }]])
        .append_exec_results([MockExecResult {
            rows_affected: 1,
            last_insert_id: 1,
        }]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::delete()
        .uri(&format!("/api/v1/client/{}", client_id))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NO_CONTENT, res.status());
}

#[actix_web::test]
async fn test_delete_unauthorized() {
    let client_id = Uuid::new_v4();
    init_test!(app, module!(client_controller::module));

    let req = TestRequest::delete()
        .uri(&format!("/api/v1/client/{}", client_id))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::UNAUTHORIZED, res.status());
}

#[actix_web::test]
async fn test_delete_not_found() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([Vec::<client::Model>::new()]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::delete()
        .uri(&format!("/api/v1/client/{}", client_id))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NOT_FOUND, res.status());
}

#[actix_web::test]
async fn test_delete_user_client() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id,
            user_id,
            name: "client1".to_string(),
            original_name: "original1".to_string(),
            active: true,
            is_user_client: true,
            valid_until: Some(create_now()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::delete()
        .uri(&format!("/api/v1/client/{}", client_id))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::BAD_REQUEST, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(body.message.unwrap(), "User client cannot be deleted");
    assert_eq!(body.code, 400);
}

#[actix_web::test]
async fn test_delete_no_access() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id,
            user_id: Uuid::new_v4(),
            name: "client1".to_string(),
            original_name: "original1".to_string(),
            active: true,
            is_user_client: false,
            valid_until: Some(create_now()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::delete()
        .uri(&format!("/api/v1/client/{}", client_id))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NOT_FOUND, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(body.message.unwrap(), "This client does not belong to you");
    assert_eq!(body.code, 404);
}

#[actix_web::test]
async fn test_delete_inactive() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id,
            user_id,
            name: "client1".to_string(),
            original_name: "original1".to_string(),
            active: false,
            is_user_client: false,
            valid_until: Some(create_now()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::delete()
        .uri(&format!("/api/v1/client/{}", client_id))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::BAD_REQUEST, res.status());
    let body: ErrorDto = read_body_json(res).await;
    #[cfg(debug_assertions)]
    assert_eq!(body.message.unwrap(), "Client is already inactive");
    assert_eq!(body.code, 400);
}

#[actix_web::test]
async fn test_delete_in_database() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[create_user(&user_id, "test")]])
        .append_query_results([[client::Model {
            id: client_id,
            user_id,
            name: "client1".to_string(),
            original_name: "original1".to_string(),
            active: true,
            is_user_client: false,
            valid_until: Some(create_now()),
            created_at: Default::default(),
            updated_at: Default::default(),
        }]])
        .append_query_results([Vec::<signing_request::Model>::new()])
        .append_query_results([Vec::<token::Model>::new()])
        .append_exec_results([MockExecResult {
            rows_affected: 1,
            last_insert_id: 1,
        }]);

    init_test!(
        app,
        module!(client_controller::module),
        TestInitData {
            db,
            ..Default::default()
        }
    );

    let req = TestRequest::delete()
        .uri(&format!(
            "/api/v1/client/{}?deleteInDatabase=true",
            client_id
        ))
        .append_header((
            "Authorization",
            encode_keycloak_token(&user_id, "test", "test", &["user"]),
        ))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(StatusCode::NO_CONTENT, res.status());
}
