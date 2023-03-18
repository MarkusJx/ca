use crate::api::Api;
use crate::certificate::Certificate;
use crate::config::Config;
use derive_more::Display;
use futures::executor::block_on;
use log::{error, info};
use shared::util::types::BasicResult;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone, Display)]
pub enum RenewalErrorCode {
    #[display(fmt = "API access failed")]
    ApiAccessFailed,
    #[display(fmt = "Failed to get certificate expiry date")]
    FailedToGetCertificateExpiry,
    #[display(fmt = "API access token invalid")]
    TokenInvalid,
}

struct Data {
    api: Api,
    config: Config,
    certificate: Certificate,
    last_error: Option<RenewalErrorCode>,
}

impl Data {
    fn set_last_error(&mut self, error: RenewalErrorCode) {
        self.last_error = Some(error);
    }

    fn reset_last_error(&mut self) {
        self.last_error = None;
    }

    fn reload_config(&mut self) -> BasicResult<()> {
        info!("Reloading config");
        self.config = Config::init()?;
        self.api = Api::new(&self.config);
        Ok(())
    }
}

pub struct CertificateRenewer {
    data: Arc<Mutex<Data>>,
}

impl CertificateRenewer {
    pub fn new(api: Api, config: Config, certificate: Certificate) -> Self {
        Self {
            data: Arc::new(Mutex::new(Data {
                api,
                config,
                certificate,
                last_error: None,
            })),
        }
    }

    pub fn get_last_error(&self) -> Option<RenewalErrorCode> {
        self.data.lock().unwrap().last_error
    }

    async fn renew(data: &mut Data) -> BasicResult<()> {
        info!("Requesting certificate");
        let csr = data.certificate.get_signing_request(&data.config)?;
        let signed = data
            .api
            .sign_certificate(csr, data.config.alt_names())
            .await?;
        info!("Storing certificate");
        data.certificate.set_certificate(signed);
        data.certificate
            .store("certs".into(), data.config.passphrase.as_ref())
            .await?;
        info!("Certificate stored");
        Ok(())
    }

    pub fn renew_periodically(&mut self) {
        let arc = self.data.clone();
        thread::spawn(move || loop {
            let mut data = arc.lock().unwrap();

            info!("Checking token");
            if let Err(e) = jsonwebtoken::decode_header(&data.config.token) {
                error!("Token invalid: {}", e);
                data.set_last_error(RenewalErrorCode::TokenInvalid);

                if let Err(e) = data.reload_config() {
                    error!("Failed to reload config: {}", e);
                }

                drop(data);
                info!("Sleeping for 10 minutes");
                thread::sleep(Duration::from_secs(10 * 60));
                continue;
            }

            let expires_in = match data.certificate.expires_in_secs() {
                Ok(Some(expires_in)) => {
                    if expires_in < (data.config.renew_threshold_days * 24 * 60 * 60) as u64 {
                        if let Err(e) = block_on(Self::renew(&mut data)) {
                            data.set_last_error(RenewalErrorCode::ApiAccessFailed);
                            error!("Failed to renew certificate: {}", e);
                        } else {
                            data.reset_last_error();
                        }
                    } else {
                        info!("Certificate is valid for {} days", expires_in / 24 / 60 / 60);
                        data.reset_last_error();
                    }

                    Some(expires_in)
                }
                Ok(None) => {
                    if let Err(e) = block_on(Self::renew(&mut data)) {
                        data.set_last_error(RenewalErrorCode::ApiAccessFailed);
                        error!("Failed to renew certificate: {}", e);
                    }

                    drop(data);
                    continue;
                }
                Err(e) => {
                    data.set_last_error(RenewalErrorCode::FailedToGetCertificateExpiry);
                    error!("Failed to get certificate expiration: {}", e);
                    None
                }
            };

            drop(data);
            let expires_in = expires_in.unwrap_or(10 * 60);
            info!("Sleeping for {} hours", expires_in / 3600);
            thread::sleep(Duration::from_secs(expires_in));
        });
    }
}
