mod config;
mod controller;
mod entity;
mod error;
mod middleware;
mod model;
mod repository;
mod service;
mod util;

use crate::controller::{
    admin_controller, certificate_controller, client_controller, common,
    signing_request_controller, swagger, user_controller,
};
use crate::middleware::keycloak_middleware;
use crate::repository::database;
use crate::service::certificate_service::CertificateService;
use crate::service::client_service::ClientService;
use crate::service::keycloak_service::KeycloakService;
use crate::service::root_certificate_service::RootCertificateService;
use crate::service::signing_request_service::SigningRequestService;
use crate::service::token_service::TokenService;
use crate::service::user_service::UserService;
use crate::util::api_doc::ApiDoc;
use crate::util::traits::map_error_to_io_error::MapErrorToIoError;
use crate::util::traits::register_module::RegisterModule;
use actix_cors::Cors;
use actix_web::web::scope;
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use config::app_state::AppState;
use log::info;
use shared::util::logger::init_logger;
use std::io;
use std::str::FromStr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

extern crate core;
extern crate lazy_static;
extern crate sea_orm;

#[actix_web::main]
async fn main() -> io::Result<()> {
    init_logger(log::LevelFilter::Debug).map_to_io_error()?;

    info!("Loading config");
    let config = config::config::Config::init().map_to_io_error()?;
    init_logger(
        log::LevelFilter::from_str(&config.log_level.to_lowercase())
            .map_err(|e| e.into())
            .map_to_io_error()?,
    )
    .map_to_io_error()?;

    info!("Connecting to database");
    let db = database::connect(&config).await.map_to_io_error()?;
    info!("Creating tables");
    database::fill(&db).await.map_to_io_error()?;

    info!("Connecting to keycloak");
    let keycloak_service = KeycloakService::new(&config).await.map_to_io_error()?;
    let user_service = UserService::new(db.clone());

    if config.keycloak_init_realm {
        info!("Initializing keycloak realm");
        keycloak_service
            .init_realm(&user_service)
            .await
            .map_to_io_error()?;
    }

    info!("Setting keycloak certificate");
    keycloak_middleware::set_keycloak_public_key(
        keycloak_service
            .get_realm_public_key()
            .await
            .map_to_io_error()?,
    )
    .map_to_io_error()?;

    info!("Starting http server");
    let port = config.port;
    HttpServer::new(move || {
        let scope = scope("/api/v1")
            .service(certificate_controller::register())
            .module(user_controller::module)
            .module(client_controller::module)
            .module(signing_request_controller::module)
            .module(admin_controller::module)
            .module(common::module);

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        let mut app = App::new()
            .app_data(web::Data::new(AppState {
                config: config.clone(),
                keycloak_service: keycloak_service.clone(),
                client_service: ClientService::new(db.clone()),
                user_service: user_service.clone(),
                signing_request_service: SigningRequestService::new(db.clone()),
                token_service: TokenService::new(db.clone()),
                certificate_service: CertificateService::new(db.clone()),
                root_certificate_service: RootCertificateService::new(db.clone()),
            }))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/schema.json", ApiDoc::openapi()),
            )
            .wrap(actix_middleware::Logger::default())
            .wrap(cors)
            .service(scope);

        if config.enable_swagger || cfg!(debug_assertions) {
            app = app.service(swagger::get_swagger_ui);
        }

        app
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
