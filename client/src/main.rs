#![allow(unreachable_code)]
use crate::api::Api;
use crate::certificate::Certificate;
use crate::certificate_renewer::CertificateRenewer;
use crate::config::Config;
use ::log::{debug, info};
use shared::model::health_info_dto::HealthInfoDto;
use shared::util::types::BasicResult;

#[macro_use]
extern crate rouille;

mod api;
mod certificate;
mod certificate_renewer;
mod config;
mod log;
mod timed_call;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> BasicResult<()> {
    log::init()?;

    info!("Loading config");
    let config = Config::init()?;

    debug!("{:?}", config);
    let api = Api::new(&config);

    api.check_api().await?;

    info!("Trying to load certificates");
    let cert = match Certificate::load("certs".into(), config.passphrase.as_ref()).await {
        Ok(cert) => {
            info!("Loaded certificates");
            cert
        }
        Err(e) => {
            info!("Failed to load certificates: {}", e);
            info!("Generating new certificates");
            let cert = Certificate::generate()?;
            info!("Storing certificates");
            cert.store("certs".into(), config.passphrase.as_ref())
                .await?;
            info!("Certificates stored");
            cert
        }
    };

    let mut renewer = CertificateRenewer::new(api, config.clone(), cert);
    renewer.renew_periodically();

    info!("Starting http server");
    rouille::start_server(format!("0.0.0.0:{}", config.server_port), move |request| {
        router!(request,
            (GET) (/api/v1/health) => {
                rouille::Response::json(&HealthInfoDto {
                    version: VERSION.to_string(),
                    keycloak_version: None,
                    status: renewer.get_last_error().map(|e| e.to_string()).unwrap_or("OK".into()),
                    ok: renewer.get_last_error().is_none(),
                    is_initialized: None,
                })
            },
            _ => rouille::Response::empty_404()
        )
    });
}
