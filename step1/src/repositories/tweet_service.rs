use std::sync::Arc;

use scylla::Session;

use crate::models::tweet::Tweet;

pub trait TweetServiceTrait {
    async fn create_tweet(&self, tweet: &Tweet) -> anyhow::Result<()>;
}

pub struct TweetService {
    pub connection: Arc<Session>,
}

impl TweetServiceTrait for TweetService {
    async fn create_tweet(&self, tweet: &Tweet) -> anyhow::Result<()> {
        let tweet_insert_query = format!(
            "INSERT INTO tweets (tweet_id, author, text, created_at) VALUES ({}, '{}', '{}', {})",
            tweet.tweet_id,
            tweet.author,
            tweet.text,
            tweet.created_at
        );

        self.connection.query(tweet_insert_query, &[]).await?;

        Ok(())
    }
}