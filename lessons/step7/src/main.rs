use std::sync::Arc;
use tokio::task::JoinSet;

use crate::connection::setup_connection;
use crate::repositories::Repositories;

mod models;
mod connection;
mod repositories;
mod workers;
mod logger;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize the logger
    println!("Initializing logger");
    logger::init();

    // Setup database connections for TweetService and TimelineService
    let connection = Arc::new(setup_connection().await);
    let repositories = Arc::new(Repositories::new(Arc::clone(&connection)).await);

    // Initialize a JoinSet to manage multiple asynchronous tasks
    let mut set = JoinSet::new();

    // Generate a list of user workers
    let workers = 20;

    // Spawn tasks for both Twitter ingestion and fetching timelines
    for i in 1..workers {
        // Clone the necessary Arcs for each task
        let repository = Arc::clone(&repositories);

        // Spawn an async task for Twitter ingestion
        set.spawn(async move {
            let _ = workers::ingestion::twitter_ingestion(i, repository).await;
        });
    }

    while let Some(res) = set.join_next().await {
        res?;
    }

    Ok(())
}
