pub mod db;
use color_eyre::eyre::Result;
use db::{connect_db, fetch_posts};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let (client, _connection_handle) = connect_db().await?;

    let comment_counts = comment_count_per_post(&client).await?;
    println!("Fetched comment counts for {} posts.", comment_counts.len());

    for (post_id, count) in comment_counts {
        println!("Post ID {} has {} comments.", post_id, count);
    }

    Ok(())
}

async fn comment_count_per_post(client: &tokio_postgres::Client) -> Result<HashMap<i32, usize>> {
    let posts = fetch_posts(client).await?;
    let mut comment_counts = HashMap::new();
    
    for post in posts {
        let comments = post.fetch_comments(client).await?;
        comment_counts.insert(post.id, comments.len());
    }

    Ok(comment_counts)
}
