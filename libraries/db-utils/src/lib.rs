use sqlx::mysql::MySqlPoolOptions;

pub async fn create_database(connection_string: &str, name: &str) -> anyhow::Result<()> {
    let db_pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(connection_string)
        .await?;

    let query = format!("CREATE DATABASE IF NOT EXISTS {}", name);
    sqlx::query(&query).bind(name).execute(&db_pool).await?;

    Ok(())
}