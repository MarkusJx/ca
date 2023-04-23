use crate::{init_test, module};
use actix_web::test::{call_service, read_body_json, TestRequest};
use ca_backend::controller::common;
use ca_backend::entity::root_certificate;
use ca_backend::error::http_response_error::HttpResponseError;
use ca_backend::service::keycloak_service::MockKeycloakService;
use sea_orm::{DatabaseBackend, MockDatabase};
use shared::model::health_info_dto::HealthInfoDto;

#[actix_web::test]
async fn test_health_check() {
    let mut kc = MockKeycloakService::new();
    kc.expect_get_server_info()
        .returning(|| Box::pin(async { Err(HttpResponseError::not_found("")) }));

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([Vec::<root_certificate::Model>::new()]);

    init_test!(
        app,
        module!(common::module),
        TestInitData {
            kc: Box::new(kc),
            db
        }
    );

    let req = TestRequest::get().uri("/api/v1/health").to_request();
    let res = call_service(&app, req).await;

    assert_eq!(200, res.status().as_u16());
    let body: HealthInfoDto = read_body_json(res).await;
    assert_eq!(body.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(body.keycloak_version, None);
    assert_eq!(body.status, "OK");
    assert_eq!(body.ok, true);
    assert_eq!(body.is_initialized, Some(false));
}
