use anyhow::Ok;

#[tokio::main]
async fn main()-> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db_connection_string = std::env::var("DATABASE_URL")?;
    let connection_pool = sqlx::SqlitePool::connect(&db_connection_string).await?;

    //Run migrations
    sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await?;

    Ok(())
}
