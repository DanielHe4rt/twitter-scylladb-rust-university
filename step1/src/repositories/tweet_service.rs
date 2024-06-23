use std::sync::Arc;

use charybdis::QueryError;
use charybdis::types::Timeuuid;
use scylla::Session;
use uuid::Uuid;

use crate::models::tweet::Tweet;

pub trait TweetServiceTrait {
    async fn create_tweet(&self, author: &str, text: String) -> Result<Tweet, QueryError>;
}

pub struct TweetService {
    pub connection: Arc<Session>,
}

impl TweetServiceTrait for TweetService {
    async fn create_tweet(&self, author: &str, text: String) -> Result<Tweet, QueryError> {

        let tweet = Tweet {
            tweet_id: Uuid::new_v4(),
            author: author.to_string(),
            text,
            created_at: Timeuuid::now_v1(&[1,2,3,4,5,6])
        };

        let tweet_insert_query = format!(
            "INSERT INTO tweets (tweet_id, author, text, created_at) VALUES ({}, '{}', '{}', {})",
            tweet.tweet_id,
            tweet.author,
            tweet.text,
            tweet.created_at
        );

        println!("{}", tweet_insert_query);

        let session = self.connection.query(tweet_insert_query, &[]).await;

        match session {
            Ok(_) => Ok(tweet),
            Err(e) => {
                println!("Error inserting tweet: {:?}", e);
                Err(e)
            },
        }
    }
}