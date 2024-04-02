use std::str::FromStr;
use std::sync::Arc;

use charybdis::QueryError;
use scylla::frame::value::CqlTimeuuid;
use scylla::Session;
use uuid::Uuid;

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
            created_at: Uuid::now_v1(&[1, 2, 3, 4, 5, 6]),
        };

        let tweet_insert_query = self.connection.prepare(
            "INSERT INTO mykeyspace.tweets (tweet_id, author, text, created_at) VALUES (?, ?, ?, ?)",
        ).await?;

        let payload = (
            tweet.tweet_id.clone(),
            tweet.author.clone(),
            tweet.text.clone(),
            CqlTimeuuid::from_str(tweet.created_at.to_string().as_str()).unwrap()
        );

        println!("{:?}", tweet_insert_query);

        let session = self.connection.execute(&tweet_insert_query, payload).await;

        match session {
            Ok(_) => Ok(tweet),
            Err(e) => {
                println!("Error inserting tweet: {:?}", e);
                Err(e)
            }
        }
    }
}