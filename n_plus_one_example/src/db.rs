use dotenv::dotenv;
use std::env;
use tokio_postgres::{Error, NoTls};

pub async fn connect_db() -> Result<
    (
        tokio_postgres::Client,
        tokio::task::JoinHandle<Result<(), Error>>,
    ),
    Error,
> {
    
    dotenv().ok();

    // Get environment variables
    let username = env::var("username").expect("username not set in .env");
    let password = env::var("password").expect("password not set in .env");
    let dbname = env::var("dbname").expect("dbname not set in .env");

    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host=localhost user={} password={} dbname={}",
            username, password, dbname
        ),
        NoTls,
    )
    .await?;

    let handle = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
        Ok(())
    });

    Ok((client, handle))
}

pub async fn fetch_post_ids(client: &tokio_postgres::Client) -> Result<Vec<i32>, Error> {
    let rows = client.query("SELECT id FROM posts", &[]).await?;
    Ok(rows.into_iter().map(|row| row.get(0)).collect())
}

pub async fn fetch_comments_by_post(
    client: &tokio_postgres::Client,
    post_id: i32,
) -> Result<Vec<i32>, Error> {
    let stmt = client
        .prepare("SELECT id FROM comments WHERE post_id = $1")
        .await?;
    let rows = client.query(&stmt, &[&post_id]).await?;
    Ok(rows.into_iter().map(|row| row.get(0)).collect())
}
