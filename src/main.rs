use std::net::SocketAddr;
use anyhow::Ok;
use axum::{extract::Multipart, response::Html, routing::{get, post}, Extension, Router};
use sqlx::Row;

#[tokio::main]
async fn main()-> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db_connection_string = std::env::var("DATABASE_URL")?;
    let connection_pool = sqlx::SqlitePool::connect(&db_connection_string).await?;

    //Run migrations
    sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await?;

    let app = Router::new()
        .route("/", get(index_page))
        .route("/upload", post(uploader))
        .layer(Extension(connection_pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// async fn test(Extension(pool): Extension<sqlx::SqlitePool>) -> String {
//     let result = sqlx::query("SELECT COUNT(id) FROM images")
//         .fetch_one(&pool)
//         .await
//         .unwrap();
//     let count = result.get::<i64, _>(0);
//     format!("{count} images in the database")
// }
async fn index_page() -> Html<String> {
    let path = std::path::Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
async fn insert_image_into_database(pool: &sqlx::SqlitePool, tags:&str)-> anyhow::Result<i64>{
    let row = 
    sqlx::query("ÏNSERT INTO images (tags) VALUES (?) RETURNING id")
    .bind(tags)
    .fetch_one(pool)
    .await?;
    Ok(row.get(0))
}

async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    let base_path = std::path::Path::new("images");
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }
    let image_path = base_path.join(format!("{id}.jpg"));
    if image_path.exists() {
        anyhow::bail!("File already exists");
    }
    tokio::fs::write(image_path, bytes).await?;
    Ok(())
}

async fn uploader(
    Extension(pool): Extension<sqlx::SqlitePool>,
    mut multipart: Multipart
) -> String {
    let mut tags = None; 
    let mut image = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()), 
            "image" => image = Some(data.to_vec()),
            _ => panic!("Unknown field: {name}"),
        }
    }

    if let (Some(tags), Some(image)) = (tags, image) { 
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
    } else {
        panic!("Missing field");
    }

    "Ok".to_string()
}