use crate::api::Api;
use crate::certificate::Certificate;
use crate::config::Config;
use ::log::{debug, info};
use shared::util::types::BasicResult;

mod api;
mod certificate;
mod config;
mod log;
mod timed_call;

#[tokio::main]
async fn main() -> BasicResult<()> {
    log::init()?;

    info!("Loading config");
    let config = Config::init()?;

    debug!("{:?}", config);
    let api = Api::new(&config);

    api.check_api().await?;

    info!("Trying to load certificates");
    let mut cert = match Certificate::load("certs".into(), config.passphrase.as_ref()).await {
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

    info!("Checking token");
    jsonwebtoken::decode_header(&config.token)?;

    if !cert.has_certificate() {
        info!("Requesting certificate");
        let csr = cert.get_signing_request(&config)?;
        let signed = api.sign_certificate(csr, config.alt_names()).await?;
        info!("Storing certificate");
        cert.set_certificate(signed);
        cert.store("certs".into(), config.passphrase.as_ref())
            .await?;
        info!("Certificate stored");
    }

    info!("Certificate expiration: {:?}", cert.get_cert_expiration());

    Ok(())
}
