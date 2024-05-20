use std::sync::Arc;

use crate::connection::setup_connection;
use crate::repositories::timeline_service::TimelineService;
use crate::repositories::tweet_service::TweetService;
use crate::utils::generate_users;

mod models;
mod connection;
mod utils;
mod repositories;
mod workers;

#[tokio::main]
async fn main() -> Result<(), anyerror::AnyError> {
    let tweet_service = Arc::new(TweetService { connection: Arc::new(setup_connection().await) });
    let timeline_service = Arc::new(TimelineService { connection: Arc::new(setup_connection().await) });
    let timeline_service2 = Arc::clone(&timeline_service);
    let users = Arc::new(generate_users(1000));
    let users2 = Arc::clone(&users);
    let ingestion_task = tokio::spawn(async move {
        workers::ingestion::twitter_ingestion(
            Arc::clone(&timeline_service),
            Arc::clone(&tweet_service),
            Arc::clone(&users)
        ).await;
    });

    let metrics_task = tokio::spawn(async move {
        workers::metrics::fetch_timelines(
            timeline_service2,
            Arc::clone(&users2)
        ).await;
    });

    tokio::try_join!(ingestion_task, metrics_task).unwrap();

    Ok(())
}
