use std::sync::Arc;
use charybdis::options::Consistency;
use rand::prelude::SliceRandom;
use scylla::ExecutionProfile;
use utils::generate_users;
use crate::connection::setup_connection;
use crate::repositories::timeline_service::{TimelineService, TimelineServiceTrait};
use crate::repositories::tweet_service::{TweetService, TweetServiceTrait};

mod models;
mod connection;
mod utils;
mod repositories;

const USERS_COUNT: i32 = 1000;

#[tokio::main]
async fn main() {
    let connection = Arc::new(setup_connection().await);
    let tweet_service = TweetService { connection: Arc::clone(&connection) };
    let timeline_service = TimelineService { connection: Arc::clone(&connection) };

    let user_list = generate_users(USERS_COUNT);

    loop {
        let author = user_list.choose(&mut rand::thread_rng()).unwrap();
        let text = "This is a tweet".to_string();

        if let Ok(tweet) = tweet_service.create_tweet(author.to_string(), text.clone()).await {
            println!("Tweet created: {:?}", tweet);

            for _ in 0..10 {
                let username = user_list.choose(&mut rand::thread_rng()).unwrap();
                let timeline = timeline_service.insert_to_timeline(username, &tweet).await;

                match timeline {
                    Ok(timeline) => println!("Timeline created: {:?}", timeline),
                    Err(e) => println!("Error creating timeline: {:?}", e)
                }
            }
            let fetch_timeline =  timeline_service.get_timeline_by_username(author).await;

            match fetch_timeline {
                Ok(timeline) => println!("Timeline fetched: {:?}", timeline),
                Err(e) => println!("Error fetching timeline: {:?}", e)
            }

        } else {
            println!("Error creating tweet: ");
        }
    }
}
