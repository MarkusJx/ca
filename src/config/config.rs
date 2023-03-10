use dotenv::dotenv;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i32,
}

impl Config {
    pub fn init() -> Result<Config, Box<dyn Error>> {
        dotenv()?;

        let database_url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?;
        let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| "JWT_SECRET must be set")?;
        let jwt_expires_in =
            std::env::var("JWT_EXPIRED_IN").map_err(|_| "JWT_EXPIRES_IN must be set")?;
        let jwt_max_age = std::env::var("JWT_MAX_AGE").map_err(|_| "JWT_MAX_AGE must be set")?;

        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age: jwt_max_age.parse::<i32>()?,
        })
    }
}
