use std::sync::Arc;

use crate::repositories::timeline_service::{TimelineService, TimelineServiceTrait};
use crate::repositories::tweet_service::{TweetService, TweetServiceTrait};

pub async fn twitter_ingestion(
    timeline_service: Arc<TimelineService>,
    tweet_service: Arc<TweetService>,
) {
    let author = "danielhe4rt";
    let text = "very nice".to_string();
    loop {

        let tweet_creation = tweet_service.create_tweet(&author, text.clone()).await;

        match tweet_creation {
            Ok(tweet) => {
                println!("Tweet created!");
                let timeline = timeline_service.insert_to_timeline(&author, &tweet).await;

                match timeline {
                    Ok(timeline) => println!("Timeline created!"),
                    Err(e) => println!("Error creating timeline: {:?}", e)
                }
            }
            Err(e) => println!("Error creating tweet: {:?}", e)
        }
    }
}