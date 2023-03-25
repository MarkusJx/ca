use crate::config::config::Config;
use crate::entity::{certificate, client, root_certificate, signing_request, token, user};
use log::debug;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Schema};
use std::error::Error;
use std::time::Duration;

macro_rules! generate_schema {
    ($db: ident, $($entity: ident), *) => {
        let builder = $db.get_database_backend();
        let schema = Schema::new(builder);
        $(
            let $entity = builder.build(
                schema
                    .create_table_from_entity($entity::Entity)
                    .if_not_exists(),
            );
            $db.execute($entity).await?;
        )+
    };
}

pub async fn connect(config: &Config) -> Result<DatabaseConnection, Box<dyn Error>> {
    let url = format!(
        "{}://{}:{}@{}:{}/{}",
        config.db_vendor,
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_name
    );
    debug!("Connecting to database: {}", url);

    let mut opts = ConnectOptions::new(url);
    opts.max_connections(10)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("public".into());

    Database::connect(opts).await.map_err(|e| e.into())
}

pub async fn fill(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    generate_schema!(
        db,
        user,
        client,
        signing_request,
        token,
        certificate,
        root_certificate
    );

    Ok(())
}
