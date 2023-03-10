use crate::entities::{client, signing_request, user};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Schema};
use std::error::Error;
use std::time::Duration;

pub async fn connect() -> Result<DatabaseConnection, Box<dyn Error>> {
    let mut opts = ConnectOptions::new("postgres://postgres:postgres@localhost/postgres".into());
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
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let user = builder.build(
        schema
            .create_table_from_entity(user::Entity)
            .if_not_exists(),
    );
    let client = builder.build(
        schema
            .create_table_from_entity(client::Entity)
            .if_not_exists(),
    );
    let signing_request = builder.build(
        schema
            .create_table_from_entity(signing_request::Entity)
            .if_not_exists(),
    );

    db.execute(user).await?;
    db.execute(client).await?;
    db.execute(signing_request).await?;

    Ok(())
}
