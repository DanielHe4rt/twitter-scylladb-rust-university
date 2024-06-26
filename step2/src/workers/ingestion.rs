use std::sync::Arc;

use log::info;

use crate::repositories::Repositories;
use crate::repositories::timeline_service::{TimelineService, TimelineServiceTrait};
use crate::repositories::tweet_service::{TweetService, TweetServiceTrait};
use crate::utils::pick_random_name;

pub async fn twitter_ingestion(
    author: Arc<String>,
    repositories: Arc<Repositories>,
) {
    let author = pick_random_name();
    let text = "very nice".to_string();
    loop {
        let tweet_creation = repositories.tweet_service.create_tweet(&author, text.clone()).await;

        match tweet_creation {
            Ok(tweet) => {
                let _ = repositories.timeline_service.insert_to_timeline(&author, &tweet).await;
                let _ = repositories.timeline_service.get_timeline_by_username(&author).await;
            }
            Err(e) => info!("Error creating tweet: {:?}", e)
        }
    }
}