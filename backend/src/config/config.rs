use dotenv::dotenv;
use envconfig::Envconfig;
use log::warn;
use std::error::Error;

#[derive(Debug, Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "PORT", default = "8080")]
    pub port: u16,
    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub log_level: String,
    #[envconfig(from = "ENABLE_SWAGGER", default = "false")]
    pub enable_swagger: bool,
    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
    #[envconfig(from = "JWT_EXPIRES_IN")]
    pub jwt_expires_in: String,
    #[envconfig(from = "JWT_MAX_AGE")]
    pub jwt_max_age: i32,
    #[envconfig(from = "DB_VENDOR", default = "postgres")]
    pub db_vendor: String,
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,
    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: String,
    #[envconfig(from = "DB_NAME", default = "ca")]
    pub db_name: String,
    #[envconfig(from = "DB_USER")]
    pub db_user: String,
    #[envconfig(from = "DB_PASSWORD")]
    pub db_password: String,
    #[envconfig(from = "KEYCLOAK_URL")]
    pub keycloak_url: String,
    #[envconfig(from = "KEYCLOAK_USER")]
    pub keycloak_user: String,
    #[envconfig(from = "KEYCLOAK_PASSWORD")]
    pub keycloak_password: String,
    #[envconfig(from = "KEYCLOAK_REALM", default = "ca")]
    pub keycloak_realm: String,
    #[envconfig(from = "KEYCLOAK_INIT_REALM", default = "true")]
    pub keycloak_init_realm: bool,
    #[envconfig(from = "KEYCLOAK_DEFAULT_EMAIL_VERIFIED", default = "true")]
    pub keycloak_default_email_verified: bool,
    #[envconfig(from = "ADMIN_USER", default = "admin")]
    pub admin_user: String,
    #[envconfig(from = "ADMIN_PASSWORD", default = "admin")]
    pub admin_password: String,
    #[envconfig(from = "CA_CERT_COUNTRY", default = "DE")]
    pub ca_cert_country: String,
    #[envconfig(from = "CA_CERT_STATE", default = "Berlin")]
    pub ca_cert_state: String,
    #[envconfig(from = "CA_CERT_LOCALITY", default = "Berlin")]
    pub ca_cert_locality: String,
    #[envconfig(from = "CA_CERT_ORGANIZATION", default = "CA")]
    pub ca_cert_organization: String,
    #[envconfig(from = "CA_CERT_ORGANIZATIONAL_UNIT", default = "CA")]
    pub ca_cert_organizational_unit: String,
    #[envconfig(from = "CA_ROOT_CERT_COMMON_NAME", default = "CA Root")]
    pub ca_root_cert_common_name: String,
    #[envconfig(from = "CA_INTERMEDIATE_CERT_COMMON_NAME", default = "CA Intermediate")]
    pub ca_intermediate_cert_common_name: String,
    /// The number of days the root certificate is valid
    #[envconfig(from = "CA_ROOT_CERT_VALIDITY_DAYS", default = "2190")]
    pub ca_root_cert_validity_days: u32,
    /// The number of days the intermediate certificate is valid
    #[envconfig(from = "CA_INTERMEDIATE_CERT_VALIDITY_DAYS", default = "1095")]
    pub ca_intermediate_cert_validity_days: u32,
    /// The number of days a certificate signed by this CA is valid
    #[envconfig(from = "CERT_VALIDITY_DAYS", default = "31")]
    pub cert_validity_days: u32,
}

impl Config {
    pub fn init() -> Result<Config, Box<dyn Error>> {
        if let Err(e) = dotenv() {
            warn!("Failed to load .env file: {}", e);
        }

        Config::init_from_env().map_err(|e| e.into())
    }
}
