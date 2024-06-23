use std::sync::Arc;
use tokio::task::JoinSet;

use crate::connection::setup_connection;
use crate::repositories::timeline_service::TimelineService;
use crate::repositories::tweet_service::TweetService;
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
    let tweet_service = Arc::new(TweetService::new(Arc::clone(&connection)).await);
    let timeline_service = Arc::new(TimelineService::new(Arc::clone(&connection)).await);

    // Initialize a JoinSet to manage multiple asynchronous tasks
    let mut set = JoinSet::new();

    // Generate a list of user workers
    let workers = generate_users(10);

    // Spawn tasks for both Twitter ingestion and fetching timelines
    for user in workers {
        // Clone the necessary Arcs for each task
        let user = Arc::new(user.clone());
        let tweet_service_ingestion = Arc::clone(&tweet_service);
        let timeline_service_ingestion = Arc::clone(&timeline_service);
        let user_metrics = Arc::clone(&user);
        let timeline_service_metrics = Arc::clone(&timeline_service);

        // Spawn an async task for Twitter ingestion
        set.spawn(async move {
            workers::ingestion::twitter_ingestion(
                user,
                timeline_service_ingestion,
                tweet_service_ingestion,
            ).await;
        });

        // Spawn an async task for fetching timelines
        set.spawn(async move {
            workers::metrics::fetch_timelines(
                user_metrics,
                timeline_service_metrics,
            ).await;
        });
    }

    while let Some(res) = set.join_next().await {
        res?;
    }

    Ok(())
}
