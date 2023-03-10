use crate::config::config::Config;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
}
