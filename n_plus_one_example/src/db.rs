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

pub struct Post {
    pub id: i32,
    pub title: String,
}

impl Post {
    pub async fn fetch_comments(&self, client: &tokio_postgres::Client) -> Result<Vec<i32>, Error> {
        let stmt = client
            .prepare("SELECT id FROM comments WHERE post_id = $1")
            .await?;
        let rows = client.query(&stmt, &[&self.id]).await?;
        Ok(rows.into_iter().map(|row| row.get(0)).collect())
    }
}

pub async fn fetch_posts(client: &tokio_postgres::Client) -> Result<Vec<Post>, Error> {
    let rows = client.query("SELECT id, title FROM posts", &[]).await?;
    Ok(rows
        .into_iter()
        .map(|row| Post {
            id: row.get(0),
            title: row.get(1),
        })
        .collect())
}
