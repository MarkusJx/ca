mod config;
mod controller;
mod entities;
mod errors;
mod middlewares;
mod mk_certs;
mod models;
mod repositories;
mod util;

use crate::controller::{certificate, common, swagger, user};
use crate::errors::http_response_error::MapHttpResponseError;
use crate::mk_certs::mk_request;
use crate::repositories::database;
use crate::util::api_doc::ApiDoc;
use crate::util::traits::map_error_to_io_error::MapErrorToIoError;
use crate::util::traits::register_module::RegisterModule;
use crate::util::traits::u8_vec_to_string::U8VecToString;
use actix_web::get;
use actix_web::web::Json;
use actix_web::{middleware, web, App, HttpServer};
use config::app_state::AppState;
use log::info;
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

    info!("Connecting to database");
    let db = database::connect().await.map_to_io_error()?;
    info!("Creating tables");
    database::fill(&db).await.map_to_io_error()?;

    info!("Loading config");
    let config = config::config::Config::init().map_to_io_error()?;

    info!("Starting http server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: db.clone(),
                config: config.clone(),
            }))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/schema.json", ApiDoc::openapi()),
            )
            .wrap(middleware::Logger::default())
            .service(generate_client)
            .module(certificate::module)
            .module(swagger::module)
            .module(user::module)
            .module(common::module)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
