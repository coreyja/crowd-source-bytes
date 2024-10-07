pub mod db;
use color_eyre::eyre::Result;
use db::{connect_db, fetch_comments_by_post, fetch_post_ids};

#[tokio::main]
async fn main() -> Result<()> {
    let (client, connection_handle) = connect_db().await?;

    tokio::spawn(async move {
        if let Err(e) = connection_handle.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let post_ids = fetch_post_ids(&client).await?;
    println!("Fetched {} posts.", post_ids.len());

    // N + 1 Bug: For each post, fetch comments individually
    for post_id in post_ids {
        let comments = fetch_comments_by_post(&client, post_id).await?;
        println!("Post ID {} has {} comments.", post_id, comments.len());
    }

    Ok(())
}
