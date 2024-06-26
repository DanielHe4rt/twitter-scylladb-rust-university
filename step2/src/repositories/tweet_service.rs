use std::str::FromStr;
use std::sync::Arc;

use charybdis::types::Timeuuid;
use scylla::frame::value::CqlTimeuuid;
use scylla::prepared_statement::PreparedStatement;
use scylla::Session;
use scylla::transport::errors::QueryError;
use uuid::Uuid;

use crate::models::tweet::Tweet;

const INSERT_TWEET_QUERY: &str = "INSERT INTO tweets (tweet_id, author, text, created_at) VALUES (?, ?, ?, ?)";

pub trait TweetServiceTrait {
    async fn create_tweet(&self, author: &str, text: String) -> Result<Tweet, QueryError>;
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
    async fn create_tweet(&self, author: &str, text: String) -> Result<Tweet, QueryError> {
        let tweet = Tweet {
            tweet_id: Uuid::new_v4(),
            author: author.to_string(),
            text,
            created_at: Timeuuid::now_v1(&[1, 2, 3, 4, 5, 6]),
        };


        let payload = (
            tweet.tweet_id.clone(),
            tweet.author.clone(),
            tweet.text.clone(),
            CqlTimeuuid::from_str(tweet.created_at.to_string().as_str()).unwrap()
        );

        let session = self.connection.execute(&self.insert_tweet_query, payload).await;

        match session {
            Ok(_) => Ok(tweet),
            Err(e) => {
                println!("Error inserting tweet: {:?}", e);
                Err(e)
            }
        }
    }
}