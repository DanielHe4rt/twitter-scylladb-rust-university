use std::sync::Arc;

use anyhow::Result;
use scylla::Session;

use crate::models::timeline::Timeline;

pub trait TimelineServiceTrait {
    async fn insert_to_timeline(&self, timeline: &Timeline) -> Result<()>;

    async fn get_timeline_by_username(&self, username: &str) -> Result<()>;
}

pub struct TimelineService {
    pub connection: Arc<Session>,
}

impl TimelineServiceTrait for TimelineService {
    async fn insert_to_timeline(&self, timeline: &Timeline) -> anyhow::Result<()> {
        let timeline_insert_query = format!(
            "INSERT INTO timeline (username, tweet_id, author, text, liked, bookmarked, retweeted, created_at) VALUES ('{}', {}, '{}', '{}', {}, {}, {}, {})",
            timeline.username,
            timeline.tweet_id,
            timeline.author,
            timeline.text,
            timeline.liked,
            timeline.bookmarked,
            timeline.retweeted,
            timeline.created_at,
        );

        self.connection.query(timeline_insert_query, &[]).await?;

        Ok(())
    }

    async fn get_timeline_by_username(&self, username: &str) -> anyhow::Result<()> {

        let timeline_select_query = format!(
            "SELECT username, tweet_id, author, text, liked, bookmarked, retweeted, created_at FROM timeline WHERE username = '{}' LIMIT 50",
            username
        );

        self.connection.query(timeline_select_query, &[]).await?;

        Ok(())
    }
}