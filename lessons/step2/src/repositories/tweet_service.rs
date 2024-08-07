use std::sync::Arc;

use scylla::prepared_statement::PreparedStatement;
use scylla::Session;

use crate::models::tweet::Tweet;

const INSERT_TWEET_QUERY: &str = "INSERT INTO tweets (tweet_id, author, text, created_at) VALUES (?, ?, ?, ?)";

pub trait TweetServiceTrait {
    async fn create_tweet(&self, tweet: &Tweet) -> anyhow::Result<()>;
}

pub struct TweetService {
    connection: Arc<Session>,
    insert_tweet_query: PreparedStatement,
}

impl TweetService {
    pub async fn new(connection: Arc<Session>) -> Self {
        let insert_tweet_query = connection
            .prepare(INSERT_TWEET_QUERY)
            .await
            .unwrap();
        Self {
            connection,
            insert_tweet_query,
        }
    }
}

impl TweetServiceTrait for TweetService {
    async fn create_tweet(&self, tweet: &Tweet) -> anyhow::Result<()> {
        let payload = (
            &tweet.tweet_id,
            &tweet.author,
            &tweet.text,
            &tweet.created_at,
        );

        self.connection.execute(&self.insert_tweet_query, payload).await?;

        Ok(())
    }
}