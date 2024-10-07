use tokio_postgres::{NoTls, Error};

/// Establishes a connection to the PostgreSQL database.
/// 
/// # Returns
/// 
/// A tuple containing the database client and the connection handle.
pub async fn connect_db() -> Result<(tokio_postgres::Client, tokio::task::JoinHandle<Result<(), Error>>), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=seifmamdouh password=10122001sDM#$ dbname=n_plus_one", NoTls).await?;

    // Spawn the connection to run concurrently
    let handle = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
        Ok(())
    });

    Ok((client, handle))
}

/// Fetches all post IDs from the database.
/// 
/// # Returns
/// 
/// A vector of post IDs.
pub async fn fetch_post_ids(client: &tokio_postgres::Client) -> Result<Vec<i32>, Error> {
    let rows = client.query("SELECT id FROM posts", &[]).await?;
    Ok(rows.into_iter().map(|row| row.get(0)).collect())
}

/// Fetches comments for a specific post by post ID.
/// 
/// # Arguments
/// 
/// * `client` - The PostgreSQL client.
/// * `post_id` - The ID of the post to fetch comments for.
/// 
/// # Returns
/// 
/// A vector of comment IDs associated with the post.
pub async fn fetch_comments_by_post(client: &tokio_postgres::Client, post_id: i32) -> Result<Vec<i32>, Error> {
    let stmt = client.prepare("SELECT id FROM comments WHERE post_id = $1").await?;
    let rows = client.query(&stmt, &[&post_id]).await?;
    Ok(rows.into_iter().map(|row| row.get(0)).collect())
}
