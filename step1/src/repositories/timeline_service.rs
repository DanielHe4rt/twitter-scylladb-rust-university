use std::sync::Arc;

use charybdis::QueryError;
use scylla::{IntoTypedRows, Session};

use crate::models::timeline::Timeline;
use crate::models::tweet::Tweet;

pub trait TimelineServiceTrait {
    async fn insert_to_timeline(&self, username: String, tweet: &Tweet) -> Result<Timeline, QueryError>;

    async fn get_timeline_by_username(&self, username: String) -> Result<(), QueryError>;
}

pub struct TimelineService {
    pub connection: Arc<Session>,
}

impl TimelineServiceTrait for TimelineService {
    async fn insert_to_timeline(&self, username: String, tweet: &Tweet) -> Result<Timeline, QueryError> {
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
            created_at: Default::default(),
        };

        let timeline_insert_query = format!(
            "INSERT INTO mykeyspace.timeline (username, tweet_id, author, text, liked, bookmarked, retweeted, created_at) VALUES ('{}', {}, '{}', '{}', {}, {}, {}, now())",
            timeline.username,
            timeline.tweet_id,
            timeline.author,
            timeline.text,
            timeline.liked,
            timeline.bookmarked,
            timeline.retweeted
        );
        println!("{}", timeline_insert_query);

        let session = self.connection.query(timeline_insert_query, &[]).await;

        match session {
            Ok(_) => Ok(timeline),
            Err(e) => Err(e),
        }
    }

    async fn get_timeline_by_username(&self, username: String) -> Result<(), QueryError> {
        let timeline_select_query = format!(
            "SELECT username, tweet_id, author, text, liked, bookmarked, retweeted, created_at FROM mykeyspace.timeline WHERE username = '{}'",
            username
        );

        let timeline = self.connection.query(timeline_select_query, &[]).await;

        match timeline {
            Ok(res) => Ok(()),
            Err(e) => Err(e),
        }
    }
}