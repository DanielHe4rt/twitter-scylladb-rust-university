use std::io;
use std::sync::Arc;
use charybdis::QueryError;

use scylla::Session;

use crate::models::tweet::Tweet;

pub trait TweetServiceTrait {
    async fn create_tweet(&self, author: String, text: String) -> Result<Tweet, QueryError>;
}

pub struct TweetService {
    pub connection: Arc<Session>,
}

impl TweetServiceTrait for TweetService {
    async fn create_tweet(&self, author: String, text: String) -> Result<Tweet, QueryError> {

        let tweet = Tweet {
            tweet_id: uuid::Uuid::new_v4(),
            author,
            text,
            time: None,
            created_at: chrono::Utc::now()
        };

        let tweet_insert_query = format!(
            "INSERT INTO mykeyspace.tweets (tweet_id, author, text, time, created_at) VALUES ({}, '{}', '{}', now(), {}) LIMIT 50",
            tweet.tweet_id,
            tweet.author,
            tweet.text,
            tweet.created_at.timestamp()
        );

        println!("{}", tweet_insert_query);

        let session = self.connection.query(tweet_insert_query, &[]).await;

        match session {
            Ok(_) => Ok(tweet),
            Err(e) => Err(e),
        }
    }
}