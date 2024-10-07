use tokio_postgres::{Error, NoTls};

pub async fn connect_db() -> Result<
    (
        tokio_postgres::Client,
        tokio::task::JoinHandle<Result<(), Error>>,
    ),
    Error,
> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=seifmamdouh password=10122001sDM#$ dbname=n_plus_one",
        NoTls,
    )
    .await?;

    // Spawn the connection to run concurrently
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
