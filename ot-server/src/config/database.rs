use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr,
    Statement,
};
use sea_orm_migration::MigratorTrait;
use std::time::Duration;
use tracing::info;

use super::env::EnvironmentVariables;
use crate::migrator::Migrator;

pub async fn initialize_database(env: &EnvironmentVariables) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(env.database_url.clone());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    if db.get_database_backend() == DatabaseBackend::Postgres {
        // Check if database exists
        let result = db
            .execute(Statement::from_string(
                db.get_database_backend(),
                format!(
                    "SELECT 1 FROM pg_database WHERE datname = '{}'",
                    env.database
                ),
            ))
            .await?;

        // Create database if it doesn't exist
        if result.rows_affected() == 0 {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\"", env.database),
            ))
            .await?;
        }

        // Connect to the specific database
        let url = format!("{}/{}", env.database_url, env.database);
        info!("[Database] Connecting to database '{}'", env.database);
        let db = Database::connect(&url).await?;

        // Run migrations
        info!("[Migration] Starting fresh migration");
        Migrator::up(&db, None).await?;

        Ok(db)
    } else {
        // For other database backends, just return the initial connection
        Ok(db)
    }
}
