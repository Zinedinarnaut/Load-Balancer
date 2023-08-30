// cache.rs

use sqlx::PgPool;
use anyhow::Result;

// Define a struct to represent cached data
#[derive(Debug, sqlx::FromRow)]
pub struct CachedData {
    pub id: i32,
    pub user_id: i32,
    pub data: String,
}

// Function to insert cached data into the database
pub async fn insert_cached_data(pool: &PgPool, user_id: i32, data: &str) -> Result<()> {
    sqlx::query!(
        "INSERT INTO cache_entry (user_id, data) VALUES ($1, $2)",
        user_id,
        data
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Function to retrieve cached data from the database
pub async fn get_cached_data(pool: &PgPool, user_id: i32) -> Result<Option<String>> {
    let cached_data = sqlx::query_as!(
        CachedData,
        "SELECT id, user_id, data FROM cache_entry WHERE user_id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(cached_data.map(|entry| entry.data))
}

// Function to update cached data in the database
pub async fn update_cached_data(pool: &PgPool, user_id: i32, data: &str) -> Result<()> {
    sqlx::query!(
        "UPDATE cache_entry SET data = $1 WHERE user_id = $2",
        data,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
