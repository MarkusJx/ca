use dotenv::dotenv;
use envconfig::Envconfig;
use std::error::Error;

#[derive(Debug, Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "TOKEN")]
    pub token: String,
    #[envconfig(from = "API_URL")]
    pub api_url: String,
    #[envconfig(from = "PASSPHRASE")]
    pub passphrase: Option<String>,
    #[envconfig(from = "CERT_COUNTRY")]
    pub cert_country: Option<String>,
    #[envconfig(from = "CERT_STATE")]
    pub cert_state: Option<String>,
    #[envconfig(from = "CERT_LOCALITY")]
    pub cert_locality: Option<String>,
    #[envconfig(from = "CERT_ORGANIZATION")]
    pub cert_organization: Option<String>,
    #[envconfig(from = "CERT_ORGANIZATION_UNIT")]
    pub cert_organization_unit: Option<String>,
    #[envconfig(from = "CERT_COMMON_NAME")]
    pub cert_common_name: String,
    #[envconfig(from = "CERT_EMAIL")]
    pub cert_email: Option<String>,
    #[envconfig(from = "ALT_NAMES")]
    pub alt_names: Option<String>,
    #[envconfig(from = "RENEW_THRESHOLD_DAYS", default = "1")]
    pub renew_threshold_days: u32,
    #[envconfig(from = "SERVER_PORT", default = "8080")]
    pub server_port: u16,
    #[envconfig(from = "ENABLE_SERVER", default = "true")]
    pub enable_server: bool,
    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub log_level: String,
}

impl Config {
    pub fn init() -> Result<Config, Box<dyn Error>> {
        dotenv()?;
        Config::init_from_env().map_err(|e| e.into())
    }

    pub fn alt_names(&self) -> Option<Vec<String>> {
        self.alt_names
            .as_ref()
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
    }
}
