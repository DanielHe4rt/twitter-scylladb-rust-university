use std::sync::Arc;

use crate::connection::setup_connection;
use crate::repositories::timeline_service::TimelineService;
use crate::repositories::tweet_service::TweetService;

mod models;
mod connection;
mod utils;
mod repositories;
mod workers;

#[tokio::main]
async fn main() -> Result<(), anyerror::AnyError> {
    let connection = Arc::new(setup_connection().await);
    let tweet_service = Arc::new(TweetService { connection: Arc::clone(&connection) });
    let timeline_service = Arc::new(TimelineService { connection: Arc::clone(&connection) });
    let timeline_service2 = Arc::clone(&timeline_service);

    let ingestion_task = tokio::spawn(async move {
        workers::ingestion::twitter_ingestion(
            Arc::clone(&timeline_service),
            Arc::clone(&tweet_service),
        ).await;
    });

    let metrics_task = tokio::spawn(async move {
        workers::metrics::fetch_timelines(
            timeline_service2,
        ).await;
    });

    tokio::try_join!(ingestion_task, metrics_task).unwrap();

    Ok(())
}
