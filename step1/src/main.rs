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
    logger::init();
    let tweet_service = Arc::new(TweetService { connection: Arc::new(setup_connection().await) });
    let timeline_service = Arc::new(TimelineService { connection: Arc::new(setup_connection().await) });

    let workers = generate_users(10);

    let mut set = JoinSet::new();
    for user in &workers {
        let user = Arc::new(user.clone());
        let timeline_service = Arc::clone(&timeline_service);
        let tweet_service = Arc::clone(&tweet_service);
        set.spawn(async move {
            workers::ingestion::twitter_ingestion(
                user,
                timeline_service,
                tweet_service,
            ).await;
        });
    }
    for user in workers {
        let user = Arc::new(user.clone());
        let timeline_service = Arc::clone(&timeline_service);
        set.spawn(async move {
            workers::metrics::fetch_timelines(
                user,
                timeline_service,
            ).await;
        });
    }

    while let Some(res) = set.join_next().await {
        res?
    }

    Ok(())
}
