use std::sync::Arc;

use charybdis::QueryError;
use charybdis::types::Timeuuid;
use scylla::Session;

use crate::models::timeline::Timeline;
use crate::models::tweet::Tweet;

pub trait TimelineServiceTrait {
    async fn insert_to_timeline(&self, tweet: &Tweet) -> Result<(), QueryError>;

    async fn get_timeline_by_username(&self, username: &str) -> Result<(), QueryError>;

    async fn get_liked_timeline_by_username(&self, username: &str) -> Result<(), QueryError>;
}

pub struct TimelineService {
    pub connection: Arc<Session>,
}

impl TimelineServiceTrait for TimelineService {
    async fn insert_to_timeline(&self, tweet: &Tweet) -> Result<(), QueryError> {
        let timeline_insert_query = self.connection.prepare(
            "INSERT INTO timeline (username, tweet_id, author, text, liked, bookmarked, retweeted, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        ).await?;

        let random_liked = rand::random::<bool>();
        let random_bookmarked = rand::random::<bool>();
        let random_retweeted = rand::random::<bool>();

        let timeline = Timeline {
            username: "danielhe4rt".to_owned(),
            tweet_id: tweet.tweet_id.clone(),
            author: tweet.author.clone(),
            text: tweet.author.clone(),
            liked: random_liked,
            bookmarked: random_bookmarked,
            retweeted: random_retweeted,
            created_at: Timeuuid::now_v1(&[1, 2, 3, 4, 5, 6])
        };

        let payload = (
            timeline.username,
            timeline.tweet_id,
            timeline.author,
            timeline.text,
            timeline.liked,
            timeline.bookmarked,
            timeline.retweeted,
            tweet.created_at
        );

        self.connection.execute(&timeline_insert_query, payload).await?;

        Ok(())
    }

    async fn get_timeline_by_username(&self, username: &str) -> Result<(), QueryError> {
        let timeline_select_query = self.connection.prepare(
            "SELECT username, tweet_id, author, text, liked, bookmarked, retweeted, created_at FROM timeline WHERE username = ? LIMIT 50",
        ).await?;

        let timeline = self.connection.execute(&timeline_select_query, (username, )).await;

        match timeline {
            Ok(_res) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn get_liked_timeline_by_username(&self, username: &str) -> Result<(), QueryError> {
        let timeline_select_query = self.connection.prepare(
            "SELECT \
                        username, tweet_id, author, text, liked, bookmarked, retweeted, created_at \
                   FROM \
                        timeline_liked \
                   WHERE \
                        username = ? AND liked = ? LIMIT 50",
        ).await?;

        let timeline = self.connection.execute(&timeline_select_query, (username, true)).await;

        match timeline {
            Ok(_res) => Ok(()),
            Err(e) => Err(e),
        }
    }
}