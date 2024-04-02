use std::str::FromStr;
use std::sync::Arc;

use charybdis::QueryError;
use scylla::frame::value::CqlTimeuuid;
use scylla::Session;
use uuid::Uuid;

use crate::models::timeline::Timeline;
use crate::models::tweet::Tweet;

pub trait TimelineServiceTrait {
    async fn insert_to_timeline(&self, username: &str, tweet: &Tweet) -> Result<(), QueryError>;

    async fn get_timeline_by_username(&self, username: &str) -> Result<(), QueryError>;
}

pub struct TimelineService {
    pub connection: Arc<Session>,
}

impl TimelineServiceTrait for TimelineService {
    async fn insert_to_timeline(&self, username: &str, tweet: &Tweet) -> Result<(), QueryError> {
        let random_liked = rand::random::<bool>();
        let random_bookmarked = rand::random::<bool>();
        let random_retweeted = rand::random::<bool>();

        let timeline = Timeline {
            username: username.to_string(),
            tweet_id: tweet.tweet_id.clone(),
            author: tweet.author.clone(),
            text: tweet.author.clone(),
            liked: random_liked,
            bookmarked: random_bookmarked,
            retweeted: random_retweeted,
            created_at: Uuid::now_v1(&[1, 2, 3, 4, 5, 6]),
        };

        let timeline_insert_query = self.connection.prepare(
            "INSERT INTO mykeyspace.timeline (username, tweet_id, author, text, liked, bookmarked, retweeted, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        ).await?;

        let payload = (
            timeline.username,
            timeline.tweet_id,
            timeline.author,
            timeline.text,
            timeline.liked,
            timeline.bookmarked,
            timeline.retweeted,
            CqlTimeuuid::from_str(tweet.created_at.to_string().as_str()).unwrap()
        );

        println!("{:?}", timeline_insert_query);

        let session = self.connection.execute(&timeline_insert_query, payload).await;

        match session {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn get_timeline_by_username(&self, username: &str) -> Result<(), QueryError> {
        let timeline_select_query = self.connection.prepare(
            "SELECT username, tweet_id, author, text, liked, bookmarked, retweeted, created_at FROM mykeyspace.timeline WHERE username = ?",
        ).await?;

        let timeline = self.connection.execute(&timeline_select_query, (username, )).await;

        match timeline {
            Ok(_res) => Ok(()),
            Err(e) => Err(e),
        }
    }
}