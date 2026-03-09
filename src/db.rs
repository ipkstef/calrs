use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;

pub async fn connect(data_dir: &Path) -> Result<SqlitePool> {
    std::fs::create_dir_all(data_dir)?;
    let db_path = data_dir.join("calrs.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let options = SqliteConnectOptions::from_str(&db_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> Result<()> {
    let sql = include_str!("../migrations/001_initial.sql");
    sqlx::raw_sql(sql).execute(pool).await?;
    Ok(())
}
