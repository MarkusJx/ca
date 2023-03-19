use dotenv::dotenv;
use envconfig::Envconfig;
use std::error::Error;

#[derive(Debug, Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
    #[envconfig(from = "JWT_EXPIRES_IN")]
    pub jwt_expires_in: String,
    #[envconfig(from = "JWT_MAX_AGE")]
    pub jwt_max_age: i32,
    #[envconfig(from = "DB_VENDOR")]
    pub db_vendor: String,
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,
    #[envconfig(from = "DB_PORT")]
    pub db_port: String,
    #[envconfig(from = "DB_NAME")]
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
}

impl Config {
    pub fn init() -> Result<Config, Box<dyn Error>> {
        dotenv()?;
        Config::init_from_env().map_err(|e| e.into())
    }
}