use crate::config::Config;
use crate::timed_call::TimedCall;
use log::{debug, info};
use openssl::x509::{X509Req, X509};
use reqwest::Client;
use shared::model::health_info_dto::HealthInfoDto;
use shared::model::new_signing_request_dto::NewSigningRequestDto;
use shared::model::signing_request_dto::SigningRequestDto;
use shared::util::traits::u8_vec_to_string::U8VecToString;
use shared::util::types::BasicResult;

pub struct Api {
    token: String,
    api_url: String,
    client: Client,
}

impl Api {
    pub fn new(config: &Config) -> Self {
        Self {
            token: config.token.clone(),
            api_url: config.api_url.clone(),
            client: Client::new(),
        }
    }

    pub async fn check_api(&self) -> BasicResult<()> {
        let res = TimedCall::time(move || async move {
            self.client
                .get(format!("{}/api/v1/health", self.api_url).as_str())
                .send()
                .await?
                .error_for_status()?
                .json::<HealthInfoDto>()
                .await
                .map_err(|e| Box::new(e))
        })
        .await;

        let data = res.result?;
        info!("Connected to API: {}", data.version);
        info!(
            "Keycloak version: {}",
            data.keycloak_version.unwrap_or("unknown".into())
        );
        info!("API response time: {:?}", res.duration);

        Ok(())
    }

    pub async fn sign_certificate(
        &self,
        csr: X509Req,
        alt_names: Option<Vec<String>>,
    ) -> BasicResult<X509> {
        let req = NewSigningRequestDto {
            request: csr.to_pem()?.to_string(),
            alternative_names: alt_names,
        };

        debug!("Sending signing request: {:?}", req);
        let res = self
            .client
            .post(format!("{}/api/v1/certificate/sign", self.api_url).as_str())
            .bearer_auth(self.token.clone())
            .json(&req)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await?
            .error_for_status()?
            .json::<SigningRequestDto>()
            .await
            .map_err(|e| Box::new(e))?;

        debug!("Certificate received: {:?}", res);
        if let Some(cert) = res.certificate {
            X509::from_pem(cert.as_bytes()).map_err(|e| e.into())
        } else {
            Err("No certificate received".into())
        }
    }
}
