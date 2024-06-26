use std::sync::Arc;
use tokio::task::JoinSet;

use crate::connection::setup_connection;
use crate::repositories::Repositories;
use crate::utils::generate_users;

mod models;
mod connection;
mod utils;
mod repositories;
mod workers;
mod logger;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize the logger
    logger::init();

    // Setup database connections for TweetService and TimelineService
    let connection = Arc::new(setup_connection().await);
    let repositories = Arc::new(Repositories::new(Arc::clone(&connection)).await);


    // Initialize a JoinSet to manage multiple asynchronous tasks
    let mut set = JoinSet::new();

    // Generate a list of user workers
    let workers = generate_users(100);

    // Spawn tasks for both Twitter ingestion and fetching timelines
    for user in workers {
        // Clone the necessary Arcs for each task
        let user = Arc::new(user.clone());
        let repository = Arc::clone(&repositories);

        // Spawn an async task for Twitter ingestion
        set.spawn(async move {
            workers::ingestion::twitter_ingestion(
                user,
                repository
            ).await;
        });
    }

    while let Some(res) = set.join_next().await {
        res?;
    }

    Ok(())
}
