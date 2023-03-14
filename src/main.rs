mod config;
mod controller;
mod entity;
mod error;
mod middleware;
mod mk_certs;
mod model;
mod repository;
mod service;
mod util;

use crate::controller::{certificate, client_controller, common, swagger, user_controller};
use crate::error::http_response_error::MapHttpResponseError;
use crate::middleware::keycloak_middleware;
use crate::mk_certs::mk_request;
use crate::repository::database;
use crate::service::client_service::ClientService;
use crate::service::keycloak_service::KeycloakService;
use crate::service::signing_request_service::SigningRequestService;
use crate::service::user_service::UserService;
use crate::util::api_doc::ApiDoc;
use crate::util::traits::map_error_to_io_error::MapErrorToIoError;
use crate::util::traits::register_module::RegisterModule;
use crate::util::traits::u8_vec_to_string::U8VecToString;
use actix_web::get;
use actix_web::web::{scope, Json};
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use config::app_state::AppState;
use keycloak::types::UserRepresentation;
use log::{debug, info};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use serde::{Deserialize, Serialize};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

extern crate core;
extern crate lazy_static;
extern crate sea_orm;

#[derive(Serialize, Deserialize, ToSchema)]
struct ClientCert {
    cert: String,
    generated_at: u128,
}

#[utoipa::path(
    get,
    tag = "Certificates",
    responses(
        (status = 200, description = "Ok", body = ClientCert),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[get("/generate-client")]
async fn generate_client() -> Result<Json<ClientCert>, actix_web::Error> {
    info!("Generating client cert");
    let rsa = Rsa::generate(2048).map_internal_error(None)?;
    let key_pair = PKey::from_rsa(rsa).map_internal_error(None)?;

    let req = mk_request(&key_pair).map_internal_error(None)?;

    Ok(Json(ClientCert {
        cert: req.to_pem().map_internal_error(None)?.to_string(),
        generated_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    }))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("requests", log::LevelFilter::Debug))
        .build(
            Root::builder()
                .appender("stdout")
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    info!("Loading config");
    let config = config::config::Config::init().map_to_io_error()?;

    info!("Connecting to database");
    let db = database::connect(&config).await.map_to_io_error()?;
    info!("Creating tables");
    database::fill(&db).await.map_to_io_error()?;

    info!("Connecting to keycloak");
    let keycloak_service = KeycloakService::new(&config).await.map_to_io_error()?;

    if config.keycloak_init_realm {
        info!("Initializing keycloak realm");
        keycloak_service.init_realm().await.map_to_io_error()?;
    }

    info!("Setting keycloak certificate");
    keycloak_middleware::set_keycloak_public_key(
        keycloak_service
            .get_realm_public_key()
            .await
            .map_to_io_error()?,
    )
    .map_to_io_error()?;

    debug!(
        "{:?}",
        keycloak_service.get_client_by_name("test").await.unwrap()
    );

    info!("Starting http server");
    HttpServer::new(move || {
        let scope = scope("/api/v1")
            .service(certificate::register())
            .module(user_controller::module)
            .module(client_controller::module)
            .module(common::module);

        App::new()
            .app_data(web::Data::new(AppState {
                config: config.clone(),
                keycloak_service: keycloak_service.clone(),
                client_service: ClientService::new(db.clone()),
                user_service: UserService::new(db.clone()),
                signing_request_service: SigningRequestService::new(db.clone()),
            }))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/schema.json", ApiDoc::openapi()),
            )
            .wrap(actix_middleware::Logger::default())
            .service(generate_client)
            .service(swagger::get_swagger_ui)
            .service(scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
