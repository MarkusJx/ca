use crate::controller::helpers::encode_keycloak_token;
use crate::init_test;
use actix_web::test::{call_service, read_body_json, TestRequest};
use ca_backend::controller::admin_controller;
use ca_backend::entity::user;
use ca_backend::service::keycloak_service::MockKeycloakService;
use sea_orm::{DatabaseBackend, MockDatabase};
use uuid::Uuid;
use ca_backend::model::error_dto::ErrorDto;

#[actix_web::test]
async fn test_list_roles() {
    let mut kc = MockKeycloakService::new();
    kc.expect_get_roles()
        .returning(|| Box::pin(async { Ok(vec!["role1".to_string(), "role2".to_string()]) }));

    let user_id = Uuid::new_v4();
    let db =
        MockDatabase::new(DatabaseBackend::Postgres).append_query_results([vec![user::Model {
            name: "test".into(),
            active: true,
            id: user_id.clone(),
            original_name: "test".into(),
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
            external_id: Some(user_id.to_string()),
        }]]);

    init_test!(
        app,
        admin_controller::module,
        TestInitData {
            kc: Box::new(kc),
            db
        }
    );

    let token = encode_keycloak_token(&user_id, "test", "test", &["admin"]);
    let req = TestRequest::get()
        .uri("/api/v1/admin/roles")
        .append_header(("Authorization", token))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(200, res.status().as_u16());
    let body: Vec<String> = read_body_json(res).await;
    assert_eq!(body, vec!["role1".to_string(), "role2".to_string()]);
}

#[actix_web::test]
async fn test_list_roles_insufficient_privileges() {
    init_test!(
        app,
        admin_controller::module
    );

    let token = encode_keycloak_token(&Uuid::new_v4(), "test", "test", &["user"]);
    let req = TestRequest::get()
        .uri("/api/v1/admin/roles")
        .append_header(("Authorization", token))
        .to_request();
    let res = call_service(&app, req).await;

    assert_eq!(401, res.status().as_u16());
    let body: ErrorDto = read_body_json(res).await;
    assert_eq!(body.code, 401);
    assert_eq!(body.error, "Unauthorized");
}

#[actix_web::test]
async fn test_list_roles_no_token() {
    init_test!(
        app,
        admin_controller::module
    );

    let req = TestRequest::get()
        .uri("/api/v1/admin/roles")
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(401, res.status().as_u16());
}

#[actix_web::test]
async fn test_list_roles_invalid_token() {
    init_test!(
        app,
        admin_controller::module
    );

    let req = TestRequest::get()
        .uri("/api/v1/admin/roles")
        .append_header(("Authorization", "Bearer invalid"))
        .to_request();

    let res = call_service(&app, req).await;
    assert_eq!(401, res.status().as_u16());
}
