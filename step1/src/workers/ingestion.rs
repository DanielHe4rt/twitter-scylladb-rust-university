use std::sync::Arc;
use log::info;

use crate::repositories::timeline_service::{TimelineService, TimelineServiceTrait};
use crate::repositories::tweet_service::{TweetService, TweetServiceTrait};

pub async fn twitter_ingestion(
    author: Arc<String>,
    timeline_service: Arc<TimelineService>,
    tweet_service: Arc<TweetService>,
) {
    let author = author.as_str();
    let text = "very nice".to_string();
    loop {

        let tweet_creation = tweet_service.create_tweet(&author, text.clone()).await;

        match tweet_creation {
            Ok(tweet) => {
                info!("Tweet created!");
                let timeline = timeline_service.insert_to_timeline(&author, &tweet).await;

                match timeline {
                    Ok(timeline) => info!("Timeline created!"),
                    Err(e) => info!("Error creating timeline: {:?}", e)
                }
            }
            Err(e) => info!("Error creating tweet: {:?}", e)
        }
    }
}